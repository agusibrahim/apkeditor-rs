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

use apk_info_axml::{AXML, ARSC};

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
    custom_pem: Option<&str>,
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

        // Handle native libraries (.so) - must be uncompressed and page-aligned (4096)
        if file.name().ends_with(".so") {
            let options = zip::write::FileOptions::<ExtendedFileOptions>::default()
                .compression_method(zip::CompressionMethod::Stored)
                .with_alignment(4096);
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
    
    // Sign the APK
    let signed_apk = sign::sign_apk_bytes(&unsigned_apk, custom_pem)?;
    Ok(signed_apk)
}

// Helper to convert P12 to PEM string
fn convert_p12_to_pem(p12_data: &[u8], password: &str) -> Result<String> {
    use p12_keystore::KeyStore;
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
    use std::fmt::Write; // for write! macro

    let keystore = KeyStore::from_pkcs12(p12_data, password)
        .map_err(|e| anyhow::anyhow!("Failed to parse P12: {:?}", e))?;
    
    // Find first private key
    let (_, chain) = keystore.private_key_chain()
        .ok_or_else(|| anyhow::anyhow!("No private key found in keystore"))?;
    
    let mut pem = String::new();
    
    // Write certificate chain
    for cert in chain.certs() {
        writeln!(&mut pem, "-----BEGIN CERTIFICATE-----")?;
        // Base64 encode the DER bytes
        let b64 = BASE64.encode(cert.as_der());
        // Wrap lines at 64 chars
        for chunk in b64.as_bytes().chunks(64) {
             writeln!(&mut pem, "{}", std::str::from_utf8(chunk)?)?;
        }
        writeln!(&mut pem, "-----END CERTIFICATE-----")?;
    }
    
    // Write private key
    // chain.key is PKCS#8 DER bytes
    writeln!(&mut pem, "-----BEGIN PRIVATE KEY-----")?;
    let b64 = BASE64.encode(chain.key().as_der());
    for chunk in b64.as_bytes().chunks(64) {
         writeln!(&mut pem, "{}", std::str::from_utf8(chunk)?)?;
    }
    writeln!(&mut pem, "-----END PRIVATE KEY-----")?;
    
    Ok(pem)
}

// ============ WASM Bindings ============

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Edit and sign an APK file from JavaScript with default debug key
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
        None, // Default debug key
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

/// Edit and sign an APK file from JavaScript with CUSTOM P12 keystore
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn edit_apk_with_keystore(
    apk_data: &[u8],
    package_name: Option<String>,
    app_name: Option<String>,
    version_code: Option<u32>,
    version_name: Option<String>,
    p12_data: &[u8],
    p12_password: &str,
) -> ApkEditResult {
    // Convert P12 to PEM
    let pem_result = convert_p12_to_pem(p12_data, p12_password);
    
    if let Err(e) = pem_result {
         return ApkEditResult {
            data: Vec::new(),
            success: false,
            error_message: format!("Keystore Error: {}", e),
        };
    }
    let pem = pem_result.unwrap();

    match edit_apk_bytes(
        apk_data, 
        package_name.as_deref(), 
        app_name.as_deref(),
        version_code,
        version_name.as_deref(),
        Some(&pem),
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

/// Edit and sign an APK file from JavaScript with PEM string (no password)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn edit_apk_with_pem(
    apk_data: &[u8],
    package_name: Option<String>,
    app_name: Option<String>,
    version_code: Option<u32>,
    version_name: Option<String>,
    pem_string: &str,
) -> ApkEditResult {
    match edit_apk_bytes(
        apk_data, 
        package_name.as_deref(), 
        app_name.as_deref(),
        version_code,
        version_name.as_deref(),
        Some(pem_string),
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

/// Verify if the password is correct for the given P12 keystore data
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn verify_keystore_password(p12_data: &[u8], password: &str) -> bool {
    use p12_keystore::KeyStore;
    // Attempt to open the keystore. If it succeeds, the password is correct.
    KeyStore::from_pkcs12(p12_data, password).is_ok()
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

/// Extract package name, app name, version code, version name using apk-info-axml
fn extract_apk_info(apk_data: &[u8]) -> Result<(String, String, u32, String)> {
    let cursor = Cursor::new(apk_data);
    let mut archive = ZipArchive::new(cursor)?;

    // 1. Try to read resources.arsc
    let mut arsc: Option<ARSC> = None;
    if let Ok(mut file) = archive.by_name("resources.arsc") {
        let mut arsc_data = Vec::new();
        if file.read_to_end(&mut arsc_data).is_ok() {
            let mut input = arsc_data.as_slice();
            if let Ok(parsed) = ARSC::new(&mut input) {
                arsc = Some(parsed);
            }
        }
    }

    // 2. Read AndroidManifest.xml
    let mut manifest_file = archive.by_name("AndroidManifest.xml")?;
    let mut manifest_data = Vec::new();
    manifest_file.read_to_end(&mut manifest_data)?;
    drop(manifest_file); // release archive borrow

    let mut input = manifest_data.as_slice();
    let axml = AXML::new(&mut input, arsc.as_ref()).map_err(|e| anyhow::anyhow!("AXML Parse Error: {:?}", e))?;

    // 3. Extract Info
    let package_name = axml.get_attribute_value("manifest", "package", arsc.as_ref()).unwrap_or_default();
    
    // Version
    let version_code_str = axml.get_attribute_value("manifest", "versionCode", arsc.as_ref()).unwrap_or("0".to_string());
    let version_code = version_code_str.parse::<u32>().unwrap_or(0);
    
    let version_name = axml.get_attribute_value("manifest", "versionName", arsc.as_ref()).unwrap_or_default();

    // App Name (Label)
    let app_name = axml.get_attribute_value("application", "label", arsc.as_ref()).unwrap_or_default();

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

/// Extract the best available icon from APK using Manifest Parsing
fn extract_icon(apk_data: &[u8]) -> Result<Vec<u8>> {
    let cursor = Cursor::new(apk_data);
    let mut archive = ZipArchive::new(cursor)?;

    // 1. Try to read resources.arsc
    let mut arsc = None;
    let mut arsc_data = Vec::new();
    if let Ok(mut file) = archive.by_name("resources.arsc") {
        if file.read_to_end(&mut arsc_data).is_ok() {
            let mut input = arsc_data.as_slice();
            if let Ok(parsed) = ARSC::new(&mut input) {
                arsc = Some(parsed);
            }
        }
    }

    // 2. Read Manifest
    let mut manifest_file = archive.by_name("AndroidManifest.xml")?;
    let mut manifest_data = Vec::new();
    manifest_file.read_to_end(&mut manifest_data)?;
    drop(manifest_file);

    let mut input = manifest_data.as_slice();
    let axml = AXML::new(&mut input, arsc.as_ref()).map_err(|e| anyhow::anyhow!("AXML Parse Error: {:?}", e))?;

    // 3. Find Icon Path
    // Priority 1: Launcher Activity Icon through helper
    // The helper returns an iterator, we take the first one or check if it matches application?
    // Wait, get_main_activities return the *activity name*. We want the icon attribute of that activity.
    
    let main_activities: Vec<String> = axml.get_main_activities().map(|s| s.to_string()).collect();
    let mut icon_path = None;

    if let Some(main_activity) = main_activities.first() {
        // Find this activity in AXML manually to get its icon attribute?
        // Or simpler: iterate attributes of the activity tag?
        // AXML struct doesn't expose easy "find element by attribute value" from public API easily.
        // It exposes `get_attribute_value` but that searches by tag.
        
        // We can traverse the tree:
        // root -> children (application) -> children (activity)
        // This is getting complicated to navigate via public API if not exposed.
        // AXML public API has `root` field which is `Element`.
        
        if let Some(app) = axml.root.childrens().find(|c| c.name() == "application") {
            // Find the activity element matching key
             if let Some(act_node) = app.childrens().find(|c| c.attr("name") == Some(main_activity.as_str())) {
                if let Some(val) = act_node.attr("icon") {
                    // Start with @? Resolve.
                    if val.starts_with("@") {
                         if let Some(arsc_ref) = arsc.as_ref() {
                             // strip @ and get
                             icon_path = arsc_ref.get_resource_value_by_name(&val[1..]);
                         }
                    } else {
                        icon_path = Some(val.to_string());
                    }
                }
             }
        }
    }

    // Priority 2: Application Icon
    if icon_path.is_none() {
        icon_path = axml.get_attribute_value("application", "icon", arsc.as_ref());
    }

    // 4. Optimize Icon Resolution
    // ARSC lookup often returns the default (mdpi) configuration.
    // We should scan for higher resolution versions of the same file.
    if let Some(path) = icon_path {
        // Try to find a better version
        if let Some(best_data) = find_high_res_variant(&mut archive, &path) {
             return Ok(best_data);
        }
    
        // Fallback to the exact path found in ARSC
        if let Ok(mut file) = archive.by_name(&path) {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            return Ok(data);
        }
    }
    
    // Fallback: Heuristic search if parsing fail or no icon
    
    let mut best_icon_index = None;
    let mut max_score = 0;
    
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        let name_lower = name.to_lowercase();
        let size = file.size();
        
        if !name_lower.ends_with(".png") { continue; }
        if name_lower.contains(".9.png") { continue; }
        
        let mut score = 0;
        if name_lower.contains("launcher") { score += 50; }
        if name_lower.contains("mipmap") { score += 10; }
        if name_lower.contains("xhdpi") { score += 20; }
        
        // Basic filter
        if size < 1000 || size > 100_000 { continue; }

        if score > max_score {
            max_score = score;
            best_icon_index = Some(i);
        }
    }
    
    if let Some(idx) = best_icon_index {
        if let Ok(mut file) = archive.by_index(idx) {
            let mut data = Vec::new();
            if file.read_to_end(&mut data).is_ok() {
                return Ok(data);
            }
        }
    }
    
    anyhow::bail!("No icon found via Manifest or Heuristic")
}

/// Helper to find higher resolution variant of an icon path
fn find_high_res_variant(archive: &mut ZipArchive<Cursor<&[u8]>>, original_path: &str) -> Option<Vec<u8>> {
    let path_parts: Vec<&str> = original_path.split('/').collect();
    if path_parts.len() < 2 { return None; } // expect res/type/name
    
    // Check if it's in res/
    if path_parts[0] != "res" { return None; }
    
    // Extract type (mipmap or drawable) from parent folder
    // Parent folder example: "mipmap-mdpi" or "mipmap"
    let parent_dir = path_parts[path_parts.len() - 2];
    let resource_type = parent_dir.split('-').next().unwrap_or(parent_dir);
    
    // Extract filename stem (ic_launcher) from filename (ic_launcher.png or ic_launcher.xml)
    let filename = path_parts[path_parts.len() - 1];
    let stem = filename.split('.').next().unwrap_or(filename);
    
    // Scan for candidates
    let mut candidates: Vec<(usize, i32)> = Vec::new(); // (index, score)
    
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let name = file.name();
            
            // Fast check: must contain stem and end in .png
            if !name.ends_with(".png") { continue; }
            
            let parts: Vec<&str> = name.split('/').collect();
            if parts.len() < 2 { continue; }
            
            // Check resource type match
            let file_parent = parts[parts.len() - 2];
            if !file_parent.starts_with(resource_type) { continue; }
            
            // Check filename stem match
            let file_name = parts[parts.len() - 1];
            if !file_name.starts_with(stem) { continue; }
            
            // Check strictly that stem matches (ic_launcher.png vs ic_launcher_background.png)
            if file_name != format!("{}.png", stem) { continue; }

            // Score based on density
            let mut score = 0;
            if file_parent.contains("xxxhdpi") { score = 100; }
            else if file_parent.contains("xxhdpi") { score = 90; }
            else if file_parent.contains("xhdpi") { score = 80; }
            else if file_parent.contains("hdpi") { score = 70; }
            else if file_parent.contains("mdpi") { score = 60; }
            else if file_parent.contains("anydpi") { score = 50; } // XML alias? but we filtered for png.
            
            candidates.push((i, score));
        }
    }
    
    // Sort by score desc
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    
    if let Some((idx, _)) = candidates.first() {
        if let Ok(mut file) = archive.by_index(*idx) {
            let mut data = Vec::new();
            if file.read_to_end(&mut data).is_ok() {
                return Some(data);
            }
        }
    }
    
    None
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
    // New dump implementation using AXML
    let cursor = Cursor::new(apk_data);
    let Ok(mut archive) = ZipArchive::new(cursor) else {
        return "Error: Could not open APK".to_string();
    };

    let mut arsc = None;
    let mut arsc_data = Vec::new();
    if let Ok(mut file) = archive.by_name("resources.arsc") {
        if file.read_to_end(&mut arsc_data).is_ok() {
            let mut input = arsc_data.as_slice();
            if let Ok(parsed) = ARSC::new(&mut input) {
                arsc = Some(parsed);
            }
        }
    }

    let Ok(mut manifest_file) = archive.by_name("AndroidManifest.xml") else {
        return "Error: No AndroidManifest.xml found".to_string();
    };
    
    let mut manifest_data = Vec::new();
    if manifest_file.read_to_end(&mut manifest_data).is_err() {
        return "Error: Could not read manifest".to_string();
    }
    
    let mut input = manifest_data.as_slice();
    match AXML::new(&mut input, arsc.as_ref()) {
        Ok(axml) => axml.get_xml_string(),
        Err(e) => format!("Error parsing manifest: {:?}", e),
    }
}
