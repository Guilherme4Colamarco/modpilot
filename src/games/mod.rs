use serde::Deserialize;
use std::fs;
use steamlocate::SteamDir;

#[derive(Deserialize, Debug)]
struct HeroicGogConfig {
    installed: Vec<HeroicGogGame>,
}

#[derive(Deserialize, Debug)]
struct HeroicGogGame {
    install_path: String,
    // O JSON do Heroic usa camelCase aqui.
    #[serde(rename = "appName")]
    app_name: String,
}

fn heroic_config_roots() -> Vec<String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    vec![
        format!("{}/.config/heroic", home),
        // Flatpak
        format!(
            "{}/.var/app/com.heroicgameslauncher.hgl/config/heroic",
            home
        ),
    ]
}

fn heroic_default_prefix_roots() -> Vec<String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    vec![
        format!("{}/Games/Heroic/Prefixes/default", home),
        format!("{}/.local/share/heroic/Prefixes", home),
        format!(
            "{}/.var/app/com.heroicgameslauncher.hgl/data/heroic/Prefixes",
            home
        ),
    ]
}

fn get_heroic_prefix(app_name: &str) -> Option<String> {
    // 1. Lê o GamesConfig específico, se existir
    for root in heroic_config_roots() {
        let config_path = format!("{}/GamesConfig/{}.json", root, app_name);
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                // Heroic aninha os dados dentro de json[app_name]{...}
                let game_config = json.get(app_name).unwrap_or(&json);
                if let Some(prefix) = game_config.get("winePrefix").and_then(|p| p.as_str()) {
                    if !prefix.is_empty() && std::path::Path::new(prefix).exists() {
                        return Some(prefix.to_string());
                    }
                }
            }
        }
    }

    // 2. Fallback: procura nos diretórios padrão de prefixos
    for root in heroic_default_prefix_roots() {
        let candidate = format!("{}/{}", root, app_name);
        if std::path::Path::new(&candidate).exists() {
            return Some(candidate);
        }
    }

    None
}

/// Retorna tupla: (nome_exibição, prefixo_wine, steam_app_id)
pub fn scan_all_games() -> Vec<(String, Option<String>, Option<u32>)> {
    let mut all_games = scan_steam_games();
    all_games.extend(scan_heroic_games());
    all_games
}

pub fn scan_steam_games() -> Vec<(String, Option<String>, Option<u32>)> {
    let mut games_found = Vec::new();

    match SteamDir::locate() {
        Ok(steam_dir) => {
            if let Ok(libraries) = steam_dir.libraries() {
                for library in libraries.flatten() {
                    for app_info in library.apps().flatten() {
                        let name = app_info
                            .name
                            .as_deref()
                            .unwrap_or("Desconhecido")
                            .to_string();

                        let app_id = app_info.app_id;
                        let prefix_path = library
                            .path()
                            .join("steamapps/compatdata")
                            .join(app_id.to_string())
                            .join("pfx");

                        let prefix_path_str = if prefix_path.exists() {
                            Some(prefix_path.to_string_lossy().to_string())
                        } else {
                            None
                        };

                        games_found.push((
                            format!("{} (Steam)", name),
                            prefix_path_str,
                            Some(app_id),
                        ));
                    }
                }
            }
        }
        Err(e) => eprintln!("Erro ao localizar Steam: {}", e),
    }

    games_found
}

pub fn scan_heroic_games() -> Vec<(String, Option<String>, Option<u32>)> {
    let mut games = Vec::new();

    for root in heroic_config_roots() {
        if !std::path::Path::new(&root).exists() {
            continue;
        }

        // GOG
        let gog_path = format!("{}/gog_store/installed.json", root);
        if let Ok(content) = fs::read_to_string(&gog_path) {
            match serde_json::from_str::<HeroicGogConfig>(&content) {
                Ok(config) => {
                    for game in config.installed {
                        let path = std::path::Path::new(&game.install_path);
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Desconhecido")
                            .to_string();
                        let prefix = get_heroic_prefix(&game.app_name);
                        games.push((format!("{} (Heroic GOG)", name), prefix, None));
                    }
                }
                Err(e) => eprintln!("Erro ao parsear {}: {}", gog_path, e),
            }
        }

        // Epic / Legendary
        let legendary_path = format!("{}/legendaryConfig/legendary/installed.json", root);
        if let Ok(content) = fs::read_to_string(&legendary_path) {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => {
                    if let Some(obj) = json.as_object() {
                        for (key, game_data) in obj {
                            let install_path = game_data
                                .get("install_path")
                                .and_then(|p| p.as_str())
                                .unwrap_or("");
                            let path = std::path::Path::new(install_path);
                            let title = game_data
                                .get("title")
                                .and_then(|t| t.as_str())
                                .or_else(|| path.file_name().and_then(|n| n.to_str()))
                                .unwrap_or("Desconhecido")
                                .to_string();
                            let prefix = get_heroic_prefix(key);
                            games.push((format!("{} (Heroic Epic)", title), prefix, None));
                        }
                    }
                }
                Err(e) => eprintln!("Erro ao parsear {}: {}", legendary_path, e),
            }
        }

        // Amazon / Nile — aceita tanto array solto quanto {"installed": [...]}
        let nile_candidates = [
            format!("{}/nile_config/nile/installed.json", root),
            format!("{}/nile_store/installed.json", root),
        ];
        for nile_path in &nile_candidates {
            let Ok(content) = fs::read_to_string(nile_path) else {
                continue;
            };
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let iter: Box<dyn Iterator<Item = &serde_json::Value>> =
                    if let Some(arr) = json.as_array() {
                        Box::new(arr.iter())
                    } else if let Some(arr) = json.get("installed").and_then(|v| v.as_array()) {
                        Box::new(arr.iter())
                    } else {
                        continue;
                    };
                for game_data in iter {
                    let install_path = game_data
                        .get("install_path")
                        .and_then(|p| p.as_str())
                        .unwrap_or("");
                    let path = std::path::Path::new(install_path);
                    if !path.exists() {
                        continue;
                    }
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Desconhecido")
                        .to_string();
                    let app_name = game_data
                        .get("app_name")
                        .or_else(|| game_data.get("id"))
                        .and_then(|a| a.as_str())
                        .unwrap_or("");
                    let prefix = get_heroic_prefix(app_name);
                    games.push((format!("{} (Heroic Amazon)", name), prefix, None));
                }
            }
        }
    }

    games
}
