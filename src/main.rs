pub mod app;
pub mod games;
pub mod i18n;
pub mod manifests;
pub mod wine;

fn main() {
    println!("Iniciando GamePiLot...");
    app::run();
}
