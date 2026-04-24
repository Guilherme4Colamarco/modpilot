use std::env;
use std::sync::OnceLock;

static LANG: OnceLock<Language> = OnceLock::new();

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Portuguese,
    Spanish,
    Italian,
}

pub fn get_language() -> Language {
    *LANG.get_or_init(|| {
        let lang_env = env::var("LANG").unwrap_or_else(|_| "en".to_string());
        if lang_env.starts_with("pt") {
            Language::Portuguese
        } else if lang_env.starts_with("es") {
            Language::Spanish
        } else if lang_env.starts_with("it") {
            Language::Italian
        } else {
            Language::English
        }
    })
}

pub fn t(key: &str) -> String {
    let lang = get_language();
    
    let text = match key {
        // Main UI
        "scan_games" => match lang {
            Language::Portuguese => "Escanear Jogos",
            Language::Spanish => "Escanear Juegos",
            Language::Italian => "Scansiona Giochi",
            Language::English => "Scan Games",
        },
        "dependencies_mods" => match lang {
            Language::Portuguese => "Dependências (Mods):",
            Language::Spanish => "Dependencias (Mods):",
            Language::Italian => "Dipendenze (Mods):",
            Language::English => "Dependencies (Mods):",
        },
        "install_tool" => match lang {
            Language::Portuguese => "Instalar Ferramenta",
            Language::Spanish => "Instalar Herramienta",
            Language::Italian => "Installa Strumento",
            Language::English => "Install Tool",
        },
        "wine_tools" => match lang {
            Language::Portuguese => "Ferramentas Wine",
            Language::Spanish => "Herramientas Wine",
            Language::Italian => "Strumenti Wine",
            Language::English => "Wine Tools",
        },
        "wine_prefix_placeholder" => match lang {
            Language::Portuguese => "Caminho do prefixo Wine (ex: /home/user/.wine)",
            Language::Spanish => "Ruta del prefijo Wine (ej: /home/user/.wine)",
            Language::Italian => "Percorso del prefisso Wine (es: /home/user/.wine)",
            Language::English => "Wine prefix path (e.g. /home/user/.wine)",
        },
        "execute_wine_command" => match lang {
            Language::Portuguese => "Executar Comando Wine",
            Language::Spanish => "Ejecutar Comando Wine",
            Language::Italian => "Esegui Comando Wine",
            Language::English => "Execute Wine Command",
        },
        
        // Status Messages
        "welcome" => match lang {
            Language::Portuguese => "Bem-vindo ao GamePiLot!",
            Language::Spanish => "¡Bienvenido a GamePiLot!",
            Language::Italian => "Benvenuto in GamePiLot!",
            Language::English => "Welcome to GamePiLot!",
        },
        "no_games_scanned" => match lang {
            Language::Portuguese => "Nenhum jogo escaneado",
            Language::Spanish => "Ningún juego escaneado",
            Language::Italian => "Nessun gioco scansionato",
            Language::English => "No games scanned",
        },
        "no_game_selected" => match lang {
            Language::Portuguese => "Nenhum jogo selecionado",
            Language::Spanish => "Ningún juego seleccionado",
            Language::Italian => "Nessun gioco selezionato",
            Language::English => "No game selected",
        },
        "no_games_found" => match lang {
            Language::Portuguese => "Nenhum jogo encontrado.",
            Language::Spanish => "No se encontraron juegos.",
            Language::Italian => "Nessun gioco trovato.",
            Language::English => "No games found.",
        },
        "no_games_found_short" => match lang {
            Language::Portuguese => "Nenhum jogo encontrado",
            Language::Spanish => "No se encontraron juegos",
            Language::Italian => "Nessun gioco trovato",
            Language::English => "No games found",
        },
        "no_mod_manifest_found" => match lang {
            Language::Portuguese => "Nenhum manifesto de mod encontrado",
            Language::Spanish => "No se encontró manifiesto de mod",
            Language::Italian => "Nessun manifesto mod trovato",
            Language::English => "No mod manifest found",
        },
        "no_wine_prefix" => match lang {
            Language::Portuguese => "❌ Nenhum prefixo Wine detectado! Selecione um jogo com prefixo ou preencha o campo manualmente.",
            Language::Spanish => "❌ ¡No se detectó prefijo Wine! Selecciona un juego con prefijo o completa el campo manualmente.",
            Language::Italian => "❌ Nessun prefisso Wine rilevato! Seleziona un gioco con un prefisso o compila il campo manualmente.",
            Language::English => "❌ No Wine prefix detected! Select a game with a prefix or fill the field manually.",
        },
        "installing_dependencies" => match lang {
            Language::Portuguese => "Instalando dependências...",
            Language::Spanish => "Instalando dependencias...",
            Language::Italian => "Installazione delle dipendenze...",
            Language::English => "Installing dependencies...",
        },
        "preparing" => match lang {
            Language::Portuguese => "Preparando...",
            Language::Spanish => "Preparando...",
            Language::Italian => "Preparazione...",
            Language::English => "Preparing...",
        },
        "dependencies_installed" => match lang {
            Language::Portuguese => "Dependências instaladas!",
            Language::Spanish => "¡Dependencias instaladas!",
            Language::Italian => "Dipendenze installate!",
            Language::English => "Dependencies installed!",
        },
        "done" => match lang {
            Language::Portuguese => "🏁 Concluído.",
            Language::Spanish => "🏁 Terminado.",
            Language::Italian => "🏁 Finito.",
            Language::English => "🏁 Done.",
        },
        "enter_wine_prefix" => match lang {
            Language::Portuguese => "Por favor, insira o caminho do prefixo Wine.",
            Language::Spanish => "Por favor, introduzca la ruta del prefijo Wine.",
            Language::Italian => "Si prega di inserire il percorso del prefisso Wine.",
            Language::English => "Please, enter the Wine prefix path.",
        },
        "base_dependencies" => match lang {
            Language::Portuguese => "📦 Instalando dependências base...",
            Language::Spanish => "📦 Instalando dependencias base...",
            Language::Italian => "📦 Installazione dipendenze di base...",
            Language::English => "📦 Installing base dependencies...",
        },
        "running_installer" => match lang {
            Language::Portuguese => "📦 Executando instalador da ferramenta...",
            Language::Spanish => "📦 Ejecutando instalador de la herramienta...",
            Language::Italian => "📦 Esecuzione del programma di installazione dello strumento...",
            Language::English => "📦 Running tool installer...",
        },
        "setup_finished" => match lang {
            Language::Portuguese => "✅ Configuração da ferramenta concluída.",
            Language::Spanish => "✅ Configuración de la herramienta completada.",
            Language::Italian => "✅ Configurazione dello strumento completata.",
            Language::English => "✅ Tool setup finished.",
        },
        "finalizing" => match lang {
            Language::Portuguese => "🏁 Finalizando configuração...",
            Language::Spanish => "🏁 Finalizando configuración...",
            Language::Italian => "🏁 Completamento configurazione...",
            Language::English => "🏁 Finalizing setup...",
        },
        
        // Se a chave não for encontrada, retorna ela mesma
        _ => return key.to_string(),
    };
    
    text.to_string()
}

// Helpers com formatação
pub fn t_install(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Instalar {}", name),
        Language::Spanish => format!("Instalar {}", name),
        Language::Italian => format!("Installa {}", name),
        Language::English => format!("Install {}", name),
    }
}

pub fn t_launch(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Lançar {} 🚀", name),
        Language::Spanish => format!("Lanzar {} 🚀", name),
        Language::Italian => format!("Avvia {} 🚀", name),
        Language::English => format!("Launch {} 🚀", name),
    }
}

pub fn t_tools_not_found(missing: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("⚠️ Ferramentas não encontradas: {}. Instale antes de usar.", missing),
        Language::Spanish => format!("⚠️ Herramientas no encontradas: {}. Instálelas antes de usar.", missing),
        Language::Italian => format!("⚠️ Strumenti non trovati: {}. Installare prima dell'uso.", missing),
        Language::English => format!("⚠️ Tools not found: {}. Install before using.", missing),
    }
}

pub fn t_games_found(count: usize) -> String {
    match get_language() {
        Language::Portuguese => format!("{} jogos encontrados.", count),
        Language::Spanish => format!("{} juegos encontrados.", count),
        Language::Italian => format!("{} giochi trovati.", count),
        Language::English => format!("{} games found.", count),
    }
}

pub fn t_starting(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Iniciando {}...", name),
        Language::Spanish => format!("Iniciando {}...", name),
        Language::Italian => format!("Avvio di {}...", name),
        Language::English => format!("Starting {}...", name),
    }
}

pub fn t_started(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("{} iniciado! 🚀", name),
        Language::Spanish => format!("¡{} iniciado! 🚀", name),
        Language::Italian => format!("{} avviato! 🚀", name),
        Language::English => format!("{} started! 🚀", name),
    }
}

pub fn t_error_launching(err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Erro ao lançar: {}", err),
        Language::Spanish => format!("❌ Error al lanzar: {}", err),
        Language::Italian => format!("❌ Errore durante l'avvio: {}", err),
        Language::English => format!("❌ Error launching: {}", err),
    }
}

pub fn t_prefix_not_found(path: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Prefixo não encontrado: {}", path),
        Language::Spanish => format!("❌ Prefijo no encontrado: {}", path),
        Language::Italian => format!("❌ Prefisso non trovato: {}", path),
        Language::English => format!("❌ Prefix not found: {}", path),
    }
}

pub fn t_executing(cmd: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Executando {}...", cmd),
        Language::Spanish => format!("Ejecutando {}...", cmd),
        Language::Italian => format!("Esecuzione di {}...", cmd),
        Language::English => format!("Executing {}...", cmd),
    }
}

pub fn t_command_started(cmd: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Comando {} iniciado.", cmd),
        Language::Spanish => format!("Comando {} iniciado.", cmd),
        Language::Italian => format!("Comando {} avviato.", cmd),
        Language::English => format!("Command {} started.", cmd),
    }
}

pub fn t_error(err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Erro: {}", err),
        Language::Spanish => format!("Error: {}", err),
        Language::Italian => format!("Errore: {}", err),
        Language::English => format!("Error: {}", err),
    }
}

// Para wine/mod.rs
pub fn t_downloading(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("📦 Baixando {}...", name),
        Language::Spanish => format!("📦 Descargando {}...", name),
        Language::Italian => format!("📦 Download di {}...", name),
        Language::English => format!("📦 Downloading {}...", name),
    }
}

pub fn t_failed_download(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Falha no download de {}", name),
        Language::Spanish => format!("❌ Error al descargar {}", name),
        Language::Italian => format!("❌ Download di {} non riuscito", name),
        Language::English => format!("❌ Failed to download {}", name),
    }
}

pub fn t_installing_via_winetricks(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("📦 Instalando {} via winetricks...", name),
        Language::Spanish => format!("📦 Instalando {} a través de winetricks...", name),
        Language::Italian => format!("📦 Installazione di {} tramite winetricks...", name),
        Language::English => format!("📦 Installing {} via winetricks...", name),
    }
}

pub fn t_failed_install(name: &str, err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Falha ao instalar {}: {}", name, err),
        Language::Spanish => format!("❌ Error al instalar {}: {}", name, err),
        Language::Italian => format!("❌ Impossibile installare {}: {}", name, err),
        Language::English => format!("❌ Failed to install {}: {}", name, err),
    }
}
