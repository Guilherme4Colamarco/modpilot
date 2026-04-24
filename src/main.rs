pub mod app;
pub mod games;
pub mod modding_apps;
pub mod wine;
pub mod manifests;

fn main() {
    println!("Iniciando GamePiLot...");
    app::run();
}
