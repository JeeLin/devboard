
pub struct WebDavConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl WebDavConfig {
    pub fn from_config() -> Option<Self> {
        let config_path = dirs::home_dir()?
            .join(".devboard")
            .join("config.toml");
        
        if !config_path.exists() {
            return None;
        }
        
        let content = std::fs::read_to_string(&config_path).ok()?;
        
        // Simple TOML parsing for webdav config
        let mut url = None;
        let mut username = None;
        let mut password = None;
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("webdav_url") {
                url = line.split('=').nth(1)?.trim().trim_matches('"').trim_matches('\'').to_string().into();
            } else if line.starts_with("webdav_username") {
                username = line.split('=').nth(1)?.trim().trim_matches('"').trim_matches('\'').to_string().into();
            } else if line.starts_with("webdav_password") {
                password = line.split('=').nth(1)?.trim().trim_matches('"').trim_matches('\'').to_string().into();
            }
        }
        
        Some(WebDavConfig {
            url: url?,
            username: username?,
            password: password?,
        })
    }
    
    pub fn save(&self) -> Result<(), String> {
        let config_path = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join(".devboard")
            .join("config.toml");
        
        let mut content = if config_path.exists() {
            std::fs::read_to_string(&config_path).unwrap_or_default()
        } else {
            String::new()
        };
        
        // Remove existing webdav config
        let lines: Vec<String> = content.lines()
            .filter(|l| !l.trim().starts_with("webdav_"))
            .map(|l| l.to_string())
            .collect();
        content = lines.join("\n");
        
        content.push_str(&format!("\n[webdav]\nwebdav_url = \"{}\"\nwebdav_username = \"{}\"\nwebdav_password = \"{}\"\n", 
            self.url, self.username, self.password));
        
        std::fs::write(&config_path, content)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        Ok(())
    }
}

pub fn init_webdav(url: &str, username: &str, password: &str) -> Result<String, String> {
    let config = WebDavConfig {
        url: url.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };
    
    config.save()?;
    Ok("WebDAV configuration saved".to_string())
}

pub fn webdav_push() -> Result<String, String> {
    let _config = WebDavConfig::from_config()
        .ok_or("WebDAV not configured. Run 'devboard sync webdav init' first.")?;
    
    // Placeholder: actual WebDAV implementation would use a WebDAV client library
    Ok("WebDAV push: not yet implemented (requires HTTP client library)".to_string())
}

pub fn webdav_pull() -> Result<String, String> {
    let _config = WebDavConfig::from_config()
        .ok_or("WebDAV not configured. Run 'devboard sync webdav init' first.")?;
    
    // Placeholder: actual WebDAV implementation would use a WebDAV client library
    Ok("WebDAV pull: not yet implemented (requires HTTP client library)".to_string())
}
