use steamlocate::SteamDir;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct HeroicGogConfig {
    installed: Vec<HeroicGogGame>,
}

#[derive(Deserialize)]
struct HeroicGogGame {
    install_path: String,
    app_name: String,
}

fn get_heroic_prefix(app_name: &str) -> Option<String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());

    let config_path = format!("{}/.config/heroic/GamesConfig/{}.json", home, app_name);
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            // Heroic aninha os dados dentro de json[app_name]{...}
            let game_config = json.get(app_name).unwrap_or(&json);
            if let Some(prefix) = game_config.get("winePrefix").and_then(|p| p.as_str()) {
                if !prefix.is_empty() {
                    return Some(prefix.to_string());
                }
            }
        }
    }

    // Fallback: prefixo padrão do Heroic
    let default_prefix = format!("{}/.local/share/heroic/Prefixes/{}", home, app_name);
    if std::path::Path::new(&default_prefix).exists() {
        return Some(default_prefix);
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

                        // Detecção do prefixo Steam: compatdata/{app_id}/pfx
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
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    
    // GOG Store (Array)
    let gog_path = format!("{}/.config/heroic/gog_store/installed.json", home);
    if let Ok(content) = fs::read_to_string(&gog_path) {
        if let Ok(config) = serde_json::from_str::<HeroicGogConfig>(&content) {
            for game in config.installed {
                let path = std::path::Path::new(&game.install_path);
                if path.exists() {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            let prefix = get_heroic_prefix(&game.app_name);
                            games.push((format!("{} (Heroic GOG)", name_str), prefix, None));
                        }
                    }
                }
            }
        }
    }

    // Epic / Legendary (Dicionário)
    let legendary_path = format!("{}/.config/heroic/legendaryConfig/legendary/installed.json", home);
    if let Ok(content) = fs::read_to_string(&legendary_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(obj) = json.as_object() {
                for (key, game_data) in obj {
                    if let Some(install_path) = game_data.get("install_path").and_then(|p| p.as_str()) {
                        let path = std::path::Path::new(install_path);
                        if path.exists() {
                            let title = game_data.get("title").and_then(|t| t.as_str())
                                .or_else(|| path.file_name().and_then(|n| n.to_str()))
                                .unwrap_or("Desconhecido");
                            let prefix = get_heroic_prefix(key);
                            games.push((format!("{} (Heroic Epic)", title), prefix, None));
                        }
                    }
                }
            }
        }
    }

    // Amazon / Nile (Array similar ao GOG na maioria das vezes, mas checando como Value genérico)
    let nile_path = format!("{}/.config/heroic/nile_store/installed.json", home);
    if let Ok(content) = fs::read_to_string(&nile_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(arr) = json.as_array() {
                for game_data in arr {
                    if let Some(install_path) = game_data.get("install_path").and_then(|p| p.as_str()) {
                        let path = std::path::Path::new(install_path);
                        if path.exists() {
                            if let Some(name) = path.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    let app_name = game_data.get("app_name").and_then(|a| a.as_str()).unwrap_or("");
                                    let prefix = get_heroic_prefix(app_name);
                                    games.push((format!("{} (Heroic Amazon)", name_str), prefix, None));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    games
}
