# Zap

A minimalist time tracking desktop app built with Tauri + Vue 3.

## Features

- **Task Timer** - Start/stop timer for tasks, track time per session
- **Today Focus** - Set ONE daily highlight task (The Daily Highlight methodology)
- **Categories** - Organize tasks with color-coded categories
- **Statistics** - View time stats by today/week/month
- **System Tray** - Close to tray, quick access from menu bar

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Vue 3 + TypeScript + Naive UI |
| Backend | Rust + Tauri 2 |
| Database | SQLite + sqlx |
| Build | Vite |

## Development

### Prerequisites

- Node.js 18+
- pnpm
- Rust 1.92+

### Setup

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Project Structure

```
zap/
├── src/                    # Frontend (Vue)
│   ├── components/         # Vue components
│   ├── composables/        # Vue composables
│   └── App.vue
├── src-tauri/              # Backend (Rust)
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── sqlite.rs       # Database setup
│   │   └── lib.rs          # App entry
│   └── migrations/         # SQL migrations
└── package.json
```

## License

MIT OR Apache-2.0
