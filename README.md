# 💰 Finance Manager

A personal finance management tool written in Rust with an intuitive CLI interface.

## ✨ Features

- 💵 Track income and expenses
- 📊 View account balance
- 📋 List all transactions
- 🗂️ Categorize transactions
- 💾 SQLite database for reliability
- 🎯 Interactive UI with arrow navigation
- 🚀 Fast and lightweight

## 🚀 Installation

### Option 1: Quick Install (Recommended)
```bash
./install.sh
```

### Option 2: Manual Install
```bash
cargo install --path .
```

### Option 3: Local Development
```bash
cargo run
```

## 🎯 Usage

### Basic Commands
```bash
# Start the application (production mode)
finance

# Development mode (with sample data)
cargo run

# Force production mode (if needed)
RUST_ENV=production cargo run
```

### Development vs Production Mode

The application automatically detects the running mode:

- **Development Mode**: When running with `cargo run` or from the development directory
  - Uses in-memory database (resets on restart)
  - Loads sample data for testing
  - Shows development indicator

- **Production Mode**: When running installed binary
  - Uses persistent SQLite database
  - Starts with empty database
  - Clean interface without development indicators

### Interface Navigation
- **Arrow keys** (↑/↓): Navigate options
- **Enter**: Select option
- **Ctrl+C**: Exit application

## 🔧 Development

### Prerequisites
- Rust 1.70+
- Cargo

### Build
```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Test
```bash
cargo test
```

### Using Make/Just
```bash
# With Make
make build-release
make install-local

# With Just (if installed)
just build-release
just install
```

## 📦 Release Management

### Version Bumping
```bash
# Patch version (0.1.0 -> 0.1.1)
make bump-patch

# Minor version (0.1.0 -> 0.2.0)
make bump-minor
```

### Creating Releases
```bash
# Create packaged release
make release

# Create distribution package
make package
```

## 🗄️ Database

The application uses SQLite for data storage:

- **Development**: In-memory database (resets on restart)
- **Production**: Persistent `finance.db` file

### Backup Database
```bash
make backup-db
```

## 🔐 Data Privacy

- All data is stored locally on your machine
- No data is sent to external servers
- SQLite database is portable and can be backed up easily

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details.

## 🛠️ Technical Details

- **Language**: Rust
- **Database**: SQLite
- **UI**: dialoguer for interactive CLI
- **Build**: Cargo with optimized release profile

## 🚧 Roadmap

- [ ] Export to CSV/JSON
- [ ] Monthly/yearly reports
- [ ] Budget tracking
- [ ] Recurring transactions
- [ ] Multi-currency support
- [ ] Data visualization

## 📞 Support

For issues or questions, please open an issue on GitHub.

---

Made with ❤️ in Rust
