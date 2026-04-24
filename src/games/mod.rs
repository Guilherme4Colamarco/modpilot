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
}

pub fn scan_all_games() -> Vec<String> {
    let mut all_games = scan_steam_games();
    all_games.extend(scan_heroic_games());
    all_games
}

pub fn scan_steam_games() -> Vec<String> {
    let mut games_found = Vec::new();

    match SteamDir::locate() {
        Ok(steam_dir) => {
            if let Ok(libraries) = steam_dir.libraries() {
                for lib in libraries {
                    if let Ok(library) = lib {
                        let apps = library.apps();
                        for app in apps {
                            if let Ok(app_info) = app {
                                let name = app_info.name
                                    .as_deref()
                                    .unwrap_or("Desconhecido")
                                    .to_string();
                                games_found.push(format!("{} (Steam)", name));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Erro ao localizar Steam: {}", e),
    }

    games_found
}

pub fn scan_heroic_games() -> Vec<String> {
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
                            games.push(format!("{} (Heroic GOG)", name_str));
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
                for (_key, game_data) in obj {
                    if let Some(install_path) = game_data.get("install_path").and_then(|p| p.as_str()) {
                        let path = std::path::Path::new(install_path);
                        if path.exists() {
                            let title = game_data.get("title").and_then(|t| t.as_str())
                                .or_else(|| path.file_name().and_then(|n| n.to_str()))
                                .unwrap_or("Desconhecido");
                            games.push(format!("{} (Heroic Epic)", title));
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
                                    games.push(format!("{} (Heroic Amazon)", name_str));
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
