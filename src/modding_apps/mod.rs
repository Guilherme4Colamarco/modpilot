pub trait ModdingApp {
    fn name(&self) -> &str;
    fn install(&self, prefix: &str) -> anyhow::Result<()>;
}

pub struct OpenIV;

impl ModdingApp for OpenIV {
    fn name(&self) -> &str {
        "OpenIV"
    }

    fn install(&self, _prefix: &str) -> anyhow::Result<()> {
        println!("Instalando OpenIV (Simulado)...");
        Ok(())
    }
}
