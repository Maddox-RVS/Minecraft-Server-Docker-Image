use std::os::unix::process::CommandExt;
use std::process::Command;
use console::style;

pub fn start_minecraft_server(detached: &bool) {
    let mut start_cmd: Command = Command::new("bash");
    start_cmd.arg("/home/mcadmin/minecraft_server/start_minecraft_server.sh");
    if *detached { start_cmd.arg("-d"); }

    println!("{}", style("Starting Minecraft Server...").cyan());
    let err = start_cmd.exec();
    eprintln!("{}: {}", style("Failed to start minecraft server").red(), err);
}

pub fn stop_minecraft_server() {
    let mut start_cmd: Command = Command::new("bash");
    start_cmd.arg("/home/mcadmin/minecraft_server/stop_minecraft_server.sh");

    println!("{}", style("Stopping Minecraft Server...").cyan());
    let err = start_cmd.exec();
    eprintln!("{}: {}", style("Failed to stop minecraft server").red(), err);
}