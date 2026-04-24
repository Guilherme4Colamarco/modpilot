use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize, Clone)]
pub struct GameManifest {
    pub name: String,
    pub identifiers: Identifiers,
    #[serde(default)]
    pub tools: BTreeMap<String, ModTool>,
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
    pub nexus_game_domain: Option<String>,
    pub nexus_mod_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GamesFile {
    #[serde(default)]
    games: HashMap<String, GameManifest>,
}

pub fn load_manifests() -> Vec<GameManifest> {
    let mut manifests = Vec::new();

    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());

    let paths = [
        std::path::PathBuf::from(format!("{}/.config/gamepilot/manifests", home)),
        std::path::PathBuf::from("manifests"),
    ];

    for path in &paths {
        if !path.is_dir() {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(path) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("toml") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&p) else {
                continue;
            };

            // Primeiro tenta o formato unificado ([games.<slug>] ...)
            match toml::from_str::<GamesFile>(&content) {
                Ok(file) if !file.games.is_empty() => {
                    manifests.extend(file.games.into_values());
                    continue;
                }
                _ => {}
            }

            // Fallback: arquivo único por jogo (formato legado)
            match toml::from_str::<GameManifest>(&content) {
                Ok(m) => manifests.push(m),
                Err(e) => eprintln!("Erro ao parsear manifesto {:?}: {}", p, e),
            }
        }
    }

    manifests
}
