# Universal Android Debloater 🦀

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-orange)

A blazing-fast, cross-platform, and user-friendly GUI for debloating Android devices, built with the safety and performance of Rust. This tool provides a simple and effective way to remove unwanted bloatware from your phone, freeing up resources and improving privacy.

---

## ✨ Features

-   **🚀 Blazing Fast:** Built with native Rust for maximum performance and minimum resource usage.
-   **🎨 Modern & Clean UI:** An intuitive and aesthetically pleasing interface built with the `egui` framework.
-   **🔍 Powerful Filtering:** Instantly search for packages by name, or filter by list type (`Recommended`, `Advanced`, etc.) and removal status.
-   **ℹ️ Detailed Information:** View descriptions, dependencies, and other critical information for each package before removal.
-   **🛡️ Safe & Reliable:** Leverage Rust's safety guarantees for a crash-free experience.
-   **📦 Self-Contained:** The required ADB binaries are embedded directly into the application. No external dependencies needed!
-   **💻 Cross-Platform:** Single codebase that compiles and runs on Windows, macOS, and Linux.

---

## 📸 Screenshots

<img src="./assets/img/UI.png" alt="UAD UI"></img>


> **Note:** You will need to replace the URL above with a link to an actual screenshot.

---

## 🏗️ Building From Source

Want to compile it yourself?

1.  **Install Rust:** If you don't have it, install the Rust toolchain: `https://rustup.rs/`
2.  **Clone the Repository:**
    ```bash
    git clone https://github.com/Md-Siam-Mia-Code/UAD-Universal-Android-Debloater.git
    cd UAD-Universal-Android-Debloater
    ```
3.  **Build & Run:**
    ```bash
    # For a debug build (faster compilation)
    cargo run

    # For a release build (optimized for performance)
    cargo run --release
    ```
---

## 🛠️ Usage Guide

Getting started is simple!

1.  **Download:** Grab the latest executable for your operating system from the [Releases Page](https://github.com/your-repo/UAD/releases).
2.  **Enable USB Debugging:** On your Android device, go to `Settings > About Phone` and tap on `Build Number` 7 times to unlock Developer Options. Then, go to `Settings > Developer Options` and enable `USB Debugging`.
3.  **Connect Device:** Connect your Android device to your computer via a USB cable. You may need to authorize the connection on your phone's screen.
4.  **Run UAD:** Launch the application.
5.  **Detect & List:**
    -   Click **`1. Detect Device`** to ensure your phone is recognized.
    -   Click **`2. List Packages`** to load all installed packages from your device.
6.  **Select & Uninstall:**
    -   Scroll through the list or use the filters to find packages you want to remove.
    -   Click the checkbox next to each package.
    -   Once you've made your selection, click **`Uninstall Selected (#)`**.
7.  **(Optional) Reboot:** Click **`Reboot Device`** to apply changes that may require a restart.

---

## 💻 Technical Details

This project was built from the ground up to be a lightweight and performant alternative to other debloater tools.

-   **Language:** [**Rust**](https://www.rust-lang.org/) (Edition 2021)
-   **GUI Framework:** [**egui**](https://github.com/emilk/egui) - A simple, fast, and highly portable immediate mode GUI library.
-   **ADB Interaction:** All Android Debug Bridge (ADB) commands are executed via Rust's standard `std::process::Command` API.

---

## 🙏 Acknowledgements & Credits

This project would not be possible without the incredible work done by the original **Universal Android Debloater** team.

The comprehensive package list (`uad_lists.json`), which forms the core intelligence of this application, is sourced directly from their repository. All credit for the monumental task of researching, compiling, and maintaining this data goes to **0x192** and the contributors to the original project.

-   **Original Project:** [0x192/universal-android-debloater](https://github.com/0x192/universal-android-debloater)
-   **Source for `uad_lists.json`:** [Link to JSON file](https://github.com/0x192/universal-android-debloater/blob/main/resources/assets/uad_lists.json)

Please support the original project!

---

## 🤝 Contributing

Contributions are welcome! If you have ideas for new features, bug fixes, or improvements, feel free to:

-   Open an issue to discuss your idea.
-   Submit a pull request with your changes.

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.