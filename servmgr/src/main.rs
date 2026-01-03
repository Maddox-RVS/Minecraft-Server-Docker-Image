mod server_activator;

use clap::{Parser, Subcommand};

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
    Man,

    /// Create an automatic backup of the world loaded onto the server after a set period of time (default is 1 hour),
    /// each argument is respective of itself (e.g.) "-s 5 -m 10 -h 3 -d 4" is 4 days, 3 hours, 10 minutes, and 5 seconds between each backup
    Auto {
        /// Seconds between each backup
        #[arg(short, long, default_value = "0")]
        seconds: i32,

        /// Minutes between each backup
        #[arg(short, long, default_value = "0")]
        minutes: i32,

        /// Hours between each backup
        #[arg(short = 'H', long, default_value = "0")]
        hours: i32,

        /// Days between each backup
        #[arg(short, long, default_value = "0")]
        days: i32,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start {detached} => {
            server_activator::start_minecraft_server(detached);
        },
        Commands::Stop {..} => {
            server_activator::stop_minecraft_server();
        },
        Commands::Backup {action} => {
            match action {
                BackupCommands::Man {..} => {
                    println!("Backing up server manually...");
                },
                BackupCommands::Auto {seconds, minutes, hours, days} => {
                    println!("Backing up server automatically...\n\tSeconds: {}\n\tMinutes: {}\n\tHours: {}\n\tDays: {}", seconds, minutes, hours, days);
                },
            }
        },
        Commands::Worlds {..} => {
            println!("Opening worlds manager...");
        },
    }
}
