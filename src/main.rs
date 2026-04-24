mod menu;

use menu::{MenuAction, read_choice, show_main_menu};

fn main() {
    loop {
        show_main_menu();

        match read_choice() {
            MenuAction::ListGames => println!("[em breve] Listagem de jogos."),
            MenuAction::ManageMods => println!("[em breve] Gerenciamento de mods."),
            MenuAction::InstallDeps => println!("[em breve] Instalação de dependências."),
            MenuAction::Quit => {
                println!("Até logo!");
                break;
            }
        }
    }
}
