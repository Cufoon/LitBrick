use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_current_dir() -> Result<PathBuf, String> {
    match env::current_dir() {
        Ok(path) => Ok(path),
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
}

pub fn find_file_upwards(start: &Path, filename: &str) -> Option<PathBuf> {
    let mut current = start.to_path_buf();

    loop {
        let file_path = current.join(filename);
        if file_path.exists() {
            return Some(file_path);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

pub fn run_command<S>(cmd: &[S]) -> Result<(), String>
where
    S: AsRef<OsStr>,
{
    if cmd.is_empty() {
        return Err("No command provided".to_string());
    }

    let program = &cmd[0];
    let mut command = Command::new(program);
    command.args(&cmd[1..]);
    command.stdout(Stdio::piped());

    let mut command = command.spawn().map_err(|e| e.to_string())?;
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            println!("{}", line);
        }
    }

    let status = command.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Command exited with error status.".to_string());
    }

    Ok(())
}