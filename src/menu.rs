use std::io::{self, Write};

pub enum MenuAction {
    ListGames,
    ManageMods,
    InstallDeps,
    Quit,
}

pub fn show_main_menu() {
    println!("\n=== ModPilot ===");
    println!("[1] Listar jogos detectados");
    println!("[2] Gerenciar mods");
    println!("[3] Instalar dependências");
    println!("[0] Sair");
}

pub fn read_choice() -> MenuAction {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return MenuAction::ListGames,
            "2" => return MenuAction::ManageMods,
            "3" => return MenuAction::InstallDeps,
            "0" => return MenuAction::Quit,
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}
