mod profile;
mod tool;
mod sync;

use clap::{Parser, Subcommand};
use profile::{GitConfig, RepoItem, RepoItemData};
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "LitBrick")]
#[command(author = "Cufoon Lin")]
#[command(version)]
#[command(about = "A simple tool to manage sub-things!")]
#[command(long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initializing configuration file")]
    Init {},
    #[command(about = "Sync a sub-repo to local position")]
    #[command(long_about = "If you do not supply a name, all sub-repo will synced!")]
    Sync {
        #[arg(value_name = "NAME")]
        #[arg(help = "The name of sub-repo to synchronize")]
        name_position: Option<String>,
        #[arg(short, long)]
        #[arg(help = "The name of sub-repo to synchronize")]
        name: Option<String>,
        #[arg(short, long)]
        #[arg(help = "Whether or not to clean before synchronize")]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => {
            println!("Initializing...");
            let path = Path::new(".litbrick");
            if path.exists() {
                if path.is_dir() {
                    println!("Error: {} is a directory.", path.display());
                    return;
                } else {
                    println!("The file {} already exists. Do you want to overwrite it? (y/n)", path.display());
                    let mut input = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut input).unwrap();
                    if input.trim().to_ascii_lowercase() != "y" {
                        println!("File will not be overwritten.");
                        println!("Exit.");
                        return;
                    }
                }
            }
            let data = vec![RepoItem {
                name: "example".to_string(),
                path: "example".to_string(),
                data: RepoItemData::Git(GitConfig {
                    url: "https://example.com/example.git".to_string(),
                    hash: "".to_string(),
                }),
            }];
            let mut file = match File::create(".litbrick") {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to create file: {}", e);
                    return;
                }
            };
            let serialized: Vec<u8> = serde_json::to_vec(&data).unwrap();
            if let Err(e) = file.write_all(&serialized) {
                println!("Failed to write to file: {}", e);
            } else {
                println!("Successfully wrote to file!");
            }
        }
        Commands::Sync { name, name_position, force } => {
            if cli.verbose {
                println!("Verbose mode is on.");
            }
            let name = name.or(name_position);
            println!("Hello, {:?}!", name);
            if force && name.is_none() {
                println!("when using --force, the name is required. for all repos, using * for name");
                return;
            }
            match tool::get_current_dir() {
                Ok(current_dir) => {
                    println!("Current directory: {:?}", current_dir);
                    let filename = ".litbrick";
                    if let Some(file_path) = tool::find_file_upwards(&current_dir, filename) {
                        println!("Found file at: {:?}", file_path);
                        match fs::read_to_string(file_path) {
                            Ok(contents) => {
                                match serde_json::from_str::<Vec<RepoItem>>(&contents) {
                                    Ok(items) => {
                                        if force {
                                            let mut items = items;
                                            let name = name.unwrap();
                                            if name == "*" {
                                                println!("you choose to force sync everyone.");
                                            } else {
                                                let item_found = items.iter().find(|item| item.name == name);
                                                if item_found.is_none() {
                                                    println!("the repo {} is not exist!", name);
                                                    return;
                                                }
                                                items = vec![item_found.unwrap().clone()];
                                            }
                                            sync::sync_force(&items);
                                            return;
                                        }
                                        sync::sync(&items);
                                    }
                                    Err(e) => {
                                        println!("Failed to deserialize: {}", e);
                                    }
                                }
                            }
                            Err(e) => println!("Failed to read file: {}", e),
                        };
                    } else {
                        println!("File {} not found.", filename);
                    }
                }
                Err(err_msg) => {
                    eprintln!("{}", err_msg); // 使用 eprintln! 打印错误信息到标准错误
                }
            }
        }
    }
}
