// adb.rs
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

fn extract_adb_binaries() -> PathBuf {
    let target_dir = std::env::temp_dir().join("uad_adb");

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).expect("Failed to create adb temp dir");
    }

    for file in ADB_DIR.files() {
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap();
        let dest_path = target_dir.join(file_name);

        if !dest_path.exists() {
            let mut f = File::create(&dest_path).expect("Failed to create adb file");
            f.write_all(file.contents())
                .expect("Failed to write adb file");

            // On Unix systems, we need to set the executable permission for the adb binary.
            #[cfg(unix)]
            if file_name == "adb" {
                use std::os::unix::fs::PermissionsExt;
                let perms = fs::Permissions::from_mode(0o755); // rwxr-xr-x
                fs::set_permissions(&dest_path, perms)
                    .expect("Failed to set executable permissions on adb");
            }
        }
    }

    target_dir
}

fn adb_path() -> PathBuf {
    let adb_dir = extract_adb_binaries();
    
    #[cfg(target_os = "windows")]
    let adb_executable = "adb.exe";
    #[cfg(not(target_os = "windows"))]
    let adb_executable = "adb";

    adb_dir.join(adb_executable)
}

pub fn detect_device() -> String {
    let output = Command::new(adb_path()).arg("devices").output();

    match output {
        Ok(res) => String::from_utf8_lossy(&res.stdout).to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn list_packages() -> Vec<String> {
    let output = Command::new(adb_path())
        .arg("shell")
        .arg("pm list packages")
        .output();

    match output {
        Ok(res) => String::from_utf8_lossy(&res.stdout)
            .lines()
            .map(|line| line.replace("package:", ""))
            .collect(),
        Err(_) => vec!["Failed to list packages.".to_string()],
    }
}

pub fn uninstall(package: &str) {
    let _ = Command::new(adb_path())
        .arg("shell")
        .arg("pm uninstall --user 0")
        .arg(package)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output();
}

pub fn reboot_device() {
    let _ = Command::new(adb_path())
        .arg("reboot")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}