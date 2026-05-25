# LangSwitcher

[🇷🇺 Русский](#русский) | [🇬🇧 English](#english)

---

## Русский

Утилита для автоматической конвертации текста между русской и английской раскладками клавиатуры. Работает в фоне, живёт в системном трее.

**Поддерживаемые платформы:** Windows 10, Windows 11, macOS

### Как это работает

1. Выделите текст, набранный не в той раскладке
2. Нажмите хоткей (по умолчанию `Win+Alt+C` на Windows / `Cmd+Option+C` на macOS)
3. Программа скопирует текст, автоматически переключит раскладку и вставит обратно

Программа определяет направление конвертации сама: если в тексте больше русских символов — конвертирует в английские, и наоборот.

### Зависимости проекта

В Rust-проектах нет аналога `requirements.txt` — все зависимости хранятся в двух файлах:

| Файл | Назначение |
|---|---|
| `src-tauri/Cargo.toml` | Список Rust-зависимостей (аналог `requirements.txt`) |
| `src-tauri/Cargo.lock` | Зафиксированные точные версии (аналог `requirements.lock`) |
| `package.json` | JavaScript-зависимости фронтенда |

`cargo` и `yarn` скачивают все зависимости автоматически при первой сборке.

### Системные требования для сборки

#### Все платформы

| Инструмент | Версия | Установка |
|---|---|---|
| Rust + Cargo | ≥ 1.85 | https://rustup.rs |
| Node.js | ≥ 18 | https://nodejs.org |
| Yarn | любая | `npm install -g yarn` |

#### Только Windows

- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) или Visual Studio с компонентом «Разработка классических приложений на C++»
- WebView2 (встроен в Windows 10/11, устанавливается автоматически Tauri при необходимости)

#### Только macOS

- Xcode Command Line Tools: `xcode-select --install`

### Установка и запуск

#### Вариант 1 — Готовый бинарник (только Windows)

Скачайте `langswitcher.exe` из репозитория и запустите. Установка не требуется.

#### Вариант 2 — Сборка из исходников

##### 1. Клонируйте репозиторий

```bash
git clone <url-репозитория>
cd LangSwitcher
```

##### 2. Установите JavaScript-зависимости

```bash
yarn install
```

##### 3. Запустите режим разработки (опционально)

```bash
yarn tauri dev
```

Откроется окно приложения с горячей перезагрузкой при изменении фронтенда.

##### 4. Соберите приложение

```bash
yarn tauri build
```

Готовый файл появится в:

| Платформа | Путь |
|---|---|
| Windows | `src-tauri/target/release/langswitcher.exe` |
| Windows (установщик) | `src-tauri/target/release/bundle/msi/` |
| macOS (.app) | `src-tauri/target/release/bundle/macos/` |
| macOS (.dmg) | `src-tauri/target/release/bundle/dmg/` |

### Настройка на macOS

#### Разрешение на доступ к специальным возможностям

`rdev` (библиотека перехвата клавиш) на macOS требует разрешения **Accessibility**. Без него хоткей не будет работать.

**При первом запуске macOS сама предложит выдать разрешение.** Если этого не произошло:

1. Откройте **Системные настройки** → **Конфиденциальность и безопасность** → **Универсальный доступ**
2. Нажмите `+` и добавьте `langswitcher` (или терминал при запуске через `tauri dev`)
3. Перезапустите приложение

### Настройки программы

Окно настроек открывается при запуске или кликом по иконке в трее.

| Параметр | Описание |
|---|---|
| **Активация** | Выбор третьей клавиши хоткея (C, S или L) |
| **Закрывать в трей** | Если включено — окно скрывается, программа продолжает работать |

#### Доступные хоткеи

| Windows | macOS |
|---|---|
| `Win + Alt + C` | `Cmd + Option + C` |
| `Win + Alt + S` | `Cmd + Option + S` |
| `Win + Alt + L` | `Cmd + Option + L` |

### Структура проекта

```text
LangSwitcher/
├── src/                    # Фронтенд (React + SCSS)
│   ├── index.jsx           # Окно настроек
│   └── index.scss
├── src-tauri/              # Бэкенд (Rust)
│   ├── src/main.rs         # Логика перехвата клавиш и конвертации
│   ├── Cargo.toml          # Rust-зависимости
│   ├── Cargo.lock          # Зафиксированные версии зависимостей
│   ├── tauri.conf.json     # Конфигурация Tauri
│   └── icons/              # Иконки приложения
├── public/                 # HTML-шаблон
├── package.json            # JavaScript-зависимости
└── webpack.config.js       # Конфигурация сборки фронтенда
```

### Решение проблем

**Хоткей не работает на macOS**
→ Проверьте разрешение Accessibility (см. раздел выше)

**Ошибка при сборке: `error: linker 'link.exe' not found`**
→ Установите Microsoft C++ Build Tools

**Ошибка: `cargo: command not found`**
→ Установите Rust через <https://rustup.rs> и перезапустите терминал

**Текст вставляется не туда или не вставляется совсем**
→ Убедитесь, что текст был выделён перед нажатием хоткея

---

## English

A utility for automatic text conversion between Russian and English keyboard layouts. Runs in the background and lives in the system tray.

**Supported platforms:** Windows 10, Windows 11, macOS

### How it works

1. Select text typed in the wrong layout
2. Press the hotkey (default: `Win+Alt+C` on Windows / `Cmd+Option+C` on macOS)
3. The app copies the text, converts the layout automatically, and pastes it back

The conversion direction is detected automatically: if the text contains more Russian characters it converts to English, and vice versa.

### Project dependencies

Rust projects have no `requirements.txt` equivalent — all dependencies are stored in two files:

| File | Purpose |
|---|---|
| `src-tauri/Cargo.toml` | Rust dependency list (equivalent to `requirements.txt`) |
| `src-tauri/Cargo.lock` | Pinned exact versions (equivalent to `requirements.lock`) |
| `package.json` | JavaScript frontend dependencies |

`cargo` and `yarn` download all dependencies automatically on the first build.

### Build requirements

#### All platforms

| Tool | Version | Install |
| --- | --- | --- |
| Rust + Cargo | ≥ 1.85 | <https://rustup.rs> |
| Node.js | ≥ 18 | <https://nodejs.org> |
| Yarn | any | `npm install -g yarn` |

#### Windows only

- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with the "Desktop development with C++" workload
- WebView2 (bundled with Windows 10/11, installed automatically by Tauri if missing)

#### macOS only

- Xcode Command Line Tools: `xcode-select --install`

### Installation and running

#### Option 1 — Pre-built binary (Windows only)

Download `langswitcher.exe` from the repository and run it. No installation required.

#### Option 2 — Build from source

##### 1. Clone the repository

```bash
git clone <repository-url>
cd LangSwitcher
```

##### 2. Install JavaScript dependencies

```bash
yarn install
```

##### 3. Run in development mode (optional)

```bash
yarn tauri dev
```

This opens the app window with hot reload on frontend changes.

##### 4. Build the application

```bash
yarn tauri build
```

The output files will be located at:

| Platform | Path |
|---|---|
| Windows | `src-tauri/target/release/langswitcher.exe` |
| Windows (installer) | `src-tauri/target/release/bundle/msi/` |
| macOS (.app) | `src-tauri/target/release/bundle/macos/` |
| macOS (.dmg) | `src-tauri/target/release/bundle/dmg/` |

### macOS setup

#### Accessibility permission

`rdev` (the keyboard interception library) requires **Accessibility** permission on macOS. Without it the hotkey will not work.

**macOS will prompt you to grant the permission on first launch.** If it did not:

1. Open **System Settings** → **Privacy & Security** → **Accessibility**
2. Click `+` and add `langswitcher` (or your terminal when running via `tauri dev`)
3. Restart the application

### App settings

The settings window opens on launch or by clicking the tray icon.

| Setting | Description |
|---|---|
| **Activation key** | Choose the third hotkey character (C, S, or L) |
| **Close to tray** | When enabled, closing the window hides it; the app keeps running |

#### Available hotkeys

| Windows | macOS |
| --- | --- |
| `Win + Alt + C` | `Cmd + Option + C` |
| `Win + Alt + S` | `Cmd + Option + S` |
| `Win + Alt + L` | `Cmd + Option + L` |

### Project structure

```text
LangSwitcher/
├── src/                    # Frontend (React + SCSS)
│   ├── index.jsx           # Settings window
│   └── index.scss
├── src-tauri/              # Backend (Rust)
│   ├── src/main.rs         # Hotkey interception and conversion logic
│   ├── Cargo.toml          # Rust dependencies
│   ├── Cargo.lock          # Pinned dependency versions
│   ├── tauri.conf.json     # Tauri configuration
│   └── icons/              # Application icons
├── public/                 # HTML template
├── package.json            # JavaScript dependencies
└── webpack.config.js       # Frontend build configuration
```

### Troubleshooting

**Hotkey does not work on macOS**
→ Grant Accessibility permission (see section above)

**Build error: `error: linker 'link.exe' not found`**
→ Install Microsoft C++ Build Tools

**Error: `cargo: command not found`**
→ Install Rust from <https://rustup.rs> and restart your terminal

**Text is pasted in the wrong place or not pasted at all**
→ Make sure the text was selected before pressing the hotkey
