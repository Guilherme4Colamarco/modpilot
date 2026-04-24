use std::process::{Command, Stdio};

/// Filtra ruído do Wine (fixme:, warn:, trace:, etc.) do stderr e retorna
/// apenas linhas que representam erros reais. Retorna None se tudo for ruído.
fn extract_real_error(stderr: &str) -> Option<String> {
    let noise_prefixes = ["fixme:", "warn:", "trace:", "info:", "err:fixme", "winediag:"];
    let real_errors: Vec<&str> = stderr
        .lines()
        .filter(|l| {
            let lower = l.trim().to_lowercase();
            !noise_prefixes.iter().any(|p| lower.contains(p))
                && !l.trim().is_empty()
        })
        .collect();
    if real_errors.is_empty() {
        None
    } else {
        Some(real_errors.last().unwrap_or(&"erro desconhecido").to_string())
    }
}
use std::io::{BufRead, BufReader};

/// Verifica se as ferramentas necessárias (wine, winetricks) estão no PATH.
/// Retorna uma lista das que estão faltando.
pub fn check_prerequisites() -> Vec<String> {
    let mut missing = Vec::new();
    for tool in &["wine", "winetricks"] {
        let found = Command::new("which")
            .arg(tool)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !found {
            missing.push(tool.to_string());
        }
    }
    missing
}

/// Instala dependências winetricks reais e opcionalmente baixa/executa uma ferramenta.
/// O `progress_callback` é chamado para cada etapa para atualizar a UI.
pub fn install_dependencies<F>(prefix: &str, dependencies: Vec<String>, download_url: Option<String>, progress_callback: F)
where
    F: Fn(String),
{
    progress_callback(format!("Preparando o prefixo: {}", prefix));

    // Verificar pré-requisitos
    let missing = check_prerequisites();
    if !missing.is_empty() {
        progress_callback(format!("❌ Ferramentas não encontradas no PATH: {}", missing.join(", ")));
        progress_callback("Instale com: sudo pacman -S wine winetricks (ou equivalente)".to_string());
        return;
    }

    if dependencies.is_empty() {
        progress_callback("Nenhuma dependência do Winetricks necessária.".to_string());
    }

    for dep in &dependencies {
        progress_callback(crate::i18n::t_installing_via_winetricks(dep));

        let result = Command::new("winetricks")
            .env("WINEPREFIX", prefix)
            .arg("-q") // modo silencioso
            .arg(dep)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    progress_callback(format!("✅ {} instalado com sucesso.", dep));
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let err_msg = extract_real_error(&stderr)
                        .unwrap_or_else(|| "erro desconhecido".to_string());
                    progress_callback(crate::i18n::t_failed_install(dep, &err_msg));
                }
            }
            Err(e) => {
                progress_callback(format!("❌ Erro ao executar winetricks: {}", e));
                return;
            }
        }
    }

    if let Some(url) = download_url {
        progress_callback(crate::i18n::t_downloading(&url));

        // Baixar o arquivo para um diretório temporário dentro do prefixo
        let download_dir = format!("{}/drive_c/gamepilot_downloads", prefix);
        let _ = std::fs::create_dir_all(&download_dir);

        let filename = url.rsplit('/').next().unwrap_or("installer.exe");
        let dest_path = format!("{}/{}", download_dir, filename);

        let download_result = Command::new("wget")
            .args(["-q", "--show-progress", "-O", &dest_path, &url])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match download_result {
            Ok(output) if output.status.success() => {
                progress_callback(format!("✅ Download concluído: {}", filename));

                // Executar o instalador no prefixo Wine
                progress_callback(crate::i18n::t("running_installer"));
                let install_result = Command::new("wine")
                    .env("WINEPREFIX", prefix)
                    .arg(&dest_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                match install_result {
                    Ok(output) if output.status.success() => {
                        progress_callback(crate::i18n::t("setup_finished"));
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if let Some(err_msg) = extract_real_error(&stderr) {
                            progress_callback(format!("⚠️  Instalador retornou erro: {}", err_msg));
                        } else {
                            // Só ruído do Wine — tratar como sucesso
                            progress_callback(crate::i18n::t("setup_finished"));
                        }
                    }
                    Err(e) => {
                        progress_callback(format!("❌ Erro ao executar instalador: {}", e));
                    }
                }
            }
            Ok(_) => {
                progress_callback(crate::i18n::t_failed_download(&url));
            }
            Err(e) => {
                progress_callback(format!("❌ wget não encontrado ou erro: {}", e));
                progress_callback("Tente: sudo pacman -S wget".to_string());
            }
        }
    }

    progress_callback(crate::i18n::t("finalizing"));
}

/// Executa um comando Wine no prefixo especificado, capturando stderr em background.
pub fn execute_wine_command(prefix: &str, command_type: &str) -> Result<(), String> {
    let mut cmd;
    match command_type {
        "winecfg" => {
            cmd = Command::new("winecfg");
        }
        "explorer" => {
            cmd = Command::new("wine");
            cmd.arg("explorer");
        }
        "winetricks" => {
            cmd = Command::new("winetricks");
        }
        custom_exe => {
            // Assume que é um caminho para um .exe para rodar no wine
            cmd = Command::new("wine");
            cmd.arg(custom_exe);
        }
    }

    if !prefix.is_empty() {
        cmd.env("WINEPREFIX", prefix);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            // Log stderr em thread separada (wine spameia stderr normalmente)
            if let Some(stderr) = child.stderr.take() {
                std::thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().map_while(Result::ok) {
                        eprintln!("[wine] {}", line);
                    }
                });
            }
            // Aguarda o processo em thread separada para evitar zumbis.
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            Ok(())
        }
        Err(e) => Err(format!("Falha ao iniciar processo: {}", e)),
    }
}
