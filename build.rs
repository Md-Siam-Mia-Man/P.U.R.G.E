// /build.rs
use std::env;

fn main() {
    // This script's purpose is to embed an icon in the Windows executable.
    // It should only run when the target OS is Windows.
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();

        // We need to determine if we are cross-compiling.
        // Cargo provides the HOST and TARGET environment variables for this.
        let host = env::var("HOST").unwrap();
        let target = env::var("TARGET").unwrap();

        // If the host and target are different, we are cross-compiling.
        // In this case, we must explicitly tell `winres` which tool to use.
        if host != target {
            res.set_windres_path("x86_64-w64-mingw32-windres");
        }
        
        // The set_icon method modifies the resource but does not return a Result.
        res.set_icon("assets/img/icon.ico");

        res.compile().expect("Failed to compile Windows resource file. Ensure mingw-w64 is installed and assets/img/icon.ico exists.");
    }
}