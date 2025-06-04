# KadeIDE

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

KadeIDE is a lightweight code editor forked from VS Code, reimagined to deliver a fast, efficient coding experience with core functionality like IntelliSense and advanced AI-powered features. Built with Tauri instead of Electron, KadeIDE achieves a smaller footprint (targeting ~10-20MB binary size and ~30-50MB memory usage) while retaining essential editing capabilities. It supports flexible AI integration through APIs (e.g., Grok, OpenAI, Anthropic, Gemini, Mistral) and local models (via Ollama), inspired by tools like Cursor and Windsurf.

## ✨ Features

- **Lightweight Design**: Powered by Tauri, KadeIDE uses native webviews for minimal resource usage, making it ideal for low-end hardware.
- **IntelliSense**: Robust code completion, diagnostics, and hover info via the Language Server Protocol (LSP), supporting languages like TypeScript, JavaScript, and Python.
- **AI Integration**:
  - **API-Based**: Seamless integration with AI providers like Grok, OpenAI, Anthropic, Gemini, and Mistral for code suggestions, chat, and refactoring.
  - **Local Models**: Support for local AI models (e.g., CodeLLaMA, TinyLLaMA) via Ollama, with a model manager for easy downloads.
- **Customizable UI**: Simplified workbench inspired by VS Code, with a focus on core editing features and optional AI-driven sidebars.
- **Cross-Platform**: Runs on Windows, macOS, and Linux with a consistent experience.

## 🎯 Goals

KadeIDE aims to:

- Provide a lightweight alternative to VS Code, reducing memory and CPU usage.
- Offer flexible AI capabilities, allowing users to choose between cloud-based APIs or local models.
- Maintain core editing features like IntelliSense while stripping non-essential components (e.g., debuggers, Git integration).
- Build a modular AI provider system for easy extensibility.

## ⚡ Optimizations for Lightweight Performance

KadeIDE is optimized for minimal resource usage, targeting a binary size of ~10-20MB, memory usage of ~30-50MB, and startup time of ~1-2 seconds:

### Minimal Frontend
- Uses Monaco Editor with only essential languages (TypeScript, JavaScript, Python) to reduce bundle size.
- Employs lightweight UI frameworks (e.g., Preact or plain JavaScript) and purged Tailwind CSS for minimal CSS footprint.
- Minifies and compresses assets with esbuild and Brotli for smaller distribution.

### Tauri Backend
- Compresses binaries with UPX, reducing size by 30-50%.
- Minimizes Rust dependencies (e.g., uses ureq for HTTP, log for logging).
- Lazy-loads LSP servers and AI services to lower memory usage.
- Optionally uses WebAssembly for backend logic to further reduce binary size.

### Efficient IntelliSense
- Supports a limited set of LSP servers for core languages to minimize memory and disk usage.
- Runs LSP servers externally (e.g., system-installed typescript-language-server) to reduce binary size.
- Caches frequent LSP responses for faster performance.

### Optimized AI Integration
- Uses lightweight API clients (e.g., fetch or ureq) for AI providers like Grok and OpenAI.
- Supports small, quantized local models (e.g., TinyLLaMA with 4-bit quantization) via Ollama to reduce memory and disk usage.
- Dynamically loads AI providers to exclude unused code.

### Reduced Feature Set
- Removes non-essential VS Code features (e.g., debugging, Git integration, extensions marketplace) for a lean experience.
- Makes features like AI chat or additional languages optional via user settings.
- Includes minimal themes (e.g., vs-dark) to reduce CSS/JSON bloat.

### Fast Startup
- Lazy-loads resources (e.g., Monaco languages, LSP servers, AI models) until needed.
- Precompiles assets (e.g., Rust to WebAssembly, Vite pre-bundling) for faster initialization.
- Renders only the editor on startup, loading sidebars or AI panels on demand.

### Profiling and Testing
- Regularly profiles memory and CPU usage with tools like htop and Tauri dev tools.
- Tests on low-end hardware (e.g., 4GB RAM, no GPU) to ensure performance.
- Benchmarks against VS Code to validate improvements.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js**: Required for frontend dependencies (Monaco Editor)
- **Tauri Dependencies**: See [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for platform-specific requirements (e.g., WebKit2GTK on Linux)
- **Ollama** (optional): For local AI models, install [Ollama](https://ollama.ai/)
- **LSP Servers** (optional): Install language servers (e.g., `npm install -g typescript-language-server`) for IntelliSense

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/SHA888/KadeIDE.git
   cd KadeIDE
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

3. Install Rust dependencies:
   ```bash
   cargo install tauri-cli
   ```

4. Run in development mode:
   ```bash
   cargo tauri dev
   ```

5. Build for production:
   ```bash
   cargo tauri build
   ```

### Configuration

- **IntelliSense**: Configure LSP servers for your preferred languages in `settings.json`
- **AI Providers**:
  - **API-Based**: Add API keys for Grok, OpenAI, Anthropic, etc., in the settings UI or `config.json`
  - **Local Models**: Use the model manager to download models via Ollama (e.g., `ollama pull tinyllama`)
- **UI Customization**: Adjust the workbench layout via `settings.json` to enable/disable sidebars or AI features

## 💻 Usage

- **Editing**: Open files in the Monaco-based editor, with IntelliSense for supported languages
- **AI Features**:
  - Use inline code completion powered by AI providers (e.g., Grok for suggestions)
  - Access the AI chat sidebar for code explanations or refactoring
  - Switch between API-based and local models in the settings panel
- **Performance**: Monitor memory usage (~30-50MB target) and optimize by disabling unused features

## 🗺️ Roadmap

1. **Phase 1**: Port VS Code's core editor (Monaco) and IntelliSense (LSP) to Tauri
2. **Phase 2**: Implement AI integration with one provider (e.g., Grok) and local models via Ollama
3. **Phase 3**: Apply optimizations for binary size, memory, and startup time
4. **Phase 4**: Build a minimal extension system for custom plugins
5. **Phase 5**: Community-driven features and cross-platform testing

## 🤝 Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit changes: `git commit -m "Add your feature"`
4. Push to your fork: `git push origin feature/your-feature`
5. Open a pull request

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md) and check the [Contributing Guidelines](CONTRIBUTING.md) for details.

## 📄 License

KadeIDE is licensed under the MIT License, based on the original VS Code repository. Note that AI models and APIs may have their own licensing terms (e.g., check xAI's API for Grok).

## 🙏 Acknowledgments

- Built upon VS Code and Tauri
- Inspired by AI-driven editors like Cursor and Windsurf
- Thanks to the open-source community for tools like Monaco, LSP, and Ollama

## 📬 Contact

For questions, feedback, or collaboration, open an issue on [GitHub](https://github.com/SHA888/KadeIDE/issues) or reach out on [X](https://twitter.com/).

Happy coding with KadeIDE! 🚀
