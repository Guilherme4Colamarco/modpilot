use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, ComboBoxText, Entry,
    FileChooserAction, FileChooserDialog, Label, Orientation, ResponseType,
    ScrolledWindow, Separator, Spinner, TextBuffer, TextView, Align, WrapMode, Adjustment,
    MessageDialog, DialogFlags, MessageType, ButtonsType
};
use std::cell::RefCell;
use std::rc::Rc;
use glib::clone;

#[derive(Clone)]
struct AppWidgets {
    spinner: Spinner,
    status_label: Label,
    games_combo: ComboBoxText,
    tools_combo: ComboBoxText,
    install_button: Button,
    launch_button: Button,
    log_scroll: ScrolledWindow,
    log_buffer: TextBuffer,
    prefix_entry: Entry,
    cmds_combo: ComboBoxText,
    window: ApplicationWindow,
    browse_button: Button,
    scan_button: Button,
    exec_button: Button,
}

struct AppState {
    status_message: String,
    prefix_path: String,
    selected_command: String,
    is_installing: bool,
    manifests: Vec<crate::manifests::GameManifest>,
    current_tools: Vec<crate::manifests::ModTool>,
    selected_tool_deps: Vec<String>,
    selected_tool_download: Option<String>,
    selected_tool_name: String,
    selected_tool_executable: Option<String>,
    tool_is_installed: bool,
    scanned_games: Vec<(String, Option<String>, Option<u32>)>,
    install_log: Vec<String>,
    install_log_visible: bool,
}

enum AppMsg {
    ScanGames,
    GameSelected(i32),
    ToolSelected(i32),
    InstallDependencies,
    LaunchTool,
    PrefixPathChanged(String),
    CommandSelected(i32),
    ExecuteCommand,
    InstallProgress(String),
    InstallFinished,
    BrowsePrefix,
}

pub fn run() {
    let app = Application::builder()
        .application_id("org.gamepilot.App")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GamePiLot")
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);

    let status_box = GtkBox::new(Orientation::Horizontal, 8);
    status_box.set_halign(Align::Center);
    let spinner = Spinner::new();
    let status_label = Label::new(None);
    status_box.pack_start(&spinner, false, false, 0);
    status_box.pack_start(&status_label, false, false, 0);

    let games_box = GtkBox::new(Orientation::Horizontal, 12);
    let scan_button = Button::with_label(&crate::i18n::t("scan_games"));
    let games_combo = ComboBoxText::new();
    games_combo.set_hexpand(true);
    games_box.pack_start(&scan_button, false, false, 0);
    games_box.pack_start(&games_combo, true, true, 0);

    let tools_box = GtkBox::new(Orientation::Horizontal, 12);
    let tools_label = Label::new(Some(&crate::i18n::t("dependencies_mods")));
    tools_label.set_halign(Align::Start);
    let tools_combo = ComboBoxText::new();
    tools_combo.set_hexpand(true);
    tools_box.pack_start(&tools_label, false, false, 0);
    tools_box.pack_start(&tools_combo, true, true, 0);

    let actions_box = GtkBox::new(Orientation::Horizontal, 12);
    let install_button = Button::with_label(&crate::i18n::t("install_tool"));
    let launch_button = Button::with_label("");
    actions_box.pack_start(&install_button, false, false, 0);
    actions_box.pack_start(&launch_button, false, false, 0);

    let log_scroll = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    log_scroll.set_vexpand(true);
    log_scroll.set_min_content_height(150);
    let log_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    let log_view = TextView::with_buffer(&log_buffer);
    log_view.set_editable(false);
    log_view.set_monospace(true);
    log_view.set_wrap_mode(WrapMode::WordChar);
    log_view.set_margin_start(8);
    log_view.set_margin_end(8);
    log_view.set_margin_top(8);
    log_view.set_margin_bottom(8);
    log_scroll.add(&log_view);

    let separator = Separator::new(Orientation::Horizontal);

    let wine_title = Label::new(Some(&crate::i18n::t("wine_tools")));
    wine_title.set_halign(Align::Start);
    wine_title.style_context().add_class("title-4");

    let prefix_box = GtkBox::new(Orientation::Horizontal, 8);
    let prefix_entry = Entry::new();
    prefix_entry.set_placeholder_text(Some(&crate::i18n::t("wine_prefix_placeholder")));
    prefix_entry.set_hexpand(true);
    let browse_button = Button::with_label("📂");
    browse_button.set_tooltip_text(Some(&crate::i18n::t("browse_prefix_tooltip")));
    prefix_box.pack_start(&prefix_entry, true, true, 0);
    prefix_box.pack_start(&browse_button, false, false, 0);

    let cmds_combo = ComboBoxText::new();
    cmds_combo.append_text("winecfg");
    cmds_combo.append_text("explorer");
    cmds_combo.append_text("winetricks");

    let exec_button = Button::with_label(&crate::i18n::t("execute_wine_command"));

    main_box.pack_start(&status_box, false, false, 0);
    main_box.pack_start(&games_box, false, false, 0);
    main_box.pack_start(&tools_box, false, false, 0);
    main_box.pack_start(&actions_box, false, false, 0);
    main_box.pack_start(&log_scroll, true, true, 0);
    main_box.pack_start(&separator, false, false, 0);
    main_box.pack_start(&wine_title, false, false, 0);
    main_box.pack_start(&prefix_box, false, false, 0);
    main_box.pack_start(&cmds_combo, false, false, 0);
    main_box.pack_start(&exec_button, false, false, 0);

    window.add(&main_box);

    let widgets = AppWidgets {
        spinner,
        status_label,
        games_combo,
        tools_combo,
        install_button,
        launch_button,
        log_scroll,
        log_buffer,
        prefix_entry,
        cmds_combo,
        window: window.clone(),
        browse_button,
        scan_button,
        exec_button,
    };

    let missing = crate::wine::check_prerequisites();
    let initial_status = if missing.is_empty() {
        crate::i18n::t("welcome")
    } else {
        crate::i18n::t_tools_not_found(&missing.join(", "))
    };

    let state = AppState {
        status_message: initial_status,
        prefix_path: String::new(),
        selected_command: "winecfg".to_string(),
        is_installing: false,
        manifests: crate::manifests::load_manifests(),
        current_tools: Vec::new(),
        selected_tool_deps: Vec::new(),
        selected_tool_download: None,
        selected_tool_name: String::new(),
        selected_tool_executable: None,
        tool_is_installed: false,
        scanned_games: Vec::new(),
        install_log: Vec::new(),
        install_log_visible: false,
    };

    let state = Rc::new(RefCell::new(state));
    let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

    setup_signals(&widgets, &sender);
    
    widgets.games_combo.append_text(&crate::i18n::t("no_games_scanned"));
    widgets.games_combo.set_active(Some(0));
    widgets.tools_combo.append_text(&crate::i18n::t("no_game_selected"));
    widgets.tools_combo.set_active(Some(0));
    widgets.cmds_combo.set_active(Some(0));
    
    update_ui(&state.borrow(), &widgets);
    window.show_all();
    widgets.log_scroll.hide();

    let sender_clone = sender.clone();
    receiver.attach(None, clone!(@strong state, @strong widgets => move |msg| {
        handle_msg(msg, &state, &widgets, &sender_clone);
        glib::ControlFlow::Continue
    }));
}

fn setup_signals(widgets: &AppWidgets, sender: &glib::Sender<AppMsg>) {
    widgets.scan_button.connect_clicked(clone!(@strong sender => move |_| {
        let _ = sender.send(AppMsg::ScanGames);
    }));

    widgets.games_combo.connect_changed(clone!(@strong sender => move |combo| {
        if let Some(idx) = combo.active() {
            let _ = sender.send(AppMsg::GameSelected(idx as i32));
        }
    }));

    widgets.tools_combo.connect_changed(clone!(@strong sender => move |combo| {
        if let Some(idx) = combo.active() {
            let _ = sender.send(AppMsg::ToolSelected(idx as i32));
        }
    }));

    widgets.install_button.connect_clicked(clone!(@strong sender => move |_| {
        let _ = sender.send(AppMsg::InstallDependencies);
    }));

    widgets.launch_button.connect_clicked(clone!(@strong sender => move |_| {
        let _ = sender.send(AppMsg::LaunchTool);
    }));

    widgets.prefix_entry.connect_changed(clone!(@strong sender => move |entry| {
        let _ = sender.send(AppMsg::PrefixPathChanged(entry.text().to_string()));
    }));

    widgets.browse_button.connect_clicked(clone!(@strong sender => move |_| {
        let _ = sender.send(AppMsg::BrowsePrefix);
    }));

    widgets.cmds_combo.connect_changed(clone!(@strong sender => move |combo| {
        if let Some(idx) = combo.active() {
            let _ = sender.send(AppMsg::CommandSelected(idx as i32));
        }
    }));

    widgets.exec_button.connect_clicked(clone!(@strong sender => move |_| {
        let _ = sender.send(AppMsg::ExecuteCommand);
    }));
}

fn update_ui(state: &AppState, widgets: &AppWidgets) {
    if state.is_installing {
        widgets.spinner.start();
        widgets.spinner.show();
    } else {
        widgets.spinner.stop();
        widgets.spinner.hide();
    }

    widgets.status_label.set_label(&state.status_message);

    if state.selected_tool_name.is_empty() {
        widgets.install_button.set_label(&crate::i18n::t("install_tool"));
    } else {
        widgets.install_button.set_label(&crate::i18n::t_install(&state.selected_tool_name));
    }
    widgets.install_button.set_sensitive(!state.is_installing && !state.selected_tool_name.is_empty());

    widgets.launch_button.set_label(&crate::i18n::t_launch(&state.selected_tool_name));
    widgets.launch_button.set_visible(!state.selected_tool_name.is_empty());
    widgets.launch_button.set_sensitive(state.tool_is_installed && !state.is_installing);

    if state.install_log_visible {
        widgets.log_scroll.show();
    } else {
        widgets.log_scroll.hide();
    }
    
    widgets.log_buffer.set_text(&state.install_log.join("\n"));
    
    if widgets.prefix_entry.text().as_str() != state.prefix_path {
        widgets.prefix_entry.set_text(&state.prefix_path);
    }
}

fn handle_msg(msg: AppMsg, state_rc: &Rc<RefCell<AppState>>, widgets: &AppWidgets, sender: &glib::Sender<AppMsg>) {
    let mut state = state_rc.borrow_mut();
    match msg {
        AppMsg::ScanGames => {
            let games = crate::games::scan_all_games();
            widgets.games_combo.remove_all();
            state.scanned_games = games.clone();
            
            if games.is_empty() {
                state.status_message = crate::i18n::t("no_games_found");
                widgets.games_combo.append_text("No games found"); 
                widgets.games_combo.set_active(Some(0));
            } else {
                for (game_name, _prefix, _app_id) in &games {
                    widgets.games_combo.append_text(game_name);
                }
                state.status_message = crate::i18n::t_games_found(games.len());
            }
        }
        AppMsg::GameSelected(index) => {
            widgets.tools_combo.remove_all();
            state.current_tools.clear();
            state.selected_tool_deps.clear();
            state.selected_tool_download = None;
            state.selected_tool_name.clear();
            state.selected_tool_executable = None;
            state.tool_is_installed = false;
            
            let game_info = state.scanned_games.get(index as usize).cloned();
            if let Some((game_name, prefix_opt, game_app_id)) = game_info {
                if let Some(prefix) = prefix_opt {
                    state.prefix_path = prefix;
                } else {
                    state.prefix_path.clear();
                }

                const SCORE_APP_ID: usize = usize::MAX;
                let mut best_score = 0usize;
                let mut best_manifest = None;

                for manifest in &state.manifests {
                    let mut score = 0usize;

                    if let (Some(mid), Some(gid)) = (&manifest.identifiers.steam_app_id, game_app_id) {
                        if mid == &gid {
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

                    if score > 0 && (best_manifest.is_none() || score > best_score) {
                        best_score = score;
                        best_manifest = Some(manifest.clone());
                    }
                }

                if let Some(manifest) = best_manifest {
                    for tool in manifest.tools.values() {
                        state.current_tools.push(tool.clone());
                        widgets.tools_combo.append_text(&format!("{} - {}", tool.name, tool.description));
                    }
                } else {
                    widgets.tools_combo.append_text(&crate::i18n::t("no_mod_manifest_found"));
                }
            }
        }
        AppMsg::ToolSelected(index) => {
            let tool_opt = state.current_tools.get(index as usize).cloned();
            if let Some(tool) = tool_opt {
                state.selected_tool_deps = tool.winetricks;
                state.selected_tool_download = tool.download_url;
                state.selected_tool_name = tool.name;
                state.selected_tool_executable = tool.executable_path;
                
                if let Some(exe) = &state.selected_tool_executable {
                    let path = std::path::Path::new(&state.prefix_path).join(exe);
                    state.tool_is_installed = path.exists();
                } else {
                    state.tool_is_installed = false;
                }
            }
        }
        AppMsg::LaunchTool => {
            let exe_opt = state.selected_tool_executable.clone();
            if let Some(exe) = exe_opt {
                state.status_message = crate::i18n::t_starting(&state.selected_tool_name);
                let full_path = std::path::Path::new(&state.prefix_path).join(exe);
                let path_str = full_path.to_string_lossy().to_string();
                match crate::wine::execute_wine_command(&state.prefix_path, &path_str) {
                    Ok(_) => state.status_message = crate::i18n::t_started(&state.selected_tool_name),
                    Err(e) => state.status_message = crate::i18n::t_error_launching(&e.to_string()),
                }
            }
        }
        AppMsg::InstallDependencies => {
            if state.prefix_path.is_empty() {
                let msg = crate::i18n::t("no_wine_prefix");
                state.status_message = msg.clone();
                state.install_log.clear();
                state.install_log.push(msg);
                state.install_log_visible = true;
            } else if !std::path::Path::new(&state.prefix_path).exists() {
                let msg = crate::i18n::t_prefix_not_found(&state.prefix_path);
                state.status_message = msg.clone();
                state.install_log.clear();
                state.install_log.push(msg);
                state.install_log_visible = true;
            } else {
                state.is_installing = true;
                state.status_message = crate::i18n::t("installing_dependencies");
                state.install_log.clear();
                state.install_log.push(crate::i18n::t("preparing"));
                state.install_log_visible = true;
                
                let s_prog = sender.clone();
                let s_done = sender.clone();
                let deps = state.selected_tool_deps.clone();
                let download = state.selected_tool_download.clone();
                let prefix = state.prefix_path.clone();
                
                std::thread::spawn(move || {
                    crate::wine::install_dependencies(&prefix, deps, download, move |msg| {
                        let _ = s_prog.send(AppMsg::InstallProgress(msg));
                    });
                    let _ = s_done.send(AppMsg::InstallFinished);
                });
            }
        }
        AppMsg::InstallProgress(msg) => {
            if msg.starts_with("❌") || msg.starts_with("⚠️") {
                let dialog = MessageDialog::new(
                    Some(&widgets.window),
                    DialogFlags::MODAL,
                    MessageType::Error,
                    ButtonsType::Ok,
                    &msg,
                );
                dialog.connect_response(|dialog, _| {
                    dialog.close();
                });
                dialog.show_all();
            }
            state.install_log.push(msg);
        }
        AppMsg::InstallFinished => {
            state.is_installing = false;
            state.status_message = crate::i18n::t("dependencies_installed");
            state.install_log.push(crate::i18n::t("done"));

            if let Some(exe) = &state.selected_tool_executable {
                let full_path = std::path::Path::new(&state.prefix_path).join(exe);
                state.tool_is_installed = full_path.exists();
            }
        }
        AppMsg::PrefixPathChanged(path) => {
            state.prefix_path = path;
            
            if let Some(exe) = &state.selected_tool_executable {
                let full_path = std::path::Path::new(&state.prefix_path).join(exe);
                state.tool_is_installed = full_path.exists();
            }
        }
        AppMsg::BrowsePrefix => {
            let dialog = FileChooserDialog::new(
                Some(&crate::i18n::t("select_wine_prefix")),
                Some(&widgets.window),
                FileChooserAction::SelectFolder,
            );
            dialog.add_buttons(&[
                ("Cancel", ResponseType::Cancel),
                ("Open", ResponseType::Accept),
            ]);
            let s = sender.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            let _ = s.send(AppMsg::PrefixPathChanged(path.to_string_lossy().to_string()));
                        }
                    }
                }
                dialog.close();
            });
            dialog.show_all();
        }
        AppMsg::CommandSelected(index) => {
            let commands = ["winecfg", "explorer", "winetricks"];
            if let Some(cmd) = commands.get(index as usize) {
                state.selected_command = cmd.to_string();
            }
        }
        AppMsg::ExecuteCommand => {
            if state.prefix_path.is_empty() {
                state.status_message = crate::i18n::t("enter_wine_prefix");
            } else {
                state.status_message = crate::i18n::t_executing(&state.selected_command);
                match crate::wine::execute_wine_command(&state.prefix_path, &state.selected_command) {
                    Ok(_) => state.status_message = crate::i18n::t_command_started(&state.selected_command),
                    Err(e) => state.status_message = crate::i18n::t_error(&e.to_string()),
                }
            }
        }
    }
    update_ui(&state, widgets);
}
