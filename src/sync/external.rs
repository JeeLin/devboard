use std::process::Command;

pub struct ExternalSyncConfig {
    pub method: String,  // syncthing, ssh
    pub target: String,  // directory or remote path
}

impl ExternalSyncConfig {
    pub fn from_config() -> Option<Self> {
        let config_path = dirs::home_dir()?
            .join(".devboard")
            .join("config.toml");
        
        if !config_path.exists() {
            return None;
        }
        
        let content = std::fs::read_to_string(&config_path).ok()?;
        
        let mut method = None;
        let mut target = None;
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("external_method") {
                method = line.split('=').nth(1)?.trim().trim_matches('"').trim_matches('\'').to_string().into();
            } else if line.starts_with("external_target") {
                target = line.split('=').nth(1)?.trim().trim_matches('"').trim_matches('\'').to_string().into();
            }
        }
        
        Some(ExternalSyncConfig {
            method: method?,
            target: target?,
        })
    }
}

pub fn external_status() -> Result<String, String> {
    let config = ExternalSyncConfig::from_config()
        .ok_or("External sync not configured.")?;
    
    match config.method.as_str() {
        "syncthing" => {
            // Check if syncthing is running
            let output = Command::new("pgrep")
                .arg("syncthing")
                .output()
                .map_err(|e| format!("Failed to check syncthing: {}", e))?;
            
            if output.status.success() {
                Ok(format!("Syncthing: running, target: {}", config.target))
            } else {
                Ok(format!("Syncthing: not running, target: {}", config.target))
            }
        }
        "ssh" => {
            Ok(format!("SSH sync: target: {}", config.target))
        }
        _ => Err(format!("Unknown sync method: {}", config.method)),
    }
}

pub fn external_start() -> Result<String, String> {
    let config = ExternalSyncConfig::from_config()
        .ok_or("External sync not configured.")?;
    
    match config.method.as_str() {
        "syncthing" => {
            // Try to start syncthing
            let output = Command::new("syncthing")
                .arg("--no-browser")
                .output()
                .map_err(|e| format!("Failed to start syncthing: {}", e))?;
            
            if output.status.success() {
                Ok("Syncthing started".to_string())
            } else {
                Err(format!("Failed to start syncthing: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        "ssh" => {
            // SSH sync is typically triggered by rsync or scp
            let db_dir = super::git::get_db_dir();
            let output = Command::new("rsync")
                .args(["-avz", db_dir.to_str().unwrap_or("."), &config.target])
                .output()
                .map_err(|e| format!("Failed to sync via rsync: {}", e))?;
            
            if output.status.success() {
                Ok("SSH sync completed".to_string())
            } else {
                Err(format!("SSH sync failed: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        _ => Err(format!("Unknown sync method: {}", config.method)),
    }
}
