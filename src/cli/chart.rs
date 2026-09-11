use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum ChartCommands {
    /// Export a specific chart
    Export {
        /// Chart type (gantt/distribution)
        #[arg(short = 't', long)]
        chart_type: String,
        /// Output format (svg/csv)
        #[arg(short = 'f', long, default_value = "svg")]
        format: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Date for distribution chart (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    /// Export all charts
    ExportAll {
        /// Output directory
        #[arg(short, long, default_value = "./charts")]
        dir: String,
        /// Output format (svg/csv)
        #[arg(short = 'f', long, default_value = "svg")]
        format: String,
    },
}

pub fn handle(args: ChartCommands) {
    match args {
        ChartCommands::Export { chart_type, format: _, output, date } => {
            let output_path = PathBuf::from(&output);
            match chart_type.as_str() {
                "gantt" => {
                    match crate::charts::gantt::generate_gantt(0) {
                        Ok(svg) => {
                            std::fs::write(&output_path, svg).unwrap_or_else(|e| {
                                eprintln!("Error writing file: {}", e);
                                std::process::exit(1);
                            });
                            println!("Exported gantt chart to {}", output);
                        }
                        Err(e) => eprintln!("Error generating chart: {}", e),
                    }
                }
                "distribution" => {
                    let target_date = date.unwrap_or_else(|| {
                        chrono::Local::now().format("%Y-%m-%d").to_string()
                    });
                    match crate::charts::distribution::generate_distribution(&target_date) {
                        Ok(svg) => {
                            std::fs::write(&output_path, svg).unwrap_or_else(|e| {
                                eprintln!("Error writing file: {}", e);
                                std::process::exit(1);
                            });
                            println!("Exported distribution chart to {}", output);
                        }
                        Err(e) => eprintln!("Error generating chart: {}", e),
                    }
                }
                _ => {
                    eprintln!("Unknown chart type: {}", chart_type);
                    eprintln!("Supported types: gantt, distribution");
                }
            }
        }
        ChartCommands::ExportAll { dir, format: _ } => {
            let output_dir = PathBuf::from(&dir);
            std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
                eprintln!("Error creating directory: {}", e);
                std::process::exit(1);
            });

            // Export gantt
            let gantt_path = output_dir.join("gantt.svg");
            match crate::charts::gantt::generate_gantt(0) {
                Ok(svg) => {
                    std::fs::write(&gantt_path, svg).unwrap_or_else(|e| {
                        eprintln!("Error writing gantt: {}", e);
                    });
                    println!("Exported: {}", gantt_path.display());
                }
                Err(e) => eprintln!("Error generating gantt: {}", e),
            }

            // Export distribution
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let dist_path = output_dir.join("distribution.svg");
            match crate::charts::distribution::generate_distribution(&today) {
                Ok(svg) => {
                    std::fs::write(&dist_path, svg).unwrap_or_else(|e| {
                        eprintln!("Error writing distribution: {}", e);
                    });
                    println!("Exported: {}", dist_path.display());
                }
                Err(e) => eprintln!("Error generating distribution: {}", e),
            }
        }
    }
}
