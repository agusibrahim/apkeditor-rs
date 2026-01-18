//! rsapkeditor WASM Library
//! 
//! This module provides APK editing and signing capabilities for WebAssembly.

pub mod manifest;
pub mod sign;

use std::io::{Cursor, Read, Write};
use anyhow::Result;
use zip::{write::ExtendedFileOptions, ZipArchive, ZipWriter};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use js_sys::Uint8Array;

/// Result of APK editing operation
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct ApkEditResult {
    data: Vec<u8>,
    success: bool,
    error_message: String,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ApkEditResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn error_message(&self) -> String {
        self.error_message.clone()
    }

    #[wasm_bindgen]
    pub fn get_data(&self) -> Uint8Array {
        Uint8Array::from(&self.data[..])
    }
}

/// Validate package name
pub fn validate_pkgname(pkgname: &str) -> Result<String, String> {
    let chars = pkgname.chars();
    let mut repeated = false;
    for char in chars {
        if char == '.' {
            if repeated {
                return Err(
                    "Input package name contains more than one separator per complement"
                        .to_string(),
                );
            }
            repeated = true;
        } else {
            repeated = false;
        }
    }
    let components = pkgname.split(".");
    for component in components {
        if component.chars().nth(0).is_some_and(|c| c.is_ascii_digit()) {
            return Err("First character after a '.' should never be a number".to_string());
        }
        for char in component.chars() {
            if char != '_' && !char.is_ascii_alphanumeric() {
                return Err(
                    "Package name can only contain alphanumerical characters or '_'".to_string(),
                );
            }
        }
    }
    Ok(pkgname.to_ascii_lowercase())
}

/// Check if file is v1 signature
fn is_v1sign(filename: &str) -> bool {
    filename.starts_with("META-INF/") && (filename.ends_with(".SF") || filename.ends_with("RSA"))
}

/// Edit APK from bytes - the main function for WASM
pub fn edit_apk_bytes(
    apk_data: &[u8],
    package_name: Option<&str>,
    app_name: Option<&str>,
    version_code: Option<u32>,
    version_name: Option<&str>,
) -> Result<Vec<u8>> {
    // Validate package name if provided
    if let Some(pkg) = package_name {
        validate_pkgname(pkg).map_err(|e| anyhow::anyhow!(e))?;
    }

    let cursor = Cursor::new(apk_data);
    let mut input_apk = ZipArchive::new(cursor)?;
    
    let output = Vec::new();
    let mut output_apk = ZipWriter::new(Cursor::new(output));
    
    let needs_manifest_edit = package_name.is_some() || app_name.is_some() 
        || version_code.is_some() || version_name.is_some();
    
    // Process all files - remove v1 signatures and edit manifest if needed
    for i in 0..input_apk.len() {
        let mut file = input_apk.by_index(i)?;
        
        // Skip v1 signatures
        if is_v1sign(file.name()) {
            continue;
        }
        
        // Handle resources.arsc with alignment
        if file.name() == "resources.arsc" {
            let options = zip::write::FileOptions::<ExtendedFileOptions>::default()
                .compression_method(zip::CompressionMethod::Stored)
                .with_alignment(4);
            output_apk.start_file(file.name(), options)?;
            std::io::copy(&mut file, &mut output_apk)?;
            continue;
        }

        // Handle native libraries (.so) - must be uncompressed and aligned
        if file.name().ends_with(".so") {
            let options = zip::write::FileOptions::<ExtendedFileOptions>::default()
                .compression_method(zip::CompressionMethod::Stored)
                .with_alignment(4);
            output_apk.start_file(file.name(), options)?;
            std::io::copy(&mut file, &mut output_apk)?;
            continue;
        }
        
        // Edit AndroidManifest.xml if any edit options are provided
        if file.name() == "AndroidManifest.xml" && needs_manifest_edit {
            let mut file_data = Vec::with_capacity(file.size().try_into()?);
            file.read_to_end(&mut file_data)?;
            
            let edit_options = manifest::ManifestEditOptions {
                app_name,
                package_name,
                version_code,
                version_name,
            };
            
            let edited = manifest::edit_manifest(&file_data, &edit_options)?;
            output_apk.start_file(
                file.name(),
                zip::write::FileOptions::<ExtendedFileOptions>::default(),
            )?;
            output_apk.write_all(&edited)?;
            continue;
        }
        
        // Copy other files as-is
        output_apk.raw_copy_file(file)?;
    }
    
    let output_cursor = output_apk.finish()?;
    let unsigned_apk = output_cursor.into_inner();
    
    // Sign the APK with debug keystore
    let signed_apk = sign::sign_apk_bytes(&unsigned_apk)?;
    Ok(signed_apk)
}

// ============ WASM Bindings ============

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Edit and sign an APK file from JavaScript
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn edit_apk(
    apk_data: &[u8],
    package_name: Option<String>,
    app_name: Option<String>,
    version_code: Option<u32>,
    version_name: Option<String>,
) -> ApkEditResult {
    match edit_apk_bytes(
        apk_data, 
        package_name.as_deref(), 
        app_name.as_deref(),
        version_code,
        version_name.as_deref(),
    ) {
        Ok(data) => ApkEditResult {
            data,
            success: true,
            error_message: String::new(),
        },
        Err(e) => ApkEditResult {
            data: Vec::new(),
            success: false,
            error_message: e.to_string(),
        },
    }
}

/// Validate a package name
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_package_name(name: &str) -> bool {
    validate_pkgname(name).is_ok()
}

/// Get version info
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// APK information extracted from manifest
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct ApkInfo {
    package_name: String,
    app_name: String,
    version_code: u32,
    version_name: String,
    success: bool,
    error_message: String,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ApkInfo {
    #[wasm_bindgen(getter)]
    pub fn package_name(&self) -> String {
        self.package_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn version_code(&self) -> u32 {
        self.version_code
    }

    #[wasm_bindgen(getter)]
    pub fn version_name(&self) -> String {
        self.version_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn error_message(&self) -> String {
        self.error_message.clone()
    }
}

/// Extract APK info (package name, app name, version) from APK bytes
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_apk_info(apk_data: &[u8]) -> ApkInfo {
    match extract_apk_info(apk_data) {
        Ok(info) => ApkInfo {
            package_name: info.0,
            app_name: info.1,
            version_code: info.2,
            version_name: info.3,
            success: true,
            error_message: String::new(),
        },
        Err(e) => ApkInfo {
            package_name: String::new(),
            app_name: String::new(),
            version_code: 0,
            version_name: String::new(),
            success: false,
            error_message: e.to_string(),
        },
    }
}

/// Extract package name, app name, version code, version name from APK
fn extract_apk_info(apk_data: &[u8]) -> Result<(String, String, u32, String)> {
    use apk::res::{Chunk, ResValueType};
    
    let cursor = Cursor::new(apk_data);
    let mut archive = ZipArchive::new(cursor)?;
    
    let mut manifest_file = archive.by_name("AndroidManifest.xml")?;
    let mut manifest_data = Vec::new();
    manifest_file.read_to_end(&mut manifest_data)?;
    
    let mut cursor = Cursor::new(&manifest_data);
    let Chunk::Xml(xchunks) = Chunk::parse(&mut cursor)? else {
        anyhow::bail!("Invalid manifest format");
    };
    
    let (string_pool, chunks) = xchunks.split_first().unwrap();
    let Chunk::StringPool(strings, _) = string_pool else {
        anyhow::bail!("Invalid string pool");
    };
    
    let mut package_name = String::new();
    let mut app_name = String::new();
    let mut version_code: u32 = 0;
    let mut version_name = String::new();
    
    for chunk in chunks {
        if let Chunk::XmlStartElement(_, el, attrs) = chunk {
            let el_name = strings.get(el.name as usize).map(|s| s.as_str()).unwrap_or("");
            
            // Get package name and version from manifest element
            if el_name == "manifest" {
                for attr in attrs {
                    let attr_name = strings.get(attr.name as usize).map(|s| s.as_str()).unwrap_or("");
                    
                    if attr_name == "package" {
                        if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                            if let Some(s) = strings.get(attr.typed_value.data as usize) {
                                package_name = s.clone();
                            }
                        }
                    }
                    
                    if attr_name == "versionCode" {
                        // versionCode is typically an integer
                        version_code = attr.typed_value.data;
                    }
                    
                    if attr_name == "versionName" {
                        if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                            if let Some(s) = strings.get(attr.typed_value.data as usize) {
                                version_name = s.clone();
                            }
                        }
                    }
                }
            }
            
            // Get app name from application element
            if el_name == "application" && app_name.is_empty() {
                for attr in attrs {
                    let attr_name = strings.get(attr.name as usize).map(|s| s.as_str()).unwrap_or("");
                    if attr_name == "label" {
                        if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                            if let Some(s) = strings.get(attr.typed_value.data as usize) {
                                app_name = s.clone();
                            }
                        } else {
                            // Label might be a resource reference, show placeholder
                            app_name = "(resource reference)".to_string();
                        }
                    }
                }
            }
        }
    }
    
    Ok((package_name, app_name, version_code, version_name))
}

/// Extract icon from APK - returns PNG bytes of the highest resolution icon
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_apk_icon(apk_data: &[u8]) -> Uint8Array {
    match extract_icon(apk_data) {
        Ok(data) => Uint8Array::from(&data[..]),
        Err(_) => Uint8Array::new_with_length(0),
    }
}

/// Debug: List all files in APK (returns newline-separated list)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn list_apk_files(apk_data: &[u8]) -> String {
    let cursor = Cursor::new(apk_data);
    let Ok(mut archive) = ZipArchive::new(cursor) else {
        return "Error: Could not open APK".to_string();
    };
    
    let mut files = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            files.push(file.name().to_string());
        }
    }
    files.join("\n")
}

/// Debug: Dump manifest structure
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn dump_manifest(apk_data: &[u8]) -> String {
    use apk::res::{Chunk, ResValueType};
    
    let cursor = Cursor::new(apk_data);
    let Ok(mut archive) = ZipArchive::new(cursor) else {
        return "Error: Could not open APK".to_string();
    };
    
    let Ok(mut manifest_file) = archive.by_name("AndroidManifest.xml") else {
        return "Error: No AndroidManifest.xml found".to_string();
    };
    
    let mut manifest_data = Vec::new();
    if manifest_file.read_to_end(&mut manifest_data).is_err() {
        return "Error: Could not read manifest".to_string();
    }
    drop(manifest_file);
    
    let mut cursor = Cursor::new(&manifest_data);
    let Ok(Chunk::Xml(xchunks)) = Chunk::parse(&mut cursor) else {
        return "Error: Could not parse manifest".to_string();
    };
    
    let mut output = String::new();
    
    // Get string pool
    let (string_pool, chunks) = xchunks.split_first().unwrap();
    let Chunk::StringPool(strings, _) = string_pool else {
        return "Error: No string pool".to_string();
    };
    
    output.push_str("=== STRING POOL ===\n");
    for (i, s) in strings.iter().enumerate() {
        output.push_str(&format!("[{}] {}\n", i, s));
    }
    
    output.push_str("\n=== ELEMENTS ===\n");
    for chunk in chunks {
        if let Chunk::XmlStartElement(_, el, attrs) = chunk {
            let el_name = strings.get(el.name as usize).map(|s| s.as_str()).unwrap_or("?");
            output.push_str(&format!("<{}>\n", el_name));
            
            for attr in attrs {
                let attr_name = strings.get(attr.name as usize).map(|s| s.as_str()).unwrap_or("?");
                let type_name = match ResValueType::from_u8(attr.typed_value.data_type) {
                    Some(ResValueType::String) => "string",
                    Some(ResValueType::IntDec) => "int",
                    Some(ResValueType::IntHex) => "hex",
                    Some(ResValueType::Reference) => "ref",
                    Some(ResValueType::IntBoolean) => "bool",
                    _ => "other",
                };
                let value = if let Some(ResValueType::String) = ResValueType::from_u8(attr.typed_value.data_type) {
                    strings.get(attr.typed_value.data as usize).map(|s| s.as_str()).unwrap_or("?").to_string()
                } else {
                    format!("0x{:08x}", attr.typed_value.data)
                };
                output.push_str(&format!("  {} ({}) = {}\n", attr_name, type_name, value));
            }
        }
    }
    
    output
}

/// Extract the best available icon from APK
fn extract_icon(apk_data: &[u8]) -> Result<Vec<u8>> {
    let cursor = Cursor::new(apk_data);
    let mut archive = ZipArchive::new(cursor)?;
    
    // Collect all PNG files with metadata
    let mut named_icons: Vec<(usize, i32, u64)> = Vec::new();  // (index, score, size)
    let mut fallback_pngs: Vec<(usize, u64)> = Vec::new();     // (index, size)
    
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        let name_lower = name.to_lowercase();
        let size = file.size();
        
        // Skip non-PNG files
        if !name_lower.ends_with(".png") {
            continue;
        }
        
        // Skip adaptive icon parts and 9-patch
        if name_lower.contains("_foreground") || name_lower.contains("_background") 
            || name_lower.contains("_round") || name_lower.contains(".9.png") {
            continue;
        }
        
        // Check if it has launcher in name (non-obfuscated)
        if name_lower.contains("launcher") {
            let score = if name_lower.contains("xhdpi") { 100 }
                else if name_lower.contains("hdpi") && !name_lower.contains("xxhdpi") { 90 }
                else if name_lower.contains("xxhdpi") { 80 }
                else if name_lower.contains("mdpi") { 70 }
                else { 50 };
            let score = if name_lower.contains("mipmap") { score + 10 } else { score };
            named_icons.push((i, score, size));
        } else if name_lower.starts_with("res/") && size >= 1000 && size <= 50000 {
            // Fallback: PNG files in res/ folder with icon-like size (1KB-50KB)
            fallback_pngs.push((i, size));
        }
    }
    
    // Try named icons first
    if !named_icons.is_empty() {
        named_icons.sort_by(|a, b| b.1.cmp(&a.1));
        for (idx, _, _) in named_icons {
            if let Ok(mut file) = archive.by_index(idx) {
                let mut data = Vec::new();
                if file.read_to_end(&mut data).is_ok() && !data.is_empty() {
                    return Ok(data);
                }
            }
        }
    }
    
    // Fallback: sort by size descending, prefer medium-sized PNGs (5KB-20KB typical for icons)
    if !fallback_pngs.is_empty() {
        fallback_pngs.sort_by(|a, b| {
            // Prefer sizes between 5KB-20KB (typical icon size)
            let ideal_a = if a.1 >= 5000 && a.1 <= 20000 { 1000 } else { 0 };
            let ideal_b = if b.1 >= 5000 && b.1 <= 20000 { 1000 } else { 0 };
            (ideal_b + b.1 as i64).cmp(&(ideal_a + a.1 as i64))
        });
        
        for (idx, _) in fallback_pngs {
            if let Ok(mut file) = archive.by_index(idx) {
                let mut data = Vec::new();
                if file.read_to_end(&mut data).is_ok() && !data.is_empty() {
                    return Ok(data);
                }
            }
        }
    }
    
    anyhow::bail!("No icon found in APK")
}
