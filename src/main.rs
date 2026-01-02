use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Local};
use clap::Parser;
use notify_rust::{Hint, Timeout, Urgency};

const RECAPLOG_FILE_PATH: &'static str = "RECAPLOG";
const RECAPMSG_FILE_PATH: &'static str = "RECAPMSG";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = recap_log_default_file_path().into_os_string())]
    messages_location: PathBuf,

    #[arg(short, long, default_value = "alacritty -e nvim %p")]
    editor_command: String,
}

fn main() {
    let args = Args::parse();
    unsafe {
        env::set_var("DISPLAY", ":0");
        env::set_var("DBUS_SESSION_BUS_ADDRESS", "unix:path=/run/user/1000/bus");
    }

    notify_rust::Notification::new()
        .summary("1 Hour Recap")
        .appname("dayrecap")
        .action("default", "recap")
        .urgency(Urgency::Critical)
        .hint(Hint::Resident(true))
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .wait_for_action(|action| {
            if action == "default" {
                capture_message(&args);
            }
        });
}

fn recap_log_default_file_path() -> PathBuf {
    env::home_dir()
        .expect("Couldn't find home dir")
        .join(RECAPLOG_FILE_PATH)
}

fn recap_msg_file_path() -> PathBuf {
    PathBuf::new().join(RECAPMSG_FILE_PATH)
}

fn editor_command_sub_vars(editor_cmd: &str, path: &Path) -> String {
    editor_cmd.replace("%p", path.to_str().expect("Failed to parse path as string"))
}

fn open_editor(editor_command: &str, path: &Path) -> io::Result<()> {
    Command::new(format!("bash"))
        .arg("-c")
        .arg(editor_command_sub_vars(editor_command, path))
        .spawn()?
        .wait()?;

    Ok(())
}

fn format_message(message: &mut String) {
    let time: DateTime<Local> = std::time::SystemTime::now().into();
    let time = time.format("%Y/%m/%d %H:%M");
    message.insert_str(0, format!("{} MESSAGE START\n", time).as_str());
    message.push_str("\nMESSAGE END\n\n");
}

fn capture_message(args: &Args) {
    let msg_path = &recap_msg_file_path();
    open_editor(&args.editor_command, msg_path).expect("Failed to open editor");
    let mut content = std::fs::read_to_string(msg_path).expect("Failed to read recap message");
    std::fs::remove_file(msg_path).expect("Failed to delete recap message file");

    format_message(&mut content);

    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&recap_log_default_file_path())
        .expect("Failed to open recap log file")
        .write_all(content.as_bytes())
        .expect("Failed to write to recap log file");
}
