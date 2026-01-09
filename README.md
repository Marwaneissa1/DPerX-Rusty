# DPerX - Rusty

The proud successor to DPerX - Reborn, built with cutting-edge technology for maximum performance and reliability.

## Overview

DPerX - Rusty represents the next evolution in game enhancement tools, combining the power of Rust and modern web technologies to deliver a seamless, high-performance experience. Built from the ground up with Tauri and Angular, this application offers unmatched speed, security, and customization.

## Core Features

### Combat Enhancements

- **Aimbot Weapons** - Precision targeting system for all weapon types
- **Aimbot Hook** - Advanced hook assistance with predictive algorithms
- **Predictions** - Smart prediction engine for enhanced accuracy
- **Auto Fire** - Intelligent automatic firing system
- **Auto Hammer** - Automated hammer mechanics

### Visual Enhancements

- **ESP (Extra Sensory Perception)** - Comprehensive visual information overlay
- **Custom UI** - Modern, customizable interface with dark theme

### Gameplay Tools

- **Balancer** - Advanced game balancing features
- **Auto Tower** - Automated tower building system
- **Key Bindings** - Fully customizable hotkey system

### Security & Privacy

- **Local Spoofer** - Advanced local identity protection
- **Undetectable** - Operates under the radar unless manually reported
- **Auto-Update Offsets** - Automatic offset updates when available

## Technical Stack

Built with modern, high-performance technologies:

- **Rust** - Core backend for maximum performance and memory safety
- **Tauri** - Lightweight, secure desktop application framework
- **Angular** - Reactive, component-based frontend architecture
- **Native CSS** - Custom design system with 60-30-10 color palette

## Key Advantages

### Performance

- Lightning-fast startup and operation
- Minimal memory footprint
- Native performance through Rust backend

### Security

- Sandboxed execution environment
- No external dependencies
- Secure by design

### Customization

- Fully customizable interface
- Flexible key binding system
- Modular feature configuration

## Getting Started

### Prerequisites

- Node.js 18+ and pnpm
- Rust toolchain
- Windows 10/11

### Installation

1. Clone the repository

```bash
git clone https://github.com/kiocode/DPerX-Rusty.git
cd DPerX-Rusty
```

2. Install dependencies

```bash
pnpm install
```

3. Run in development mode

```bash
pnpm run tauri dev
```

4. Build for production

```bash
pnpm run tauri build
```

## Configuration

The application features three main configuration pages:

- **Options1** - General settings and preferences
- **Options2** - Network and performance tuning
- **Options3** - Advanced features and experimental options

All settings are accessible through the intuitive sidebar navigation.

## Development

### Project Structure

```
DPerX-Rusty/
├── src/                    # Angular frontend
│   ├── app/
│   │   ├── components/     # Reusable UI components
│   │   ├── pages/          # Application pages
│   │   └── models/         # Data models
│   └── styles.css          # Global styles
├── src-tauri/              # Rust backend
│   ├── src/
│   │   └── lib.rs          # Core logic
│   └── tauri.conf.json     # Tauri configuration
└── README.md
```

### Code Formatting

```bash
pnpm run format
```

## Features Roadmap

- ✅ Custom titlebar with window controls
- ✅ Modern UI with glassmorphism effects
- ✅ Atomic component architecture
- ✅ Configurable options system
- 🔄 Offset auto-update system
- 🔄 Advanced prediction algorithms
- 🔄 Enhanced ESP features

## License

This project is for educational purposes only. Use responsibly and at your own risk.

## Credits

Developed by **kiocode**

---

_DPerX - Rusty: Performance meets precision_
