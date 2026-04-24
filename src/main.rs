pub mod app;
pub mod games;
pub mod wine;
pub mod manifests;
pub mod i18n;

fn main() {
    println!("Iniciando GamePiLot...");
    app::run();
}
