use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Retorna o caminho do arquivo de configuração do GamePiLot.
/// Ex: ~/.config/gamepilot/config.toml
fn config_path() -> PathBuf {
    let base = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("gamepilot").join("config.toml")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub nexus: NexusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NexusConfig {
    /// Chave de acesso pessoal do Nexus Mods (opcional)
    pub api_key: Option<String>,
}

impl AppConfig {
    /// Carrega a configuração do disco. Se não existir, retorna os valores padrão.
    pub fn load() -> Self {
        let path = config_path();
        if !path.exists() {
            return Self::default();
        }
        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Salva a configuração atual no disco.
    pub fn save(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = toml::to_string_pretty(self) {
            let _ = fs::write(&path, content);
        }
    }

    /// Verifica se a chave do Nexus está configurada.
    pub fn nexus_connected(&self) -> bool {
        self.nexus.api_key
            .as_ref()
            .map(|k| !k.trim().is_empty())
            .unwrap_or(false)
    }
}
