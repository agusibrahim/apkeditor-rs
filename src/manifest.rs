//! AndroidManifest.xml editing module

use std::collections::HashSet;
use std::io::Cursor;

use anyhow::{Context, Result};
use apk::res::{Chunk, ResValue, ResValueType, ResXmlAttribute};

/// Edit options for the manifest
#[derive(Default)]
pub struct ManifestEditOptions<'a> {
    pub app_name: Option<&'a str>,
    pub package_name: Option<&'a str>,
    pub version_code: Option<u32>,
    pub version_name: Option<&'a str>,
}

/// Edit the binary AndroidManifest.xml
pub fn edit_manifest(manifest: &[u8], options: &ManifestEditOptions) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(manifest);
    let Chunk::Xml(mut xchunks) = Chunk::parse(&mut cursor)? else {
        anyhow::bail!("invalid manifest 0");
    };
    let (string_pool, chunks) = xchunks.split_first_mut().unwrap();
    let Chunk::StringPool(strings, _) = string_pool else {
        anyhow::bail!("Annoying....");
    };

    // Change package name
    if let Some(pkgname) = options.package_name {
        let old_pkgname =
            edit_attr_in_element(chunks, "manifest", "package", pkgname.to_owned(), strings)?
                .with_context(|| "There is no package name in manifest.")?;

        // First, handle the case where android:authorities shares the same string pool entry
        // as android:name. In this case, we need to duplicate the string for authorities
        // so we can replace it without affecting the class name.
        duplicate_shared_authority_strings(chunks, strings, &old_pkgname);

        // Collect string indices used in android:name attributes (class references)
        // These should NOT be replaced as they reference actual Java/Kotlin classes
        let class_name_indices = collect_class_name_indices(chunks, strings);

        // Replace package name in string pool.
        //
        // Strategy: Replace ALL occurrences of the old package name EXCEPT
        // those used in android:name attributes (class references).
        //
        // This replaces:
        // - Provider authorities: "com.example.app.SomeProvider"
        // - Permissions: "com.example.app.PERMISSION_NAME"
        // - File providers: "com.example.app.fileprovider"
        //
        // This preserves:
        // - Class references in android:name: "com.example.app.MainActivity"
        for (idx, string) in strings.iter_mut().enumerate() {
            // Skip if doesn't contain old package name
            if !string.contains(&old_pkgname) {
                continue;
            }

            // Skip if this index is used for android:name (class reference)
            if class_name_indices.contains(&idx) {
                continue;
            }

            // Replace all occurrences of old package name
            *string = string.replace(&old_pkgname, pkgname);
        }
    }
    
    // Change version code (integer)
    if let Some(version_code) = options.version_code {
        edit_attr_int_in_element(chunks, "manifest", "versionCode", version_code, strings)?;
    }
    
    // Change version name (string)
    if let Some(version_name) = options.version_name {
        let _ = try_edit_attr_in_element(chunks, "manifest", "versionName", version_name.to_owned(), strings);
    }
    
    // Change app name
    if let Some(app_name) = options.app_name {
        // Edit application label - this should exist
        edit_attr_in_element(chunks, "application", "label", app_name.to_owned(), strings)?;
        // Edit activity label - optional, some APKs don't have this
        let _ = try_edit_attr_in_element(chunks, "activity", "label", app_name.to_owned(), strings);
    }
    
    // Return modified manifest
    let mut mod_manifest = Vec::new();
    Chunk::Xml(xchunks).write(&mut Cursor::new(&mut mod_manifest))?;
    Ok(mod_manifest)
}

fn edit_attr_in_element(
    elements: &mut [Chunk],
    el_name: &str,
    attr_name: &str,
    new_str: String,
    pool: &mut Vec<String>,
) -> Result<Option<String>> {
    let attrs = elements
        .iter_mut()
        .find_map(|e| parse_element(&mut *e, el_name, pool))
        .with_context(|| format!("Xml element is missing: {el_name}"))?;
    let attr = attrs
        .iter_mut()
        .find(|a| attr_has_name(a.name, attr_name, pool))
        .with_context(|| format!("Attribute {attr_name} not found in element {el_name}"))?;

    edit_attr_string(attr, new_str, pool)
}

/// Edit an integer attribute in an element
fn edit_attr_int_in_element(
    elements: &mut [Chunk],
    el_name: &str,
    attr_name: &str,
    new_value: u32,
    pool: &[String],
) -> Result<()> {
    let attrs = elements
        .iter_mut()
        .find_map(|e| parse_element(&mut *e, el_name, pool))
        .with_context(|| format!("Xml element is missing: {el_name}"))?;
    let attr = attrs
        .iter_mut()
        .find(|a| attr_has_name(a.name, attr_name, pool))
        .with_context(|| format!("Attribute {attr_name} not found in element {el_name}"))?;

    // Set as integer type
    attr.typed_value.data_type = ResValueType::IntDec as u8;
    attr.typed_value.data = new_value;
    attr.raw_value = -1; // No raw string value for integers
    
    Ok(())
}

/// Try to edit an attribute - returns Ok(None) if element or attribute not found
fn try_edit_attr_in_element(
    elements: &mut [Chunk],
    el_name: &str,
    attr_name: &str,
    new_str: String,
    pool: &mut Vec<String>,
) -> Result<Option<String>> {
    let Some(attrs) = elements
        .iter_mut()
        .find_map(|e| parse_element(&mut *e, el_name, pool))
    else {
        return Ok(None); // Element not found, that's OK
    };
    
    let Some(attr) = attrs
        .iter_mut()
        .find(|a| attr_has_name(a.name, attr_name, pool))
    else {
        return Ok(None); // Attribute not found, that's OK
    };

    edit_attr_string(attr, new_str, pool)
}

fn edit_attr_string(
    attr: &mut ResXmlAttribute,
    name: String,
    pool: &mut Vec<String>,
) -> Result<Option<String>> {
    let value = &mut attr.typed_value;
    let attr_type = ResValueType::from_u8(value.data_type)
        .with_context(|| format!("Type of label value is unknown: {}", value.data_type))?;
    match attr_type {
        ResValueType::String => Ok(Some(std::mem::replace(
            &mut pool[value.data as usize],
            name,
        ))),
        // In this case we overwrite it so that its a direct string, rid solving is pain
        _ => {
            let new_rvalue = ResValue {
                size: 8,
                res0: 0,
                data_type: ResValueType::String as u8,
                data: pool.len() as u32,
            };
            *value = new_rvalue;
            attr.raw_value = pool.len() as i32;
            pool.push(name);
            Ok(None)
        }
    }
}

fn attr_has_name(index: i32, name: &str, string_pool: &[String]) -> bool {
    let index = match usize::try_from(index) {
        Ok(usize) => usize,
        Err(_) => return false,
    };
    string_pool.get(index).is_some_and(|s| s == name)
}

/// Duplicate strings that are shared between android:name and android:authorities.
/// This allows us to replace the authorities string without affecting the class name.
fn duplicate_shared_authority_strings(chunks: &mut [Chunk], pool: &mut Vec<String>, old_pkgname: &str) {
    // First, collect indices used in android:name for provider elements
    let mut name_indices: HashSet<usize> = HashSet::new();
    for chunk in chunks.iter() {
        if let Chunk::XmlStartElement(_, el, attrs) = chunk {
            let element_name = pool.get(el.name as usize).map(|s| s.as_str()).unwrap_or("");
            if element_name == "provider" {
                for attr in attrs {
                    if attr_has_name(attr.name, "name", pool) {
                        if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                            let idx = attr.typed_value.data as usize;
                            if let Some(s) = pool.get(idx) {
                                if s.contains(old_pkgname) {
                                    name_indices.insert(idx);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Now find authorities attributes that share the same index and need duplication
    for chunk in chunks.iter_mut() {
        if let Chunk::XmlStartElement(_, el, attrs) = chunk {
            let element_name = pool.get(el.name as usize).map(|s| s.as_str()).unwrap_or("");
            if element_name == "provider" {
                for attr in attrs.iter_mut() {
                    if attr_has_name(attr.name, "authorities", pool) {
                        if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                            let idx = attr.typed_value.data as usize;
                            // If this index is also used for android:name, duplicate it
                            if name_indices.contains(&idx) {
                                if let Some(s) = pool.get(idx) {
                                    // Create a new string pool entry with the same value
                                    let new_idx = pool.len() as u32;
                                    pool.push(s.clone());
                                    // Update the attribute to use the new index
                                    attr.typed_value.data = new_idx;
                                    if attr.raw_value >= 0 {
                                        attr.raw_value = new_idx as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Collect all string pool indices that are used in android:name attributes
/// for component declarations (application, activity, service, receiver, provider).
/// These are class references and should NOT be replaced during package name changes.
fn collect_class_name_indices(chunks: &[Chunk], pool: &[String]) -> HashSet<usize> {
    let mut indices = HashSet::new();

    // Component element names that have class references in android:name
    let component_elements = ["application", "activity", "activity-alias", "service", "receiver", "provider"];

    for chunk in chunks {
        if let Chunk::XmlStartElement(_, el, attrs) = chunk {
            // Check if this element is a component declaration
            let element_name = pool.get(el.name as usize).map(|s| s.as_str()).unwrap_or("");
            if !component_elements.contains(&element_name) {
                continue;
            }

            for attr in attrs {
                // Check if this attribute is "name"
                if attr_has_name(attr.name, "name", pool) {
                    // Get the string value index
                    let value = &attr.typed_value;
                    if let Some(ResValueType::String) = ResValueType::from_u8(value.data_type) {
                        let str_idx = value.data as usize;
                        // Only add if the string looks like an absolute class name
                        // (contains dots and doesn't start with a dot)
                        if let Some(s) = pool.get(str_idx) {
                            if s.contains('.') && !s.starts_with('.') {
                                indices.insert(str_idx);
                            }
                        }
                    }
                }
            }
        }
    }

    indices
}

fn parse_element<'c>(
    chunk: &'c mut Chunk,
    name: &str,
    string_pool: &[String],
) -> Option<&'c mut Vec<ResXmlAttribute>> {
    let Chunk::XmlStartElement(_, el, attrs) = chunk else {
        return None;
    };
    if string_pool.get(el.name as usize).is_some_and(|s| s == name) {
        return Some(attrs);
    }
    None
}

