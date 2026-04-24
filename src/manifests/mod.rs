use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct GameManifest {
    pub name: String,
    pub identifiers: Identifiers,
    #[serde(default)]
    pub tools: HashMap<String, ModTool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Identifiers {
    pub steam_app_id: Option<u32>,
    pub heroic_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModTool {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub winetricks: Vec<String>,
    pub download_url: Option<String>,
    pub executable_path: Option<String>,
}

pub fn load_manifests() -> Vec<GameManifest> {
    let mut manifests = Vec::new();
    
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    
    // Procura em ~/.config/gamepilot/manifests/ e na pasta local
    let paths = vec![
        std::path::PathBuf::from(format!("{}/.config/gamepilot/manifests", home)),
        std::path::PathBuf::from("manifests"),
    ];
    
    for path in paths {
        if path.exists() && path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&path) {
                for entry in entries.flatten() {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(manifest) = toml::from_str::<GameManifest>(&content) {
                                manifests.push(manifest);
                            } else {
                                eprintln!("Erro ao parsear manifesto: {:?}", entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
    
    manifests
}
