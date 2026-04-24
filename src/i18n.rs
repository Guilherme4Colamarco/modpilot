use std::env;
use std::sync::OnceLock;

static LANG: OnceLock<Language> = OnceLock::new();

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Portuguese,
    Spanish,
    Italian,
    Russian,
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
        } else if lang_env.starts_with("ru") {
            Language::Russian
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
            Language::Russian => "Сканировать игры",
            Language::English => "Scan Games",
        },
        "dependencies_mods" => match lang {
            Language::Portuguese => "Dependências (Mods):",
            Language::Spanish => "Dependencias (Mods):",
            Language::Italian => "Dipendenze (Mods):",
            Language::Russian => "Зависимости (Моды):",
            Language::English => "Dependencies (Mods):",
        },
        "install_tool" => match lang {
            Language::Portuguese => "Instalar Ferramenta",
            Language::Spanish => "Instalar Herramienta",
            Language::Italian => "Installa Strumento",
            Language::Russian => "Установить инструмент",
            Language::English => "Install Tool",
        },
        "wine_tools" => match lang {
            Language::Portuguese => "Ferramentas Wine",
            Language::Spanish => "Herramientas Wine",
            Language::Italian => "Strumenti Wine",
            Language::Russian => "Инструменты Wine",
            Language::English => "Wine Tools",
        },
        "wine_prefix_placeholder" => match lang {
            Language::Portuguese => "Caminho do prefixo Wine (ex: /home/user/.wine)",
            Language::Spanish => "Ruta del prefijo Wine (ej: /home/user/.wine)",
            Language::Italian => "Percorso del prefisso Wine (es: /home/user/.wine)",
            Language::Russian => "Путь к префиксу Wine (напр. /home/user/.wine)",
            Language::English => "Wine prefix path (e.g. /home/user/.wine)",
        },
        "execute_wine_command" => match lang {
            Language::Portuguese => "Executar Comando Wine",
            Language::Spanish => "Ejecutar Comando Wine",
            Language::Italian => "Esegui Comando Wine",
            Language::Russian => "Выполнить команду Wine",
            Language::English => "Execute Wine Command",
        },
        
        // Status Messages
        "welcome" => match lang {
            Language::Portuguese => "Bem-vindo ao GamePiLot!",
            Language::Spanish => "¡Bienvenido a GamePiLot!",
            Language::Italian => "Benvenuto in GamePiLot!",
            Language::Russian => "Добро пожаловать в GamePiLot!",
            Language::English => "Welcome to GamePiLot!",
        },
        "no_games_scanned" => match lang {
            Language::Portuguese => "Nenhum jogo escaneado",
            Language::Spanish => "Ningún juego escaneado",
            Language::Italian => "Nessun gioco scansionato",
            Language::Russian => "Игры не отсканированы",
            Language::English => "No games scanned",
        },
        "no_game_selected" => match lang {
            Language::Portuguese => "Nenhum jogo selecionado",
            Language::Spanish => "Ningún juego seleccionado",
            Language::Italian => "Nessun gioco selezionato",
            Language::Russian => "Игра не выбрана",
            Language::English => "No game selected",
        },
        "no_games_found" => match lang {
            Language::Portuguese => "Nenhum jogo encontrado.",
            Language::Spanish => "No se encontraron juegos.",
            Language::Italian => "Nessun gioco trovato.",
            Language::Russian => "Игры не найдены.",
            Language::English => "No games found.",
        },
        "no_games_found_short" => match lang {
            Language::Portuguese => "Nenhum jogo encontrado",
            Language::Spanish => "No se encontraron juegos",
            Language::Italian => "Nessun gioco trovato",
            Language::Russian => "Игры не найдены",
            Language::English => "No games found",
        },
        "no_mod_manifest_found" => match lang {
            Language::Portuguese => "Nenhum manifesto de mod encontrado",
            Language::Spanish => "No se encontró manifiesto de mod",
            Language::Italian => "Nessun manifesto mod trovato",
            Language::Russian => "Манифест мода не найден",
            Language::English => "No mod manifest found",
        },
        "no_wine_prefix" => match lang {
            Language::Portuguese => "❌ Nenhum prefixo Wine detectado! Selecione um jogo com prefixo ou preencha o campo manualmente.",
            Language::Spanish => "❌ ¡No se detectó prefijo Wine! Selecciona un juego con prefijo o completa el campo manualmente.",
            Language::Italian => "❌ Nessun prefisso Wine rilevato! Seleziona un gioco con un prefisso o compila il campo manualmente.",
            Language::Russian => "❌ Префикс Wine не обнаружен! Выберите игру с префиксом или введите путь вручную.",
            Language::English => "❌ No Wine prefix detected! Select a game with a prefix or fill the field manually.",
        },
        "installing_dependencies" => match lang {
            Language::Portuguese => "Instalando dependências...",
            Language::Spanish => "Instalando dependencias...",
            Language::Italian => "Installazione delle dipendenze...",
            Language::Russian => "Установка зависимостей...",
            Language::English => "Installing dependencies...",
        },
        "preparing" => match lang {
            Language::Portuguese => "Preparando...",
            Language::Spanish => "Preparando...",
            Language::Italian => "Preparazione...",
            Language::Russian => "Подготовка...",
            Language::English => "Preparing...",
        },
        "dependencies_installed" => match lang {
            Language::Portuguese => "Dependências instaladas!",
            Language::Spanish => "¡Dependencias instaladas!",
            Language::Italian => "Dipendenze installate!",
            Language::Russian => "Зависимости установлены!",
            Language::English => "Dependencies installed!",
        },
        "done" => match lang {
            Language::Portuguese => "🏁 Concluído.",
            Language::Spanish => "🏁 Terminado.",
            Language::Italian => "🏁 Finito.",
            Language::Russian => "🏁 Готово.",
            Language::English => "🏁 Done.",
        },
        "enter_wine_prefix" => match lang {
            Language::Portuguese => "Por favor, insira o caminho do prefixo Wine.",
            Language::Spanish => "Por favor, introduzca la ruta del prefijo Wine.",
            Language::Italian => "Si prega di inserire il percorso del prefisso Wine.",
            Language::Russian => "Пожалуйста, введите путь к префиксу Wine.",
            Language::English => "Please, enter the Wine prefix path.",
        },
        "base_dependencies" => match lang {
            Language::Portuguese => "📦 Instalando dependências base...",
            Language::Spanish => "📦 Instalando dependencias base...",
            Language::Italian => "📦 Installazione dipendenze di base...",
            Language::Russian => "📦 Установка базовых зависимостей...",
            Language::English => "📦 Installing base dependencies...",
        },
        "running_installer" => match lang {
            Language::Portuguese => "📦 Executando instalador da ferramenta...",
            Language::Spanish => "📦 Ejecutando instalador de la herramienta...",
            Language::Italian => "📦 Esecuzione del programma di installazione dello strumento...",
            Language::Russian => "📦 Запуск установщика инструмента...",
            Language::English => "📦 Running tool installer...",
        },
        "setup_finished" => match lang {
            Language::Portuguese => "✅ Configuração da ferramenta concluída.",
            Language::Spanish => "✅ Configuración de la herramienta completada.",
            Language::Italian => "✅ Configurazione dello strumento completata.",
            Language::Russian => "✅ Настройка инструмента завершена.",
            Language::English => "✅ Tool setup finished.",
        },
        "finalizing" => match lang {
            Language::Portuguese => "🏁 Finalizando configuração...",
            Language::Spanish => "🏁 Finalizando configuración...",
            Language::Italian => "🏁 Completamento configurazione...",
            Language::Russian => "🏁 Завершение настройки...",
            Language::English => "🏁 Finalizing setup...",
        },
        "browse_prefix_tooltip" => match lang {
            Language::Portuguese => "Escolher diretório do prefixo Wine",
            Language::Spanish => "Elegir directorio del prefijo Wine",
            Language::Italian => "Scegli la cartella del prefisso Wine",
            Language::Russian => "Выбрать каталог префикса Wine",
            Language::English => "Choose the Wine prefix directory",
        },
        "select_wine_prefix" => match lang {
            Language::Portuguese => "Selecionar prefixo Wine",
            Language::Spanish => "Seleccionar prefijo Wine",
            Language::Italian => "Seleziona prefisso Wine",
            Language::Russian => "Выбрать префикс Wine",
            Language::English => "Select Wine prefix",
        },
        "confirm_close" => match lang {
            Language::Portuguese => "Existem operações em andamento. Tem certeza que deseja fechar?",
            Language::Spanish => "Hay operaciones en curso. ¿Está seguro de que desea cerrar?",
            Language::Italian => "Ci sono operazioni in corso. Sei sicuro di voler chiudere?",
            Language::Russian => "Выполняются операции. Вы уверены, что хотите закрыть?",
            Language::English => "There are operations in progress. Are you sure you want to close?",
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
        Language::Russian => format!("Установить {}", name),
        Language::English => format!("Install {}", name),
    }
}

pub fn t_launch(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Lançar {} 🚀", name),
        Language::Spanish => format!("Lanzar {} 🚀", name),
        Language::Italian => format!("Avvia {} 🚀", name),
        Language::Russian => format!("Запустить {} 🚀", name),
        Language::English => format!("Launch {} 🚀", name),
    }
}

pub fn t_tools_not_found(missing: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("⚠️ Ferramentas não encontradas: {}. Instale antes de usar.", missing),
        Language::Spanish => format!("⚠️ Herramientas no encontradas: {}. Instálelas antes de usar.", missing),
        Language::Italian => format!("⚠️ Strumenti non trovati: {}. Installare prima dell'uso.", missing),
        Language::Russian => format!("⚠️ Инструменты не найдены: {}. Установите их перед использованием.", missing),
        Language::English => format!("⚠️ Tools not found: {}. Install before using.", missing),
    }
}

pub fn t_games_found(count: usize) -> String {
    match get_language() {
        Language::Portuguese => format!("{} jogos encontrados.", count),
        Language::Spanish => format!("{} juegos encontrados.", count),
        Language::Italian => format!("{} giochi trovati.", count),
        Language::Russian => format!("Найдено игр: {}.", count),
        Language::English => format!("{} games found.", count),
    }
}

pub fn t_starting(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Iniciando {}...", name),
        Language::Spanish => format!("Iniciando {}...", name),
        Language::Italian => format!("Avvio di {}...", name),
        Language::Russian => format!("Запуск {}...", name),
        Language::English => format!("Starting {}...", name),
    }
}

pub fn t_started(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("{} iniciado! 🚀", name),
        Language::Spanish => format!("¡{} iniciado! 🚀", name),
        Language::Italian => format!("{} avviato! 🚀", name),
        Language::Russian => format!("{} запущен! 🚀", name),
        Language::English => format!("{} started! 🚀", name),
    }
}

pub fn t_error_launching(err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Erro ao lançar: {}", err),
        Language::Spanish => format!("❌ Error al lanzar: {}", err),
        Language::Italian => format!("❌ Errore durante l'avvio: {}", err),
        Language::Russian => format!("❌ Ошибка при запуске: {}", err),
        Language::English => format!("❌ Error launching: {}", err),
    }
}

pub fn t_prefix_not_found(path: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Prefixo não encontrado: {}", path),
        Language::Spanish => format!("❌ Prefijo no encontrado: {}", path),
        Language::Italian => format!("❌ Prefisso non trovato: {}", path),
        Language::Russian => format!("❌ Префикс не найден: {}", path),
        Language::English => format!("❌ Prefix not found: {}", path),
    }
}

pub fn t_executing(cmd: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Executando {}...", cmd),
        Language::Spanish => format!("Ejecutando {}...", cmd),
        Language::Italian => format!("Esecuzione di {}...", cmd),
        Language::Russian => format!("Выполнение {}...", cmd),
        Language::English => format!("Executing {}...", cmd),
    }
}

pub fn t_command_started(cmd: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Comando {} iniciado.", cmd),
        Language::Spanish => format!("Comando {} iniciado.", cmd),
        Language::Italian => format!("Comando {} avviato.", cmd),
        Language::Russian => format!("Команда {} запущена.", cmd),
        Language::English => format!("Command {} started.", cmd),
    }
}

pub fn t_error(err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("Erro: {}", err),
        Language::Spanish => format!("Error: {}", err),
        Language::Italian => format!("Errore: {}", err),
        Language::Russian => format!("Ошибка: {}", err),
        Language::English => format!("Error: {}", err),
    }
}

// Para wine/mod.rs
pub fn t_downloading(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("📦 Baixando {}...", name),
        Language::Spanish => format!("📦 Descargando {}...", name),
        Language::Italian => format!("📦 Download di {}...", name),
        Language::Russian => format!("📦 Загрузка {}...", name),
        Language::English => format!("📦 Downloading {}...", name),
    }
}

pub fn t_failed_download(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Falha no download de {}", name),
        Language::Spanish => format!("❌ Error al descargar {}", name),
        Language::Italian => format!("❌ Download di {} non riuscito", name),
        Language::Russian => format!("❌ Ошибка загрузки {}", name),
        Language::English => format!("❌ Failed to download {}", name),
    }
}

pub fn t_installing_via_winetricks(name: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("📦 Instalando {} via winetricks...", name),
        Language::Spanish => format!("📦 Instalando {} a través de winetricks...", name),
        Language::Italian => format!("📦 Installazione di {} tramite winetricks...", name),
        Language::Russian => format!("📦 Установка {} через winetricks...", name),
        Language::English => format!("📦 Installing {} via winetricks...", name),
    }
}

pub fn t_failed_install(name: &str, err: &str) -> String {
    match get_language() {
        Language::Portuguese => format!("❌ Falha ao instalar {}: {}", name, err),
        Language::Spanish => format!("❌ Error al instalar {}: {}", name, err),
        Language::Italian => format!("❌ Impossibile installare {}: {}", name, err),
        Language::Russian => format!("❌ Ошибка при установке {}: {}", name, err),
        Language::English => format!("❌ Failed to install {}: {}", name, err),
    }
}
