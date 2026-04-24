use std::time::Duration;

const BASE_URL: &str = "https://api.nexusmods.com/v1";

/// Valida se a chave do Nexus é válida fazendo uma chamada simples de perfil.
/// Retorna Ok(nome_do_usuario) ou Err com mensagem amigável.
pub fn validate_api_key(api_key: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("GamePiLot/0.1")
        .build()
        .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

    let resp = client
        .get(format!("{}/users/validate.json", BASE_URL))
        .header("apikey", api_key)
        .send()
        .map_err(|_| crate::i18n::t("nexus_no_internet"))?;

    match resp.status().as_u16() {
        200 => {
            let json: serde_json::Value = resp
                .json()
                .map_err(|_| "Resposta inválida do Nexus Mods".to_string())?;
            let name = json["name"]
                .as_str()
                .unwrap_or("usuário")
                .to_string();
            Ok(name)
        }
        401 => Err(crate::i18n::t("nexus_key_invalid")),
        429 => Err(crate::i18n::t("nexus_rate_limit")),
        _ => Err(crate::i18n::t("nexus_error_generic")),
    }
}

/// Obtém os metadados dos arquivos de um mod específico.
/// Retorna o file_id e nome do arquivo mais recente.
pub fn get_latest_file(
    api_key: &str,
    game_domain: &str,
    mod_id: u32,
) -> Result<(u64, String), String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("GamePiLot/0.1")
        .build()
        .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

    let url = format!("{}/games/{}/mods/{}/files.json", BASE_URL, game_domain, mod_id);
    let resp = client
        .get(&url)
        .header("apikey", api_key)
        .send()
        .map_err(|e| format!("{} ({})", crate::i18n::t("nexus_no_internet"), e))?;

    if resp.status().as_u16() == 401 {
        return Err(crate::i18n::t("nexus_key_invalid"));
    }
    if resp.status().as_u16() == 404 {
        return Err(crate::i18n::t("nexus_mod_not_found"));
    }

    let json: serde_json::Value = resp.json().map_err(|_| "Resposta inválida".to_string())?;
    let files = json["files"].as_array().ok_or("Nenhum arquivo encontrado")?;

    // Pegar o arquivo mais recente da categoria "main"
    let latest = files
        .iter()
        .filter(|f| f["category_name"].as_str().unwrap_or("") == "MAIN")
        .max_by_key(|f| f["file_id"].as_u64().unwrap_or(0))
        .or_else(|| files.iter().max_by_key(|f| f["file_id"].as_u64().unwrap_or(0)))
        .ok_or_else(|| crate::i18n::t("nexus_mod_not_found"))?;

    let file_id = latest["file_id"].as_u64().ok_or("file_id inválido")?;
    let file_name = latest["file_name"]
        .as_str()
        .unwrap_or("installer.exe")
        .to_string();

    Ok((file_id, file_name))
}

/// Gera o link temporário de download para um arquivo específico.
pub fn get_download_url(
    api_key: &str,
    game_domain: &str,
    mod_id: u32,
    file_id: u64,
) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("GamePiLot/0.1")
        .build()
        .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

    let url = format!(
        "{}/games/{}/mods/{}/files/{}/download_link.json",
        BASE_URL, game_domain, mod_id, file_id
    );

    let resp = client
        .get(&url)
        .header("apikey", api_key)
        .send()
        .map_err(|e| format!("{} ({})", crate::i18n::t("nexus_no_internet"), e))?;

    match resp.status().as_u16() {
        200 => {
            let links: serde_json::Value = resp.json().map_err(|_| "Resposta inválida".to_string())?;
            let uri = links
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|l| l["URI"].as_str())
                .ok_or_else(|| "Link de download não encontrado".to_string())?;
            Ok(uri.to_string())
        }
        401 => Err(crate::i18n::t("nexus_key_invalid")),
        403 => Err("Download de usuários free requer acesso manual ao Nexus Mods".to_string()),
        429 => Err(crate::i18n::t("nexus_rate_limit")),
        _ => Err(crate::i18n::t("nexus_error_generic")),
    }
}
