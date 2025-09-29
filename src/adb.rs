// src/adb.rs
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use include_dir::{include_dir, Dir};

#[cfg(target_os = "windows")]
const ADB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/vendor/win");
#[cfg(target_os = "linux")]
const ADB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/vendor/linux");
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This application is only supported on Windows and Linux.");

fn extract_adb_binaries() -> std::io::Result<PathBuf> {
    let target_dir = std::env::temp_dir().join("purge_adb");
    fs::create_dir_all(&target_dir)?;

    for file in ADB_DIR.files() {
        let dest_path = target_dir.join(file.path().file_name().unwrap());
        if !dest_path.exists() {
            let mut f = File::create(&dest_path)?;
            f.write_all(file.contents())?;

            #[cfg(unix)]
            if file.path().file_name().unwrap() == "adb" {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755))?;
            }
        }
    }
    Ok(target_dir)
}

fn adb_path() -> std::io::Result<PathBuf> {
    let adb_dir = extract_adb_binaries()?;
    #[cfg(target_os = "windows")]
    let adb_executable = "adb.exe";
    #[cfg(not(target_os = "windows"))]
    let adb_executable = "adb";
    Ok(adb_dir.join(adb_executable))
}

pub fn detect_device() -> Result<(), String> {
    let adb = adb_path().map_err(|e| format!("Failed to prepare ADB: {}", e))?;
    let output = Command::new(adb).arg("devices").output();

    match output {
        Ok(res) => {
            let stdout = String::from_utf8_lossy(&res.stdout);
            if stdout.lines().any(|line| line.ends_with("\tdevice")) {
                Ok(())
            } else {
                Err("No authorized device found.".to_string())
            }
        }
        Err(e) => Err(format!("ADB command failed: {}", e)),
    }
}

pub fn get_device_model() -> Result<String, String> {
    let adb = adb_path().map_err(|e| format!("Failed to prepare ADB: {}", e))?;
    let output = Command::new(adb)
        .arg("shell")
        .arg("getprop")
        .arg("ro.product.model")
        .output();

    match output {
        Ok(res) if res.status.success() => {
            let model = String::from_utf8_lossy(&res.stdout).trim().to_string();
            if model.is_empty() {
                Err("Device model name is empty.".to_string())
            } else {
                Ok(model)
            }
        }
        _ => Err("Could not retrieve device model.".to_string()),
    }
}

pub fn list_packages() -> Result<Vec<String>, String> {
    let adb = adb_path().map_err(|e| format!("Failed to prepare ADB: {}", e))?;
    let output = Command::new(adb)
        .arg("shell")
        .arg("pm list packages")
        .output();

    match output {
        Ok(res) => Ok(String::from_utf8_lossy(&res.stdout)
            .lines()
            .map(|line| line.replace("package:", ""))
            .collect()),
        Err(e) => Err(format!("Failed to list packages: {}", e)),
    }
}

pub fn uninstall(package: &str) {
    if let Ok(adb) = adb_path() {
        let _ = Command::new(adb)
            .arg("shell")
            .arg("pm uninstall --user 0")
            .arg(package)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

pub fn reboot_device() {
    if let Ok(adb) = adb_path() {
        let _ = Command::new(adb)
            .arg("reboot")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}