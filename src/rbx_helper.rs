use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static! {
    // TODO: Future, can create version for Macs?

    static ref CLIENT_VERSION_URL: String = "https://clientsettingscdn.roblox.com/v1/client-version/WindowsPlayer".to_string();
    static ref STUDIO_VERSION_URL: String = "https://clientsettingscdn.roblox.com/v1/client-version/WindowsStudio".to_string();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobloxVersionResponse {
    version: String,
    #[serde(rename = "clientVersionUpload")]
    client_version_upload: String,
    #[serde(rename = "bootstrapperVersion")]
    bootstrapper_version: String,
}

async fn get_version(url: &String) -> Result<RobloxVersionResponse, String> {
    let response = reqwest::get(url).await;

    match response {
        Ok(result) => {
            let body = result.text().await;

            match body {
                Ok(raw_body) => {
                    let version_response: RobloxVersionResponse = serde_json::from_str(raw_body.as_str()).unwrap();
                    Ok(version_response)
                },
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }
}

pub async fn get_roblox_client_version() -> Result<RobloxVersionResponse, String> {
    Ok(get_version(&CLIENT_VERSION_URL).await?)
}

pub async fn get_roblox_studio_version() -> Result<RobloxVersionResponse, String> {
    Ok(get_version(&STUDIO_VERSION_URL).await?)
}