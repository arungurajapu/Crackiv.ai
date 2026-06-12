# Crackiv 🚀

![Tauri](https://img.shields.io/badge/Tauri-v2-24C8DB?style=for-the-badge&logo=tauri&logoColor=FFFFFF)
![React](https://img.shields.io/badge/React-19-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![TypeScript](https://img.shields.io/badge/TypeScript-5-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/Vite-7-B73BFE?style=for-the-badge&logo=vite&logoColor=FFD62E)
![Rust](https://img.shields.io/badge/Rust-Backend-000000?style=for-the-badge&logo=rust&logoColor=white)

A high-performance, cross-platform desktop application built with the modern web stack and Rust. 

Crackiv leverages **Tauri** to provide a lightweight native desktop experience while using **React** and **TypeScript** for a dynamic, robust frontend.

---

## ✨ Key Features

- **Cross-Platform Native Binaries**: Compile to Windows `.msi`/`.exe`, macOS `.app`/`.dmg`, and Linux `.deb`/`.AppImage`.
- **Ultra-Lightweight**: Uses the native OS webview (Edge WebView2, WebKit), drastically reducing app bundle size and RAM usage compared to Electron.
- **Secure by Default**: Core backend logic is isolated in Rust. Access to the OS is strictly managed via Tauri's IPC and plugin system.
- **Blazing Fast Development**: Powered by Vite for instant Hot Module Replacement (HMR).

---

## 🛠️ Technology Stack

| Layer | Technology | Description |
| :--- | :--- | :--- |
| **Frontend** | React 19 + TypeScript | UI components and application state |
| **Build Tool** | Vite 7 | Fast bundling and HMR development server |
| **Backend** | Rust + Tauri v2 | System-level operations and window management |
| **Styling** | Vanilla CSS / App.css | Default styling (can be extended to Tailwind/SASS) |

---

## 📋 Prerequisites

Ensure your system meets the requirements before starting development:

1. **Node.js**: `v18.x` or newer ([Download](https://nodejs.org/))
2. **Rust**: Latest stable compiler and cargo ([Install](https://www.rust-lang.org/tools/install))
3. **OS-Specific Build Dependencies**:
   - **Windows**: [Visual Studio C++ Build Tools](https://tauri.app/develop/prerequisites/windows) & Edge WebView2
   - **macOS**: CLang and macOS development dependencies (`xcode-select --install`)
   - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

---

## 🚀 Getting Started

### 1. Clone & Install
Clone the repository and install the Node.js frontend dependencies.
```bash
# Clone the repository
git clone https://github.com/your-username/crackiv.git
cd crackiv

# Install dependencies
npm install
```

### 2. Start Development Server
Starts the Vite dev server and opens the native Tauri window automatically.
```bash
npm run tauri dev
```

---

## 📜 Available Scripts

In the project directory, you can run:

- `npm run dev` - Starts only the frontend Vite server in your browser (no Rust backend/Tauri APIs).
- `npm run tauri dev` - Starts the full desktop app in development mode.
- `npm run tauri build` - Compiles the app and creates standalone installers/executables for your current OS.
- `npm run build` - Builds the frontend web assets into the `dist` directory.
- `npm run preview` - Locally previews the production build of the frontend.

---

## 📂 Project Structure

```text
crackiv/
├── src/                # React Frontend code (Components, Hooks, Pages)
│   ├── App.tsx         # Main React component
│   └── main.tsx        # React DOM entry point
├── src-tauri/          # Rust Backend & Tauri configuration
│   ├── src/            # Rust source code (main.rs, lib.rs)
│   ├── tauri.conf.json # Tauri configuration & permissions
│   └── Cargo.toml      # Rust package manifests
├── public/             # Static assets
└── vite.config.ts      # Vite configuration
```

---

## 🔌 Calling Rust from React

Tauri allows you to write heavy logic in Rust and call it from the frontend.

**Rust (`src-tauri/src/lib.rs`):**
```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

**React (`src/App.tsx`):**
```typescript
import { invoke } from "@tauri-apps/api/core";

// Call the Rust command
const response = await invoke("greet", { name: "User" });
```

---

## 🆘 Troubleshooting & FAQ

- **Rust build is failing on Windows**: Make sure you have installed the "C++ build tools" workload in the Visual Studio Installer.
- **Tauri APIs aren't working in the browser**: Tauri IPC commands only work when running inside the Tauri window (`npm run tauri dev`), not in a standard web browser.
- **How do I change the App icon?**: Replace the icons in the `src-tauri/icons/` folder and update `tauri.conf.json`.

---

## 🤝 Contributing

Contributions are always welcome! 
1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
