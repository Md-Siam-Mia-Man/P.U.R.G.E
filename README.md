<p align="center"><img src="assets/img/Banner.png" alt="P.U.R.G.E. Banner"></p>

<p align="center">
  <img alt="Build Status" src="https://img.shields.io/badge/build-passing-brightgreen">
  <img alt="Version" src="https://img.shields.io/badge/version-1.2.0-blue">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux-informational">
</p>

> [!WARNING] > **Use at your own risk!** 💀 This is powerful software. I am not responsible for anything that could happen to your phone. Always make a full backup of your data before proceeding.

<p align="center">
  <em>Say goodbye to bloatware with a slick, modern, and truly native GUI.</em>
</p>

![P.U.R.G.E. Screenshot](assets/img/UI.png)

---

## 📖 Summary

Welcome to **P.U.R.G.E.**! 📱 This is a native debloater GUI built from scratch in Rust and `egui` for a fast, responsive, and truly cross-platform experience. Our mission is to help you reclaim your device's privacy and battery life by making it simple to remove unnecessary system apps. A leaner device also means a smaller [attack surface](https://en.wikipedia.org/wiki/Attack_surface), hardening your security.

Packages are documented to give you a clear idea of what's safe to remove. The worst-case scenario is a bootloop, but don't panic! After about 5 failed boots, your phone will enter recovery mode, where you can perform a factory reset.

**Bottom line: You CANNOT permanently brick your device with this software!**

---

## ✨ Features

- **🚀 Blazing Fast & Feather-Light:** Built with native Rust for instant startup and minimal resource usage. No sluggish Electron apps here!
- **🎨 Custom-Crafted Native Interface:** No web-views, no bloat. A stunning dark theme with a custom, frame-less title bar and a responsive layout built for clarity.
- **💡 Intuitive Package Inspector:** A master-detail view. Click any package to see its full description, dependencies, and labels in a dedicated side panel.
- **🚦 At-a-Glance Safety Indicators:** Color-coded dots (🟢, 🟡, 🔴) instantly show the removal safety level for each package, right next to its name.
- **🔍 Advanced Filtering & Search:** Instantly find any package with powerful search and multi-category filters for lists and safety levels.
- **🖥️ Integrated Status Display:** A clean, intelligent status indicator in the sidebar keeps you informed without the clutter of a verbose log.
- **📦 All-In-One Executable:** The required ADB binaries for Windows and Linux are embedded directly into the app. Zero setup required!
- **💻 Genuinely Cross-Platform:** One codebase that compiles and runs flawlessly on both Windows and Linux.

---

## 📚 Debloat Lists

This tool stands on the shoulders of giants, using the meticulously researched package lists from the original UAD project. It covers dozens of manufacturers and major mobile carriers.

---

## 🛠️ How To Use It

#### Step 1: Prep Your Phone 📱

1.  **BACKUP YOUR DATA!** Seriously. Do it.
2.  **Enable Developer Options:** Go to `Settings > About Phone` and tap `Build Number` 7 times. You're a developer now!
3.  **Enable USB Debugging:** Find the new `Developer Options` menu and toggle on `USB Debugging`.

#### Step 2: Use The App 🖥️

1.  **Download P.U.R.G.E.:** Grab the latest release for your OS from the [**Releases Page**](https://github.com/Md-Siam-Mia-Man/UAD-Universal-Android-Debloater/releases).
2.  **Connect Your Phone:** Plug your device into your computer. A prompt to "Allow USB debugging" will appear on your phone. Check "Always allow" and tap "Allow".
3.  **Launch P.U.R.G.E.:** Double-click the executable.
4.  **List Packages:**
    - In the sidebar, click the big **`🔄 Refresh Connection`** button.
    - The app will find your device, display its name, and list all removable packages.
5.  **Select & Destroy:**
    - Use the **Search** and **Filter** controls at the top of the list to find your targets.
    - **Click anywhere** on a package card to select it for removal.
    - When you're ready, smash the big **`🔥 Purge`** button in the sidebar.
6.  **Reboot & Enjoy:** Click **`Reboot Device`** to restart your phone and enjoy a cleaner, faster experience!

> **P.S.** Your phone manufacturer might reinstall bloatware after a major system update. Just run this tool again to clean it up!

---

## 🏗️ Building From Source

1.  **Install Rust:** `https://rustup.rs/`
2.  **Install build dependencies** (see the [eframe docs](https://github.com/emilk/eframe#compiling) for your OS).
3.  **Clone the Repo** and `cd` into it.
4.  **Run with Cargo:** `cargo run --release`

---

## 🙏 Acknowledgements & Credits

This project would not be possible without the incredible work done by the original **Universal Android Debloater** team and its contributors, especially **0x192**, for researching and maintaining the package data.

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
