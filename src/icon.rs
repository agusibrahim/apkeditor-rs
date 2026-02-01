//! APK Icon handling module
//!
//! This module provides functionality for finding and replacing app icons in APK files.

use std::io::{Cursor, Read};
use anyhow::Result;
use zip::ZipArchive;

use apk_info_axml::{AXML, ARSC};

/// Android screen density qualifiers with their corresponding sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Density {
    Ldpi = 120,
    Mdpi = 160,
    Hdpi = 240,
    Xhdpi = 320,
    XXhdpi = 480,
    XXXhdpi = 640,
    Any,    // Adaptive icons
    Tv,     // TV icons (typically 192x192)
}

impl Density {
    /// Parse density from folder name (e.g., "mipmap-xxxhdpi" -> Density::XXXhdpi)
    pub fn from_folder_name(folder: &str) -> Option<Self> {
        let folder_lower = folder.to_lowercase();
        if folder_lower.contains("xxxhdpi") {
            Some(Density::XXXhdpi)
        } else if folder_lower.contains("xxhdpi") {
            Some(Density::XXhdpi)
        } else if folder_lower.contains("xhdpi") {
            Some(Density::Xhdpi)
        } else if folder_lower.contains("hdpi") {
            Some(Density::Hdpi)
        } else if folder_lower.contains("mdpi") {
            Some(Density::Mdpi)
        } else if folder_lower.contains("ldpi") {
            Some(Density::Ldpi)
        } else if folder_lower.contains("anydpi") {
            Some(Density::Any)
        } else if folder_lower.contains("tvdpi") || folder_lower.contains("tvdpi") {
            Some(Density::Tv)
        } else {
            None
        }
    }

    /// Get recommended icon size for this density (for xxxhdpi base)
    pub fn icon_size(self) -> u32 {
        match self {
            Density::Ldpi => 36,      // 36x36
            Density::Mdpi => 48,      // 48x48
            Density::Hdpi => 72,      // 72x72
            Density::Xhdpi => 96,     // 96x96
            Density::XXhdpi => 144,   // 144x144
            Density::XXXhdpi => 192,  // 192x192
            Density::Any => 192,      // Adaptive icon layer
            Density::Tv => 192,       // TV banner/icon
        }
    }

    /// Get multiplier from mdpi baseline
    pub fn multiplier(self) -> f32 {
        match self {
            Density::Ldpi => 0.75,
            Density::Mdpi => 1.0,
            Density::Hdpi => 1.5,
            Density::Xhdpi => 2.0,
            Density::XXhdpi => 3.0,
            Density::XXXhdpi => 4.0,
            Density::Any => 4.0,
            Density::Tv => 4.0,
        }
    }
}

/// Information about an icon file in the APK
#[derive(Debug, Clone)]
pub struct IconInfo {
    pub path: String,
    pub density: Option<Density>,
    pub is_adaptive: bool,
}

/// Find all launcher icon paths in the APK
///
/// This function uses the same logic as extract_icon() to find the base icon,
/// then scans for all density variants.
pub fn find_icon_paths(apk_data: &[u8]) -> Result<Vec<IconInfo>> {
    let cursor = Cursor::new(apk_data);
    let mut archive = ZipArchive::new(cursor)?;

    // 1. Try to read resources.arsc for resolving resource references
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

    // 2. Read Manifest to find the icon path (same logic as extract_icon)
    let mut icon_path = None;
    let mut resource_name = None; // Store resource name like "drawable/icon" or just "icon"

    if let Ok(mut manifest_file) = archive.by_name("AndroidManifest.xml") {
        let mut manifest_data = Vec::new();
        manifest_file.read_to_end(&mut manifest_data)?;
        drop(manifest_file);

        let mut input = manifest_data.as_slice();
        if let Ok(axml) = AXML::new(&mut input, arsc.as_ref()) {
            // Priority 1: Launcher Activity Icon
            let main_activities: Vec<String> = axml.get_main_activities().map(|s| s.to_string()).collect();

            if let Some(main_activity) = main_activities.first() {
                if let Some(app) = axml.root.childrens().find(|c| c.name() == "application") {
                    if let Some(act_node) = app.childrens().find(|c| c.attr("name") == Some(main_activity.as_str())) {
                        if let Some(val) = act_node.attr("icon") {
                            eprintln!("[ICON DEBUG] Launcher activity icon raw value: {:?}", val);
                            if val.starts_with("@") {
                                resource_name = Some(val[1..].to_string()); // Store "drawable/icon"
                                eprintln!("[ICON DEBUG] Resource name stored: {:?}", resource_name);
                                if let Some(arsc_ref) = arsc.as_ref() {
                                    let resolved = arsc_ref.get_resource_value_by_name(&val[1..]);
                                    eprintln!("[ICON DEBUG] ARSC resolved path: {:?}", resolved);
                                    icon_path = resolved;
                                }
                            } else {
                                eprintln!("[ICON DEBUG] Direct path (no @): {}", val);
                                icon_path = Some(val.to_string());
                            }
                        }
                    }
                }
            }

            // Priority 2: Application Icon
            if icon_path.is_none() {
                // Get RAW value first without ARSC resolution
                if let Some(app) = axml.root.childrens().find(|c| c.name() == "application") {
                    if let Some(val) = app.attr("icon") {
                        eprintln!("[ICON DEBUG] Application icon raw value: {:?}", val);
                        if val.starts_with("@") {
                            if resource_name.is_none() {
                                resource_name = Some(val[1..].to_string());
                                eprintln!("[ICON DEBUG] Resource name stored (application): {:?}", resource_name);
                            }
                            if let Some(arsc_ref) = arsc.as_ref() {
                                let resolved = arsc_ref.get_resource_value_by_name(&val[1..]);
                                eprintln!("[ICON DEBUG] ARSC resolved path (application): {:?}", resolved);
                                icon_path = resolved;
                            }
                        } else {
                            eprintln!("[ICON DEBUG] Direct path application icon: {}", val);
                            icon_path = Some(val.to_string());
                        }
                    }
                }
            }

            eprintln!("[ICON DEBUG] Final icon_path: {:?}, resource_name: {:?}", icon_path, resource_name);
        }
    }

    // 3. Scan for all density variants based on the found icon path
    let mut icons = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    // Determine resource type and filename stem from icon_path or resource_name
    let (resource_type, stem) = if let Some(ref base_path) = icon_path {
        // Try to parse from ARSC result path (e.g., "res/drawable-xxxhdpi/icon.png")
        eprintln!("[ICON DEBUG] Parsing from icon_path: {:?}", base_path);
        let path_parts: Vec<&str> = base_path.split('/').collect();
        if path_parts.len() >= 2 && path_parts[0] == "res" {
            let parent_dir = path_parts[path_parts.len() - 2];
            let resource_type = parent_dir.split('-').next().unwrap_or(parent_dir);
            let filename = path_parts[path_parts.len() - 1];
            let stem = filename.split('.').next().unwrap_or(filename);
            eprintln!("[ICON DEBUG] Parsed - resource_type: {}, stem: {}", resource_type, stem);
            (Some(resource_type.to_string()), Some(stem.to_string()))
        } else {
            eprintln!("[ICON DEBUG] Path doesn't start with 'res' or too short");
            (None, None)
        }
    } else if let Some(ref res_name) = resource_name {
        // Parse from resource name (e.g., "drawable/icon" or "mipmap/ic_launcher")
        eprintln!("[ICON DEBUG] Parsing from resource_name: {:?}", res_name);
        let parts: Vec<&str> = res_name.split('/').collect();
        if parts.len() == 2 {
            eprintln!("[ICON DEBUG] Parsed - resource_type: {}, stem: {}", parts[0], parts[1]);
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        } else {
            // Just a name, no type - search both mipmap and drawable
            eprintln!("[ICON DEBUG] Only stem, no resource_type: {}", parts[0]);
            (None, Some(parts[0].to_string()))
        }
    } else {
        eprintln!("[ICON DEBUG] No icon_path or resource_name, using fallback");
        (None, None)
    };

    eprintln!("[ICON DEBUG] Final - resource_type: {:?}, stem: {:?}", resource_type, stem);

    // Scan for all density variants
    if let (Some(ref res_type), Some(ref icon_stem)) = (&resource_type, &stem) {
        // We know both resource type and icon name - scan for exact matches
        eprintln!("[ICON DEBUG] Scanning for resource_type={} stem={}", res_type, icon_stem);
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let name = file.name();
                let name_lower = name.to_lowercase();

                if !name_lower.ends_with(".png") { continue; }
                if name_lower.contains(".9.png") { continue; }

                let parts: Vec<&str> = name.split('/').collect();
                if parts.len() < 2 { continue; }

                // Check resource type match
                let file_parent = parts[parts.len() - 2];
                if !file_parent.starts_with(res_type) { continue; }

                // Check filename stem match (exact match, not just prefix)
                let file_name = parts[parts.len() - 1];
                let file_stem = file_name.split('.').next().unwrap_or(file_name);
                if file_stem != *icon_stem { continue; }

                if !seen_paths.contains(name) {
                    eprintln!("[ICON DEBUG] Found icon: {}", name);
                    let density = Density::from_folder_name(file_parent);
                    let is_adaptive = name_lower.contains("anydpi") || name_lower.ends_with(".xml");

                    icons.push(IconInfo {
                        path: name.to_string(),
                        density,
                        is_adaptive,
                    });
                    seen_paths.insert(name.to_string());
                }
            }
        }
    } else if let Some(ref icon_stem) = stem {
        // We only know icon name, no resource type - search both mipmap and drawable
        eprintln!("[ICON DEBUG] Scanning for stem={} (no resource_type)", icon_stem);
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let name = file.name();
                let name_lower = name.to_lowercase();

                if !name_lower.ends_with(".png") { continue; }
                if name_lower.contains(".9.png") { continue; }

                let parts: Vec<&str> = name.split('/').collect();
                if parts.len() < 2 { continue; }

                let file_parent = parts[parts.len() - 2];
                // Must be in mipmap-* or drawable-* folder
                if !file_parent.starts_with("mipmap") && !file_parent.starts_with("drawable") { continue; }

                let file_name = parts[parts.len() - 1];
                let file_stem = file_name.split('.').next().unwrap_or(file_name);
                if file_stem != *icon_stem { continue; }

                if !seen_paths.contains(name) {
                    eprintln!("[ICON DEBUG] Found icon: {}", name);
                    let density = Density::from_folder_name(file_parent);
                    let is_adaptive = name_lower.contains("anydpi") || name_lower.ends_with(".xml");

                    icons.push(IconInfo {
                        path: name.to_string(),
                        density,
                        is_adaptive,
                    });
                    seen_paths.insert(name.to_string());
                }
            }
        }
    } else {
        eprintln!("[ICON DEBUG] No resource_type or stem available for scanning");
    }

    eprintln!("[ICON DEBUG] Total icons found: {}", icons.len());

    // Fallback: Heuristic scan if no icons found through manifest
    if icons.is_empty() {
        icons = heuristic_find_icons(&mut archive)?;
    }

    if icons.is_empty() {
        anyhow::bail!("No launcher icons found in APK");
    }

    // Sort by density (highest first) for consistency
    icons.sort_by(|a, b| {
        match (&a.density, &b.density) {
            (Some(da), Some(db)) => db.cmp(da),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.path.cmp(&b.path),
        }
    });

    Ok(icons)
}

/// Heuristic icon finding when manifest parsing fails
fn heuristic_find_icons(archive: &mut ZipArchive<Cursor<&[u8]>>) -> Result<Vec<IconInfo>> {
    let mut icons = Vec::new();

    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let name = file.name().to_string();
            let name_lower = name.to_lowercase();

            // Look for PNG files in mipmap or drawable folders
            if (name_lower.contains("mipmap") || name_lower.contains("drawable"))
                && name_lower.ends_with(".png")
                && !name_lower.contains(".9.png") {

                let parent = std::path::Path::new(&name)
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str());

                let density = parent.and_then(|p| Density::from_folder_name(p));

                icons.push(IconInfo {
                    path: name,
                    density,
                    is_adaptive: false,
                });
            }
        }
    }

    Ok(icons)
}

/// Resize PNG icon to target size
///
/// This is a placeholder - the actual PNG resizing will be done on the frontend
/// where we have access to Canvas API for image processing.
/// This function validates that the input data is a valid PNG.
pub fn validate_png(data: &[u8]) -> Result<()> {
    // Check PNG signature
    if data.len() < 8 {
        anyhow::bail!("Invalid PNG: file too small");
    }

    let png_signature: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    if &data[0..8] != &png_signature {
        anyhow::bail!("Invalid PNG: incorrect signature");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_from_folder_name() {
        assert_eq!(Density::from_folder_name("mipmap-xxxhdpi"), Some(Density::XXXhdpi));
        assert_eq!(Density::from_folder_name("drawable-xxhdpi"), Some(Density::XXhdpi));
        assert_eq!(Density::from_folder_name("mipmap-xhdpi"), Some(Density::Xhdpi));
        assert_eq!(Density::from_folder_name("drawable-hdpi"), Some(Density::Hdpi));
        assert_eq!(Density::from_folder_name("mipmap-mdpi"), Some(Density::Mdpi));
        assert_eq!(Density::from_folder_name("drawable-ldpi"), Some(Density::Ldpi));
        assert_eq!(Density::from_folder_name("mipmap-anydpi-v26"), Some(Density::Any));
        assert_eq!(Density::from_folder_name("drawable-tvdpi"), Some(Density::Tv));
        assert_eq!(Density::from_folder_name("values"), None);
    }

    #[test]
    fn test_density_icon_size() {
        assert_eq!(Density::Ldpi.icon_size(), 36);
        assert_eq!(Density::Mdpi.icon_size(), 48);
        assert_eq!(Density::Hdpi.icon_size(), 72);
        assert_eq!(Density::Xhdpi.icon_size(), 96);
        assert_eq!(Density::XXhdpi.icon_size(), 144);
        assert_eq!(Density::XXXhdpi.icon_size(), 192);
    }

    #[test]
    fn test_validate_png() {
        let valid_png = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(validate_png(&valid_png).is_ok());

        let invalid_png = [0x00, 0x00, 0x00, 0x00];
        assert!(validate_png(&invalid_png).is_err());
    }
}
