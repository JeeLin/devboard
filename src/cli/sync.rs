use clap::Subcommand;

#[derive(Subcommand)]
pub enum SyncCommands {
    /// Git sync
    #[command(subcommand)]
    Git(GitCommands),
    /// WebDAV sync
    #[command(subcommand)]
    Webdav(WebdavCommands),
    /// External sync (Syncthing/SSH)
    #[command(subcommand)]
    External(ExternalCommands),
}

#[derive(Subcommand)]
pub enum GitCommands {
    /// Initialize Git repository
    Init,
    /// Push changes to remote
    Push {
        /// Remote name (default: origin)
        #[arg(short, long)]
        remote: Option<String>,
    },
    /// Pull changes from remote
    Pull {
        /// Remote name (default: origin)
        #[arg(short, long)]
        remote: Option<String>,
    },
    /// Show Git status
    Status,
}

#[derive(Subcommand)]
pub enum WebdavCommands {
    /// Configure WebDAV connection
    Init {
        /// WebDAV server URL
        #[arg(long)]
        url: String,
        /// Username
        #[arg(long)]
        user: String,
        /// Password
        #[arg(long)]
        password: String,
    },
    /// Push data to WebDAV server
    Push,
    /// Pull data from WebDAV server
    Pull,
}

#[derive(Subcommand)]
pub enum ExternalCommands {
    /// Show external sync status
    Status,
    /// Start external sync
    Start,
}

pub fn handle(args: SyncCommands) {
    match args {
        SyncCommands::Git(git_args) => {
            match git_args {
                GitCommands::Init => {
                    match crate::sync::git::init_git() {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                GitCommands::Push { remote } => {
                    match crate::sync::git::git_push(remote.as_deref()) {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                GitCommands::Pull { remote } => {
                    match crate::sync::git::git_pull(remote.as_deref()) {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                GitCommands::Status => {
                    match crate::sync::git::git_status() {
                        Ok(status) => {
                            if status.trim().is_empty() {
                                println!("Working directory clean");
                            } else {
                                println!("Changes:");
                                println!("{}", status);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
        SyncCommands::Webdav(webdav_args) => {
            match webdav_args {
                WebdavCommands::Init { url, user, password } => {
                    match crate::sync::webdav::init_webdav(&url, &user, &password) {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                WebdavCommands::Push => {
                    match crate::sync::webdav::webdav_push() {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                WebdavCommands::Pull => {
                    match crate::sync::webdav::webdav_pull() {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
        SyncCommands::External(external_args) => {
            match external_args {
                ExternalCommands::Status => {
                    match crate::sync::external::external_status() {
                        Ok(status) => println!("{}", status),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                ExternalCommands::Start => {
                    match crate::sync::external::external_start() {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }
}
