mod backup_controller;
mod server_activator;

use clap::{Parser, Subcommand};
use console::style;

#[derive(Parser)]
#[command(name = "servmgr", about = "Minecraft Server Manager")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the minecraft server and attaches your shell to a tmux session dashboard for server monitoring
    Start {
        /// Starts the tmux session containing the server dashboard but doesn't attach it to your shell
        #[arg(short, long)]
        detached: bool,
    },

    /// Stops the minecraft server and subsequently the tmux session for server monitoring
    Stop,

    /// Creates a backup of the server
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },

    /// Manages worlds on the server (creating new worlds, selecting active worlds, deleting worlds)
    Worlds,
}

#[derive(Subcommand)]
enum BackupCommands {
    /// Create a manual one time backup of the world loaded onto the server in its current state
    Man {
        /// RCON password for the server
        #[arg(short, long)]
        password_rcon: String,
    },

    /// Create an automatic backup of the world loaded onto the server after a set period of time (default is 1 hour),
    /// each argument is respective of itself (e.g.) "-s 5 -m 10 -h 3 -D 4" is 4 days, 3 hours, 10 minutes, and 5 seconds between each backup
    Auto {
        /// RCON password for the server
        #[arg(short, long)]
        password_rcon: String,

        /// Starts the backup schedule in detached mode (doesn't attach to the tmux session)
        #[arg(short, long)]
        detached: bool,

        /// Seconds between each backup
        #[arg(short, long, default_value = "30")]
        seconds: i32,

        /// Minutes between each backup
        #[arg(short, long, default_value = "0")]
        minutes: i32,

        /// Hours between each backup
        #[arg(short = 'H', long, default_value = "0")]
        hours: i32,

        /// Days between each backup
        #[arg(short = 'D', long, default_value = "0")]
        days: i32,
    },

    /// Stops the automatic backup schedule
    AutoStop {
        /// RCON password for the server
        #[arg(short, long)]
        password_rcon: String
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start {detached} => {
            server_activator::start_minecraft_server(&detached);
        },
        Commands::Stop {..} => {
            server_activator::stop_minecraft_server();
        },
        Commands::Backup {action} => {
            match action {
                BackupCommands::Man {password_rcon} => {
                    let current_time = chrono::Local::now();
                    let success: bool = backup_controller::backup_minecraft_server(&password_rcon);
                    if success {
                        println!("{}", style(format!("[INFO] Backed up Minecraft server at {}", current_time.format("%Y-%m-%d %H:%M:%S"))).cyan().to_string());
                    } else {
                        println!("{}", style(format!("[ERROR] Failed to back up Minecraft server at {}", current_time.format("%Y-%m-%d %H:%M:%S"))).red().to_string());
                    }
                },
                BackupCommands::Auto {password_rcon, detached, seconds, minutes, hours, days} => {
                    let interval = std::time::Duration::from_secs(
                        seconds as u64 + minutes as u64 * 60 + hours as u64 * 3600 + days as u64 * 86400);
                    backup_controller::start_schedule_minecraft_server_backups(&password_rcon, &interval, &detached);
                },
                BackupCommands::AutoStop {password_rcon} => {
                    backup_controller::stop_scheduled_minecraft_server_backups(&password_rcon);
                },
            }
        },
        Commands::Worlds {..} => {
            println!("Opening worlds manager...");
        },
    }
}
