use std::path::PathBuf;
use std::process::Command;

pub fn get_db_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devboard")
}

pub fn init_git() -> Result<String, String> {
    let db_dir = get_db_dir();
    
    // Check if already a git repo
    if db_dir.join(".git").exists() {
        return Ok("Already a Git repository".to_string());
    }
    
    let output = Command::new("git")
        .arg("init")
        .current_dir(&db_dir)
        .output()
        .map_err(|e| format!("Failed to run git init: {}", e))?;
    
    if output.status.success() {
        // Create .gitignore
        let gitignore = "*.db-journal\n*.db-wal\n*.db-shm\n";
        std::fs::write(db_dir.join(".gitignore"), gitignore)
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;
        
        // Initial commit
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(&db_dir)
            .output();
        
        let _output = Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&db_dir)
            .output()
            .map_err(|e| format!("Failed to run git commit: {}", e))?;
        
        Ok("Git repository initialized".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn git_status() -> Result<String, String> {
    let db_dir = get_db_dir();
    
    if !db_dir.join(".git").exists() {
        return Err("Not a Git repository. Run 'devboard sync git init' first.".to_string());
    }
    
    let output = Command::new("git")
        .args(["status", "--short"])
        .current_dir(&db_dir)
        .output()
        .map_err(|e| format!("Failed to run git status: {}", e))?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn git_push(remote: Option<&str>) -> Result<String, String> {
    let db_dir = get_db_dir();
    
    if !db_dir.join(".git").exists() {
        return Err("Not a Git repository. Run 'devboard sync git init' first.".to_string());
    }
    
    // Stage all changes
    let _ = Command::new("git")
        .args(["add", "."])
        .current_dir(&db_dir)
        .output();
    
    // Check if there are changes
    let status = git_status()?;
    if status.trim().is_empty() {
        return Ok("No changes to commit".to_string());
    }
    
    // Commit
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let commit_msg = format!("Auto-sync: {}", timestamp);
    let output = Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .current_dir(&db_dir)
        .output()
        .map_err(|e| format!("Failed to commit: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    // Push
    let remote_name = remote.unwrap_or("origin");
    let output = Command::new("git")
        .args(["push", remote_name])
        .current_dir(&db_dir)
        .output()
        .map_err(|e| format!("Failed to push: {}", e))?;
    
    if output.status.success() {
        Ok("Changes committed and pushed".to_string())
    } else {
        Ok(format!("Committed but push failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

pub fn git_pull(remote: Option<&str>) -> Result<String, String> {
    let db_dir = get_db_dir();
    
    if !db_dir.join(".git").exists() {
        return Err("Not a Git repository. Run 'devboard sync git init' first.".to_string());
    }
    
    let remote_name = remote.unwrap_or("origin");
    let output = Command::new("git")
        .args(["pull", remote_name])
        .current_dir(&db_dir)
        .output()
        .map_err(|e| format!("Failed to pull: {}", e))?;
    
    if output.status.success() {
        Ok("Changes pulled successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
