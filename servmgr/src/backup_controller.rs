use std::{thread, path::PathBuf, fs, process::Command, time::Duration};
use indicatif::{ProgressBar, ProgressStyle};
use std::os::unix::process::CommandExt;
use console::style;
use chrono;

fn is_rcon_responding(rcon_password: &str) -> bool {
    let rcon_connection_test_cmd_output = Command::new("rcon-cli")
        .arg("--password")
        .arg(rcon_password)
        .arg("list")
        .output()
        .expect("Failed to execute rcon connection test command");

    rcon_connection_test_cmd_output.status.success()
}

pub fn backup_minecraft_server(rcon_password: &str) -> bool {
    let status_spinner = ProgressBar::new_spinner();
    status_spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["(●     )", "( ●    )", "(  ●   )", "(   ●  )", "(    ● )", "(     ●)", "(    ● )", "(   ●  )", "(  ●   )", "( ●    )"])
            .template("{spinner:.cyan} [{elapsed}] {msg}")
            .unwrap());
    status_spinner.set_message(style("Backing up Minecraft server...").cyan().to_string());
    status_spinner.enable_steady_tick(Duration::from_millis(100));

    if is_rcon_responding(rcon_password) { // check rcon connection
        status_spinner.println(
            format!("{} {}", style("●").green().to_string(), style("RCON Interface: RESPONDING").cyan().to_string()));

        // starting backup notifications
        let rcon_server_backup_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"Creating Server Backup...\",color:\"#FF6600\"}")
            .output()
            .expect("Failed to execute rcon server backup notification");

        if !rcon_server_backup_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server backup notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        } 

        // disabling auto-writing from RAM to DISK
        let rcon_server_save_off_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("save-off")
            .output()
            .expect("Failed to execute rcon server save-off command");

        if !rcon_server_save_off_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to disable auto-writing from RAM to DISK").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        status_spinner.println(style(format!("=> Disabled auto-writing from RAM to DISK")).dim().to_string());
        let rcon_server_save_off_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> Disabled auto-writing from RAM to DISK\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server save-off notification");

        if !rcon_server_save_off_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server save-off notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        // flush server DISK to RAM
        status_spinner.println(style(format!("=> Flushing DISK to RAM...")).dim().to_string());
        let rcon_server_flush_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> Flushing DISK to RAM...\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server flush notification");

        if !rcon_server_flush_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server flush notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        let rcon_server_flush_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("save-all")
            .output()
            .expect("Failed to execute rcon server flush command");

        if !rcon_server_flush_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to flush server DISK to RAM").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        thread::sleep(Duration::from_secs(10)); // give ample time to let server write from RAM to DISK

        status_spinner.println(style(format!("=> => Flushed DISK to RAM")).dim().to_string());
        let rcon_server_flush_finished_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> => Flushed DISK to RAM\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server flush finished notification");

        if !rcon_server_flush_finished_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server flush finished notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        // save and compress world folder
        fs::create_dir_all("/home/mcadmin/minecraft_server/backups").expect("Failed to create backups directory");
        let current_time = chrono::Local::now();

        status_spinner.println(style(format!("=> Compressing world folder...")).dim().to_string());
        let rcon_server_compress_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> Compressing world folder...\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server compress notification");

        if !rcon_server_compress_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server compress notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        let backup_save_directory: PathBuf = PathBuf::from("/home/mcadmin/minecraft_server/backups");
        let backup_file_name = format!("world_backup_{}.tar.gz", current_time.format("%Y-%m-%d_%H-%M-%S"));
        let backup_file_path = backup_save_directory.join(&backup_file_name);
        let world_path: PathBuf = PathBuf::from("/home/mcadmin/minecraft_server/world");
        let tar_command_output = Command::new("tar")
            .arg("-czf")
            .arg(backup_file_path.to_str().unwrap())
            .arg(world_path.to_str().unwrap())
            .output()
            .expect("Failed to execute tar command");

        if !tar_command_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to compress world folder").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        status_spinner.println(style(format!("=> => World folder compressed and saved")).dim().to_string());
        let rcon_server_compress_finished_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> => World folder compressed and saved\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server compress finished notification");

        if !rcon_server_compress_finished_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server compress finished notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        // enable auto-writing from RAM to DISK
        let rcon_server_save_on_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("save-on")
            .output()
            .expect("Failed to execute rcon server save-on command");

        if !rcon_server_save_on_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to enable auto-writing from RAM to DISK").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        status_spinner.println(style(format!("=> Enabled auto-writing from RAM to DISK")).dim().to_string());
        let rcon_server_save_on_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg("execute as @a run tellraw @p {text:\"=> Enabled auto-writing from RAM to DISK\",color:\"#A094FF\"}")
            .output()
            .expect("Failed to execute rcon server save-on notification");

        if !rcon_server_save_on_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server save-on notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }

        // notify that backup is complete
        status_spinner.println(style(format!("Backup Complete: {}", backup_file_path.to_str().unwrap())).dim().to_string());
        let rcon_server_backup_complete_notification_output = Command::new("rcon-cli")
            .arg("--password")
            .arg(rcon_password)
            .arg(format!("execute as @a run tellraw @p {{\"text\":\"Backup complete: {}\",\"color\":\"dark_green\"}}", backup_file_name.to_string()))
            .output()
            .expect("Failed to execute rcon server backup complete notification");

        if !rcon_server_backup_complete_notification_output.status.success() {
            status_spinner.println(
                format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server backup complete notification").cyan().to_string()));
            status_spinner.finish_and_clear();
            return false;
        }
    } else {
        status_spinner.println(
            format!("{} {}", style("○").red().to_string(), style("RCON Interface: NOT RESPONDING (Server may be booting...)").cyan().to_string()));
        status_spinner.finish_and_clear();
        return false;
    }

    status_spinner.finish_and_clear();
    true
}

pub fn start_schedule_minecraft_server_backups(rcon_password: &str, interval: &Duration, detached: &bool) {
    let session_name = "backup_server_mgr";

    // check if tmux session already exists
    let tmux_has_session_cmd_output = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(session_name)
        .output()
        .expect("Failed to check if tmux session exists");

    if tmux_has_session_cmd_output.status.success() {
        println!("{}: Tmux session '{}' already exists", style("[INFO]").cyan(), session_name);
        return;
    }

    // create tmux session
    let tmux_session_creation_cmd_output = Command::new("tmux")
        .arg("new-session")
        .arg("-d")
        .arg("-s")
        .arg(session_name)
        .output()
        .expect("Failed to create tmux session");

    if !tmux_session_creation_cmd_output.status.success() {
        eprintln!("{} Failed to create tmux session with status code: {:?}", style("[ERROR]").red(), tmux_session_creation_cmd_output.status.code().unwrap_or(-1));
        return;
    }

    // start scheduled backups in tmux session
    let start_scheduled_backups_cmd_output = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(session_name)
        .arg(format!("/home/mcadmin/minecraft_server/start_scheduled_backups.sh -s {} -p {}", interval.as_secs(), rcon_password))
        .arg("C-m")
        .output()
        .expect("Failed to start scheduled backups");

    if !start_scheduled_backups_cmd_output.status.success() {
        eprintln!("{}: Failed to start scheduled backups with status code: {:?}", style("[ERROR]").red(), start_scheduled_backups_cmd_output.status.code().unwrap_or(-1));
        return;
    }

    // attach to tmux session if detached is false
    let mut attach_session_cmd: Command = Command::new("tmux");
    attach_session_cmd.arg("attach-session")
        .arg("-t")
        .arg(session_name);

    println!("{}", style("Starting Scheduled Backups...").cyan());
    if !*detached { 
        let err = attach_session_cmd.exec();
        eprintln!("{}: {}", style("Failed to start scheduled backups").red(), err);
    }
}

pub fn stop_scheduled_minecraft_server_backups(rcon_password: &str) {
    println!("{}", style("[INFO] Stopping automatic backup schedule...").cyan().to_string());
    let session_name = "backup_server_mgr";

    // check if tmux session exists
    let tmux_has_session_cmd_output = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(session_name)
        .output()
        .expect("Failed to check if tmux session exists");

    if !tmux_has_session_cmd_output.status.success() {
        println!("{}: Tmux session '{}' does not exist", style("[INFO]").cyan(), session_name);
        return;
    }

    // enable auto-writing from RAM to DISK
    let rcon_server_save_on_output = Command::new("rcon-cli")
        .arg("--password")
        .arg(rcon_password)
        .arg("save-on")
        .output()
        .expect("Failed to execute rcon server save-on command");

    if !rcon_server_save_on_output.status.success() {
        status_spinner.println(
            format!("{} {}", style("○").red().to_string(), style("Failed to enable auto-writing from RAM to DISK").cyan().to_string()));
        status_spinner.finish_and_clear();
        return false;
    }

    println!("{}", style(format!("=> Enabled auto-writing from RAM to DISK")).dim().to_string());
    let rcon_server_save_on_notification_output = Command::new("rcon-cli")
        .arg("--password")
        .arg(rcon_password)
        .arg("execute as @a run tellraw @p {text:\"=> Enabled auto-writing from RAM to DISK\",color:\"#A094FF\"}")
        .output()
        .expect("Failed to execute rcon server save-on notification");

    if !rcon_server_save_on_notification_output.status.success() {
        status_spinner.println(
            format!("{} {}", style("○").red().to_string(), style("Failed to send RCON server save-on notification").cyan().to_string()));
        status_spinner.finish_and_clear();
        return false;
    }

    // kill tmux session
    let tmux_kill_session_cmd_output = Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg(session_name)
        .output()
        .expect("Failed to kill tmux session");

    if !tmux_kill_session_cmd_output.status.success() {
        eprintln!("{} Failed to stop tmux session with status code: {:?}", style("[ERROR]").red(), tmux_kill_session_cmd_output.status.code().unwrap_or(-1));
        return;
    }

    println!("{}", style("[INFO] Automatic backup schedule stopped").cyan().to_string());
}