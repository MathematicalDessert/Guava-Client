use lazy_static::lazy_static;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;

lazy_static! {
    static ref VERSION_CONTENT_PATH: String = "/content/".to_string();
    static ref CUSTOM_CONTENT_DIR_NAME: String = "custom-content".to_string();
}

/*pub fn get_roblox_directory() -> Result<&DirEntry, String> {
    let local_app_data_dir = match env::var("LOCALAPPDATA") {
        Ok(directory) => directory,
        Err(_) => return Err("Unable to fetch %LOCALAPPDATA% environment variable".to_string()),
    };

    let roblox_path = Path::new(local_app_data_dir. "/");
}*/