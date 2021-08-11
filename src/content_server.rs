use std::{fs::File, io::Write, path::{Path, PathBuf}};

use log::info;

pub struct ContentServer {
    url: String,
}

impl ContentServer {
    pub fn new(url: String) -> Self {
        // TODO: Check URL is valid, and server instance is good
        info!("Set ContentServer URL to: {}", url.clone());
        ContentServer {
            url
        }
    }

    pub async fn download(&self, path: PathBuf, id: String) -> Result<(), String> {
        let response = match reqwest::get(self.url.clone()).await {
            Ok(res) => res,
            Err(e) => return Err(e.to_string()),
        };

        let file_path: &Path = Path::new("");
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };

        match response.bytes().await {
            Ok(bytes) => {
                match file.write_all(&bytes) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            },
            Err(e) => Err(e.to_string()),
        }
    }
}