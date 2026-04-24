use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, RelmWidgetExt};
use relm4::gtk::prelude::*;

pub struct AppModel {
    status_message: String,
    prefix_path: String,
    selected_command: String,
    is_installing: bool,
    detected_games: gtk::StringList,
    install_log: Vec<String>,
    manifests: Vec<crate::manifests::GameManifest>,
    available_tools: gtk::StringList,
    current_tools: Vec<crate::manifests::ModTool>,
    selected_tool_deps: Vec<String>,
    selected_tool_download: Option<String>,
    selected_tool_name: String,
    selected_tool_executable: Option<String>,
    tool_is_installed: bool,
    scanned_games: Vec<(String, Option<String>, Option<u32>)>,
    install_log_buffer: gtk::TextBuffer,
    install_log_visible: bool,
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
                        set_label: &crate::i18n::t("scan_games"),
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
                        set_label: &crate::i18n::t("dependencies_mods"),
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
                            crate::i18n::t("install_tool")
                        } else {
                            crate::i18n::t_install(&model.selected_tool_name)
                        },
                        #[watch]
                        set_sensitive: !model.is_installing && !model.selected_tool_name.is_empty(),
                        connect_clicked => AppMsg::InstallDependencies,
                    },

                    gtk::Button {
                        #[watch]
                        set_label: &crate::i18n::t_launch(&model.selected_tool_name),
                        #[watch]
                        set_visible: !model.selected_tool_name.is_empty(),
                        #[watch]
                        set_sensitive: model.tool_is_installed && !model.is_installing,
                        connect_clicked => AppMsg::LaunchTool,
                    }
                },

                // Log acumulativo com scroll
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_min_content_height: 150,
                    #[watch]
                    set_visible: model.install_log_visible,

                    gtk::TextView {
                        set_editable: false,
                        set_monospace: true,
                        set_wrap_mode: gtk::WrapMode::WordChar,
                        set_top_margin: 8,
                        set_bottom_margin: 8,
                        set_left_margin: 8,
                        set_right_margin: 8,
                        set_buffer: Some(&model.install_log_buffer),
                    }
                },

                gtk::Separator {
                    set_orientation: gtk::Orientation::Horizontal,
                },

                gtk::Label {
                    set_label: &crate::i18n::t("wine_tools"),
                    set_halign: gtk::Align::Start,
                    add_css_class: "title-4",
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    gtk::Entry {
                        set_placeholder_text: Some(&crate::i18n::t("wine_prefix_placeholder")),
                        set_hexpand: true,
                        #[watch]
                        set_text: &model.prefix_path,
                        connect_changed[sender] => move |entry| {
                            sender.input(AppMsg::PrefixPathChanged(entry.text().to_string()));
                        }
                    },

                    gtk::Button {
                        set_label: "📂",
                        set_tooltip_text: Some(&crate::i18n::t("browse_prefix_tooltip")),
                        connect_clicked[sender] => move |btn| {
                            let dialog = gtk::FileDialog::new();
                            dialog.set_title(&crate::i18n::t("select_wine_prefix"));
                            let parent = btn.root().and_downcast::<gtk::Window>();
                            let sender = sender.clone();
                            dialog.select_folder(
                                parent.as_ref(),
                                None::<&relm4::gtk::gio::Cancellable>,
                                move |result| {
                                    if let Ok(file) = result {
                                        if let Some(path) = file.path() {
                                            sender.input(AppMsg::PrefixPathChanged(
                                                path.to_string_lossy().to_string(),
                                            ));
                                        }
                                    }
                                },
                            );
                        }
                    },
                },

                gtk::DropDown {
                    set_model: Some(&gtk::StringList::new(&["winecfg", "explorer", "winetricks"])),
                    connect_selected_notify[sender] => move |dropdown| {
                        sender.input(AppMsg::CommandSelected(dropdown.selected()));
                    }
                },

                gtk::Button {
                    set_label: &crate::i18n::t("execute_wine_command"),
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
        // Checar pré-requisitos ao iniciar
        let missing = crate::wine::check_prerequisites();
        let initial_status = if missing.is_empty() {
            crate::i18n::t("welcome")
        } else {
            crate::i18n::t_tools_not_found(&missing.join(", "))
        };

        let model = AppModel {
            status_message: initial_status,
            prefix_path: String::new(),
            selected_command: "winecfg".to_string(),
            is_installing: false,
            detected_games: gtk::StringList::new(&[&crate::i18n::t("no_games_scanned")]),
            install_log: Vec::new(),
            install_log_buffer: gtk::TextBuffer::new(None::<&gtk::TextTagTable>),
            install_log_visible: false,
            manifests: crate::manifests::load_manifests(),
            available_tools: gtk::StringList::new(&[&crate::i18n::t("no_game_selected")]),
            current_tools: Vec::new(),
            selected_tool_deps: Vec::new(),
            selected_tool_download: None,
            selected_tool_name: String::new(),
            selected_tool_executable: None,
            tool_is_installed: false,
            scanned_games: Vec::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ScanGames => {
                let games = crate::games::scan_all_games();
                self.detected_games.splice(0, self.detected_games.n_items(), &[]);
                self.scanned_games = games.clone();
                
                if games.is_empty() {
                    self.status_message = crate::i18n::t("no_games_found");
                    self.detected_games.append(&crate::i18n::t("no_games_found_short")); // You can map "no_games_found_short" in i18n
                } else {
                    for (game_name, _prefix, _app_id) in &games {
                        self.detected_games.append(game_name);
                    }
                    self.status_message = crate::i18n::t_games_found(games.len());
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
                
                if let Some((game_name, prefix_opt, game_app_id)) = self.scanned_games.get(index as usize) {

                    // Auto-preenche o prefix_path na interface
                    if let Some(prefix) = prefix_opt {
                        self.prefix_path = prefix.clone();
                    } else {
                        self.prefix_path.clear();
                    }

                    // Seleciona o manifest com a melhor pontuação de match.
                    // steam_app_id exato = prioridade máxima;
                    // match por substring usa o comprimento do nome para
                    // evitar que "Dishonored" ganhe de "Dishonored 2".
                    const SCORE_APP_ID: usize = usize::MAX;
                    let mut best: Option<(&crate::manifests::GameManifest, usize)> = None;

                    for manifest in &self.manifests {
                        let mut score = 0usize;

                        if let (Some(mid), Some(gid)) =
                            (&manifest.identifiers.steam_app_id, game_app_id)
                        {
                            if mid == gid {
                                score = SCORE_APP_ID;
                            }
                        }

                        if score < SCORE_APP_ID {
                            if let Some(heroic_names) = &manifest.identifiers.heroic_names {
                                for h in heroic_names {
                                    if game_name.contains(h) && h.len() > score {
                                        score = h.len();
                                    }
                                }
                            }
                            if game_name.contains(&manifest.name) && manifest.name.len() > score {
                                score = manifest.name.len();
                            }
                        }

                        if score > 0 && best.is_none_or(|(_, s)| score > s) {
                            best = Some((manifest, score));
                        }
                    }

                    if let Some((manifest, _)) = best {
                        for tool in manifest.tools.values() {
                            self.current_tools.push(tool.clone());
                            self.available_tools
                                .append(&format!("{} - {}", tool.name, tool.description));
                        }
                    } else {
                        self.available_tools
                            .append(&crate::i18n::t("no_mod_manifest_found"));
                    }
                }
            }
            AppMsg::ToolSelected(index) => {
                if let Some(tool) = self.current_tools.get(index as usize) {
                    self.selected_tool_deps = tool.winetricks.clone();
                    self.selected_tool_download = tool.download_url.clone();
                    self.selected_tool_name = tool.name.clone();
                    self.selected_tool_executable = tool.executable_path.clone();
                    
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
                    self.status_message = crate::i18n::t_starting(&self.selected_tool_name);
                    let full_path = std::path::Path::new(&self.prefix_path).join(exe);
                    let path_str = full_path.to_string_lossy().to_string();
                    match crate::wine::execute_wine_command(&self.prefix_path, &path_str) {
                        Ok(_) => self.status_message = crate::i18n::t_started(&self.selected_tool_name),
                        Err(e) => self.status_message = crate::i18n::t_error_launching(&e.to_string()),
                    }
                }
            }
            AppMsg::InstallDependencies => {
                // Bloquear instalação se não tem prefixo válido
                if self.prefix_path.is_empty() {
                    let msg = crate::i18n::t("no_wine_prefix");
                    self.status_message = msg.clone();
                    self.install_log.clear();
                    self.install_log.push(msg);
                    self.install_log_buffer.set_text(&self.install_log.join("\n"));
                    self.install_log_visible = true;
                    return;
                }

                // Verificar se o prefixo existe no disco
                if !std::path::Path::new(&self.prefix_path).exists() {
                    let msg = crate::i18n::t_prefix_not_found(&self.prefix_path);
                    self.status_message = msg.clone();
                    self.install_log.clear();
                    self.install_log.push(msg);
                    self.install_log_buffer.set_text(&self.install_log.join("\n"));
                    self.install_log_visible = true;
                    return;
                }

                self.is_installing = true;
                self.status_message = crate::i18n::t("installing_dependencies");
                self.install_log.clear();
                self.install_log.push(crate::i18n::t("preparing"));
                self.install_log_buffer.set_text(&self.install_log.join("\n"));
                self.install_log_visible = true;
                
                let progress_sender = sender.clone();
                let done_sender = sender.clone();
                let deps = self.selected_tool_deps.clone();
                let download = self.selected_tool_download.clone();
                let prefix = self.prefix_path.clone();
                
                std::thread::spawn(move || {
                    crate::wine::install_dependencies(&prefix, deps, download, move |msg| {
                        progress_sender.input(AppMsg::InstallProgress(msg));
                    });
                    done_sender.input(AppMsg::InstallFinished);
                });
            }

            AppMsg::InstallProgress(msg) => {
                self.install_log.push(msg);
                self.install_log_buffer.set_text(&self.install_log.join("\n"));
            }
            AppMsg::InstallFinished => {
                self.is_installing = false;
                self.status_message = crate::i18n::t("dependencies_installed");
                self.install_log.push(crate::i18n::t("done"));
                self.install_log_buffer.set_text(&self.install_log.join("\n"));

                // Reavaliar se a ferramenta foi instalada
                if let Some(exe) = &self.selected_tool_executable {
                    let full_path = std::path::Path::new(&self.prefix_path).join(exe);
                    self.tool_is_installed = full_path.exists();
                }
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
                    self.status_message = crate::i18n::t("enter_wine_prefix");
                    return;
                }
                self.status_message = crate::i18n::t_executing(&self.selected_command);
                match crate::wine::execute_wine_command(&self.prefix_path, &self.selected_command) {
                    Ok(_) => self.status_message = crate::i18n::t_command_started(&self.selected_command),
                    Err(e) => self.status_message = crate::i18n::t_error(&e.to_string()),
                }
            }
        }
    }
}

pub fn run() {
    let app = RelmApp::new("org.gamepilot.App");
    app.run::<AppModel>(());
}
