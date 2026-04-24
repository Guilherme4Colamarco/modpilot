use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, RelmWidgetExt};
use relm4::gtk::prelude::*;

pub struct AppModel {
    status_message: String,
    prefix_path: String,
    selected_command: String,
    is_installing: bool,
    detected_games: gtk::StringList,
    install_progress: String,
    manifests: Vec<crate::manifests::GameManifest>,
    available_tools: gtk::StringList,
    current_tools: Vec<crate::manifests::ModTool>,
    selected_tool_deps: Vec<String>,
    selected_tool_download: Option<String>,
    selected_tool_name: String,
    selected_tool_executable: Option<String>,
    tool_is_installed: bool,
    scanned_game_names: Vec<String>,
}

#[derive(Debug)]
pub enum AppMsg {
    ScanGames,
    InstallDependencies,
    PrefixPathChanged(String),
    CommandSelected(u32),
    ExecuteCommand,
    InstallFinished,
    InstallProgress(String),
    GameSelected(u32),
    ToolSelected(u32),
    LaunchTool,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("GamePiLot"),
            set_default_size: (800, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 12,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_halign: gtk::Align::Center,

                    gtk::Spinner {
                        #[watch]
                        set_spinning: model.is_installing,
                        #[watch]
                        set_visible: model.is_installing,
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &model.status_message,
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,

                    gtk::Button {
                        set_label: "Escanear Jogos",
                        connect_clicked => AppMsg::ScanGames,
                    },

                    gtk::DropDown {
                        set_model: Some(&model.detected_games),
                        set_hexpand: true,
                        connect_selected_notify[sender] => move |dropdown| {
                            sender.input(AppMsg::GameSelected(dropdown.selected()));
                        }
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,
                    
                    gtk::Label {
                        set_label: "Dependências (Mods):",
                        set_halign: gtk::Align::Start,
                    },

                    gtk::DropDown {
                        set_model: Some(&model.available_tools),
                        set_hexpand: true,
                        connect_selected_notify[sender] => move |dropdown| {
                            sender.input(AppMsg::ToolSelected(dropdown.selected()));
                        }
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,

                    gtk::Button {
                        #[watch]
                        set_label: &if model.selected_tool_name.is_empty() {
                            "Instalar Ferramenta".to_string()
                        } else {
                            format!("Instalar {}", model.selected_tool_name)
                        },
                        #[watch]
                        set_sensitive: !model.is_installing && !model.selected_tool_name.is_empty(),
                        connect_clicked => AppMsg::InstallDependencies,
                    },

                    gtk::Button {
                        #[watch]
                        set_label: &format!("Lançar {} 🚀", model.selected_tool_name),
                        #[watch]
                        set_visible: !model.selected_tool_name.is_empty(),
                        #[watch]
                        set_sensitive: model.tool_is_installed && !model.is_installing,
                        connect_clicked => AppMsg::LaunchTool,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    #[watch]
                    set_visible: !model.install_progress.is_empty(),

                    gtk::Label {
                        #[watch]
                        set_label: &format!("> {}", model.install_progress),
                        set_halign: gtk::Align::Start,
                    }
                },

                gtk::Separator {
                    set_orientation: gtk::Orientation::Horizontal,
                },

                gtk::Label {
                    set_label: "Ferramentas Wine",
                    set_halign: gtk::Align::Start,
                    add_css_class: "title-4",
                },

                gtk::Entry {
                    set_placeholder_text: Some("Caminho do prefixo Wine (ex: /home/user/.wine)"),
                    connect_changed[sender] => move |entry| {
                        sender.input(AppMsg::PrefixPathChanged(entry.text().to_string()));
                    }
                },

                gtk::DropDown {
                    set_model: Some(&gtk::StringList::new(&["winecfg", "explorer", "winetricks"])),
                    connect_selected_notify[sender] => move |dropdown| {
                        sender.input(AppMsg::CommandSelected(dropdown.selected()));
                    }
                },

                gtk::Button {
                    set_label: "Executar Comando Wine",
                    connect_clicked => AppMsg::ExecuteCommand,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            status_message: "Bem-vindo ao GamePiLot!".to_string(),
            prefix_path: String::new(),
            selected_command: "winecfg".to_string(),
            is_installing: false,
            detected_games: gtk::StringList::new(&["Nenhum jogo escaneado"]),
            install_progress: String::new(),
            manifests: crate::manifests::load_manifests(),
            available_tools: gtk::StringList::new(&["Nenhum jogo selecionado"]),
            current_tools: Vec::new(),
            selected_tool_deps: Vec::new(),
            selected_tool_download: None,
            selected_tool_name: String::new(),
            selected_tool_executable: None,
            tool_is_installed: false,
            scanned_game_names: Vec::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ScanGames => {
                self.status_message = "Escaneando jogos...".to_string();
                let games = crate::games::scan_all_games();
                self.scanned_game_names = games.clone();
                
                self.detected_games.splice(0, self.detected_games.n_items(), &[]);
                if games.is_empty() {
                    self.status_message = "Nenhum jogo encontrado.".to_string();
                    self.detected_games.append("Nenhum jogo encontrado");
                } else {
                    self.status_message = format!("{} jogos encontrados.", games.len());
                    for game in games {
                        self.detected_games.append(&game);
                    }
                }
            }
            AppMsg::GameSelected(index) => {
                self.available_tools.splice(0, self.available_tools.n_items(), &[]);
                self.current_tools.clear();
                self.selected_tool_deps.clear();
                self.selected_tool_download = None;
                self.selected_tool_name.clear();
                self.selected_tool_executable = None;
                self.tool_is_installed = false;
                
                if let Some(game_name) = self.scanned_game_names.get(index as usize) {
                    let mut found = false;
                    for manifest in &self.manifests {
                        let mut matches = game_name.contains(&manifest.name);
                        if let Some(heroic_names) = &manifest.identifiers.heroic_names {
                            for h_name in heroic_names {
                                if game_name.contains(h_name) {
                                    matches = true;
                                }
                            }
                        }
                        
                        if matches {
                            found = true;
                            for (_key, tool) in &manifest.tools {
                                self.current_tools.push(tool.clone());
                                self.available_tools.append(&format!("{} - {}", tool.name, tool.description));
                            }
                            break;
                        }
                    }
                    if !found {
                        self.available_tools.append("Nenhum manifesto de mod encontrado");
                    }
                }
            }
            AppMsg::ToolSelected(index) => {
                if let Some(tool) = self.current_tools.get(index as usize) {
                    self.selected_tool_deps = tool.winetricks.clone();
                    self.selected_tool_download = tool.download_url.clone();
                    self.selected_tool_name = tool.name.clone();
                    self.selected_tool_executable = tool.executable_path.clone();
                    
                    // Simples verificação: se tivermos o prefix_path, checamos se existe
                    // Caso contrário (como ainda não automatizamos a busca do prefixo), assumimos false
                    // ou testamos no caminho absoluto
                    if let Some(exe) = &self.selected_tool_executable {
                        let path = std::path::Path::new(&self.prefix_path).join(exe);
                        self.tool_is_installed = path.exists();
                    } else {
                        self.tool_is_installed = false;
                    }
                }
            }
            AppMsg::LaunchTool => {
                if let Some(exe) = &self.selected_tool_executable {
                    self.status_message = format!("Iniciando {}...", self.selected_tool_name);
                    let full_path = std::path::Path::new(&self.prefix_path).join(exe);
                    let path_str = full_path.to_string_lossy().to_string();
                    let _ = crate::wine::execute_wine_command(&self.prefix_path, &path_str); 
                }
            }
            AppMsg::InstallDependencies => {
                self.is_installing = true;
                self.status_message = "Instalando dependências...".to_string();
                self.install_progress = "Preparando...".to_string();
                
                let sender = _sender.clone();
                let progress_sender = _sender.clone();
                let deps = self.selected_tool_deps.clone();
                let download = self.selected_tool_download.clone();
                let prefix = if self.prefix_path.is_empty() {
                    "default_prefix".to_string()
                } else {
                    self.prefix_path.clone()
                };
                
                std::thread::spawn(move || {
                    crate::wine::install_dependencies(&prefix, deps, download, move |msg| {
                        progress_sender.input(AppMsg::InstallProgress(msg));
                    });
                    sender.input(AppMsg::InstallFinished);
                });
            }
            AppMsg::InstallProgress(msg) => {
                self.install_progress = msg;
            }
            AppMsg::InstallFinished => {
                self.is_installing = false;
                self.status_message = "Dependências instaladas!".to_string();
                self.install_progress = "Concluído.".to_string();
            }
            AppMsg::PrefixPathChanged(path) => {
                self.prefix_path = path;
                // Reavaliar se a ferramenta tá instalada quando o prefixo muda
                if let Some(exe) = &self.selected_tool_executable {
                    let full_path = std::path::Path::new(&self.prefix_path).join(exe);
                    self.tool_is_installed = full_path.exists();
                }
            }
            AppMsg::CommandSelected(index) => {
                let commands = ["winecfg", "explorer", "winetricks"];
                if let Some(cmd) = commands.get(index as usize) {
                    self.selected_command = cmd.to_string();
                }
            }
            AppMsg::ExecuteCommand => {
                if self.prefix_path.is_empty() {
                    self.status_message = "Por favor, insira o caminho do prefixo Wine.".to_string();
                    return;
                }
                self.status_message = format!("Executando {}...", self.selected_command);
                match crate::wine::execute_wine_command(&self.prefix_path, &self.selected_command) {
                    Ok(_) => self.status_message = format!("Comando {} iniciado.", self.selected_command),
                    Err(e) => self.status_message = format!("Erro: {}", e),
                }
            }
        }
    }
}

pub fn run() {
    let app = RelmApp::new("org.gamepilot.App");
    app.run::<AppModel>(());
}
