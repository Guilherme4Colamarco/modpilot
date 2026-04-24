<div align="center">

# 🎮 GamePiLot

**A Linux mod manager for Windows games running through Wine/Proton**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/UI-GTK4-green.svg)](https://gtk.org/)

---

🌐 **Language / Idioma / Lingua:**
[🇧🇷 Português](#-português-brasil) · [🇺🇸 English](#-english) · [🇪🇸 Español](#-español) · [🇮🇹 Italiano](#-italiano) · [🇷🇺 Русский](#-русский)



</div>

---

## 🇧🇷 Português (Brasil)

> [!WARNING]
> ⚠️ **AVISO: Projeto Experimental — Vibe Coding**
>
> Este projeto está sendo desenvolvido com **vibe coding** em **Rust e GTK** por alguém que **não tem experiência precisa** nessas tecnologias.
> O código pode conter más práticas, padrões não idiomáticos e soluções improvisadas.
> **Use por sua conta e risco.** Contribuições e correções são muito bem-vindas!

### O que é o GamePiLot?

O **GamePiLot** é um gerenciador de mods para Linux, projetado para facilitar a instalação e execução de ferramentas de modding de jogos Windows que rodam via **Wine** ou **Proton**. Ele detecta automaticamente seus jogos instalados e configura as dependências necessárias no prefixo Wine correto.

### ✨ Funcionalidades

- 🔍 **Detecção automática de jogos** — Escaneia Steam, Heroic (GOG, Epic, Amazon) e outros launchers
- 📦 **Instalação de dependências** — Instala winetricks, VCRedist, .NET e outras dependências automaticamente
- 🛠️ **Ferramentas de modding suportadas:**
  - SKSE64 (Script Extender para Skyrim)
  - Vortex Mod Manager / Mod Organizer 2
  - Simple Mod Framework (Hitman 3)
  - Cheat Engine / WeMod / Treinadores
  - ReShade, OpenIV, ScriptHookV
- 🍷 **Gerenciamento de prefixo Wine** — Detecta e configura prefixos automaticamente
- 🌍 **Interface multilíngue** — Inglês, Português, Espanhol, Italiano e Russo
- 🔔 **Alertas de erro inteligentes** — Filtra ruído do Wine e exibe apenas erros reais

### 📦 Dependências

```bash
# Arch Linux / CachyOS
sudo pacman -S gtk4 wine winetricks

# Ubuntu / Debian
sudo apt install libgtk-4-dev wine winetricks
```

### 🚀 Como compilar e rodar

```bash
git clone https://github.com/seu-usuario/gamepilot.git
cd gamepilot
cargo run
```

### 🎮 Jogos suportados

| Jogo | Ferramentas |
|------|------------|
| The Elder Scrolls V: Skyrim AE | SKSE64, Vortex, MO2 |
| Grand Theft Auto V | OpenIV, ScriptHookV |
| Dishonored / Dishonored 2 | Cheat Engine, WeMod, ReShade |
| HITMAN World of Assassination | Simple Mod Framework |

### 💬 Feedback e Sugestões

Leia o [FEEDBACK.md](FEEDBACK.md) ou use o script automático:
```bash
./scripts/submit_feedback.sh
```

---

## 🇺🇸 English

> [!WARNING]
> ⚠️ **WARNING: Experimental Project — Vibe Coding**
>
> This project is being developed using **vibe coding** in **Rust and GTK** by someone with **no precise experience** in these technologies.
> The code may contain bad practices, non-idiomatic patterns, and improvised solutions.
> **Use at your own risk.** Contributions and fixes are very welcome!

### What is GamePiLot?

**GamePiLot** is a Linux mod manager designed to simplify the installation and execution of Windows game modding tools running through **Wine** or **Proton**. It automatically detects your installed games and configures the required dependencies in the correct Wine prefix.

### ✨ Features

- 🔍 **Automatic game detection** — Scans Steam, Heroic (GOG, Epic, Amazon) and other launchers
- 📦 **Dependency installation** — Automatically installs winetricks, VCRedist, .NET, and other dependencies
- 🛠️ **Supported modding tools:**
  - SKSE64 (Script Extender for Skyrim)
  - Vortex Mod Manager / Mod Organizer 2
  - Simple Mod Framework (Hitman 3)
  - Cheat Engine / WeMod / Trainers
  - ReShade, OpenIV, ScriptHookV
- 🍷 **Wine prefix management** — Automatically detects and configures Wine prefixes
- 🌍 **Multilingual UI** — English, Portuguese, Spanish, Italian and Russian
- 🔔 **Smart error alerts** — Filters Wine noise and shows only real errors

### 📦 Dependencies

```bash
# Arch Linux / CachyOS
sudo pacman -S gtk4 wine winetricks

# Ubuntu / Debian
sudo apt install libgtk-4-dev wine winetricks
```

### 🚀 Build and run

```bash
git clone https://github.com/your-user/gamepilot.git
cd gamepilot
cargo run
```

### 🎮 Supported games

| Game | Tools |
|------|-------|
| The Elder Scrolls V: Skyrim AE | SKSE64, Vortex, MO2 |
| Grand Theft Auto V | OpenIV, ScriptHookV |
| Dishonored / Dishonored 2 | Cheat Engine, WeMod, ReShade |
| HITMAN World of Assassination | Simple Mod Framework |

### 💬 Feedback & Suggestions

Read [FEEDBACK.md](FEEDBACK.md) or use the automatic script:
```bash
./scripts/submit_feedback.sh
```

---

## 🇪🇸 Español

> [!WARNING]
> ⚠️ **ADVERTENCIA: Proyecto Experimental — Vibe Coding**
>
> Este proyecto está siendo desarrollado con **vibe coding** en **Rust y GTK** por alguien que **no tiene experiencia precisa** en estas tecnologías.
> El código puede contener malas prácticas, patrones no idiomáticos y soluciones improvisadas.
> **Úsalo bajo tu propio riesgo.** ¡Las contribuciones y correcciones son muy bienvenidas!

### ¿Qué es GamePiLot?

**GamePiLot** es un gestor de mods para Linux diseñado para simplificar la instalación y ejecución de herramientas de modding de juegos Windows que se ejecutan a través de **Wine** o **Proton**. Detecta automáticamente tus juegos instalados y configura las dependencias necesarias en el prefijo de Wine correcto.

### ✨ Características

- 🔍 **Detección automática de juegos** — Escanea Steam, Heroic (GOG, Epic, Amazon) y otros launchers
- 📦 **Instalación de dependencias** — Instala winetricks, VCRedist, .NET y otras dependencias automáticamente
- 🛠️ **Herramientas de modding soportadas:**
  - SKSE64 (Script Extender para Skyrim)
  - Vortex Mod Manager / Mod Organizer 2
  - Simple Mod Framework (Hitman 3)
  - Cheat Engine / WeMod / Entrenadores
  - ReShade, OpenIV, ScriptHookV
- 🍷 **Gestión de prefijos Wine** — Detecta y configura prefijos automáticamente
- 🌍 **Interfaz multilingüe** — Inglés, Portugués, Español, Italiano y Ruso
- 🔔 **Alertas de error inteligentes** — Filtra el ruido de Wine y muestra solo errores reales

### 📦 Dependencias

```bash
# Arch Linux / CachyOS
sudo pacman -S gtk4 wine winetricks

# Ubuntu / Debian
sudo apt install libgtk-4-dev wine winetricks
```

### 🚀 Compilar y ejecutar

```bash
git clone https://github.com/tu-usuario/gamepilot.git
cd gamepilot
cargo run
```

### 🎮 Juegos soportados

| Juego | Herramientas |
|-------|-------------|
| The Elder Scrolls V: Skyrim AE | SKSE64, Vortex, MO2 |
| Grand Theft Auto V | OpenIV, ScriptHookV |
| Dishonored / Dishonored 2 | Cheat Engine, WeMod, ReShade |
| HITMAN World of Assassination | Simple Mod Framework |

### 💬 Comentarios y Sugerencias

Lea [FEEDBACK.md](FEEDBACK.md) o use el script automático:
```bash
./scripts/submit_feedback.sh
```

---

## 🇮🇹 Italiano

> [!WARNING]
> ⚠️ **ATTENZIONE: Progetto Sperimentale — Vibe Coding**
>
> Questo progetto è sviluppato con **vibe coding** in **Rust e GTK** da qualcuno che **non ha un'esperienza precisa** in queste tecnologie.
> Il codice potrebbe contenere cattive pratiche, pattern non idiomatici e soluzioni improvvisate.
> **Usalo a tuo rischio.** Contributi e correzioni sono benvenuti!

### Cos'è GamePiLot?

**GamePiLot** è un gestore di mod per Linux progettato per semplificare l'installazione e l'esecuzione di strumenti di modding per giochi Windows eseguiti tramite **Wine** o **Proton**. Rileva automaticamente i giochi installati e configura le dipendenze necessarie nel prefisso Wine corretto.

### ✨ Funzionalità

- 🔍 **Rilevamento automatico dei giochi** — Scansiona Steam, Heroic (GOG, Epic, Amazon) e altri launcher
- 📦 **Installazione delle dipendenze** — Installa automaticamente winetricks, VCRedist, .NET e altre dipendenze
- 🛠️ **Strumenti di modding supportati:**
  - SKSE64 (Script Extender per Skyrim)
  - Vortex Mod Manager / Mod Organizer 2
  - Simple Mod Framework (Hitman 3)
  - Cheat Engine / WeMod / Trainer
  - ReShade, OpenIV, ScriptHookV
- 🍷 **Gestione del prefisso Wine** — Rileva e configura i prefissi Wine automaticamente
- 🌍 **Interfaccia multilingua** — Inglese, Portoghese, Spagnolo, Italiano e Russo
- 🔔 **Avvisi di errore intelligenti** — Filtra il rumore di Wine e mostra solo gli errori reali

### 📦 Dipendenze

```bash
# Arch Linux / CachyOS
sudo pacman -S gtk4 wine winetricks

# Ubuntu / Debian
sudo apt install libgtk-4-dev wine winetricks
```

### 🚀 Compilare ed eseguire

```bash
git clone https://github.com/tuo-utente/gamepilot.git
cd gamepilot
cargo run
```

### 🎮 Giochi supportati

| Gioco | Strumenti |
|-------|-----------|
| The Elder Scrolls V: Skyrim AE | SKSE64, Vortex, MO2 |
| Grand Theft Auto V | OpenIV, ScriptHookV |
| Dishonored / Dishonored 2 | Cheat Engine, WeMod, ReShade |
| HITMAN World of Assassination | Simple Mod Framework |

### 💬 Feedback e Suggerimenti

Leggi [FEEDBACK.md](FEEDBACK.md) o usa lo script automatico:
```bash
./scripts/submit_feedback.sh
```

---

## 🇷🇺 Русский

> [!WARNING]
> ⚠️ **ПРЕДУПРЕЖДЕНИЕ: Экспериментальный проект — Vibe Coding**
>
> Этот проект разрабатывается с использованием **vibe coding** на **Rust и GTK** кем-то, кто **не имеет точного опыта** в этих технологиях.
> Код может содержать плохие практики, неидиоматические паттерны и импровизированные решения.
> **Используйте на свой страх и риск.** Вклады и исправления приветствуются!

### Что такое GamePiLot?

**GamePiLot** — это менеджер модов для Linux, предназначенный для упрощения установки и запуска инструментов для моддинга Windows-игр, работающих через **Wine** или **Proton**. Он автоматически определяет установленные игры и настраивает необходимые зависимости в правильном префиксе Wine.

### ✨ Возможности

- 🔍 **Автоматическое обнаружение игр** — сканирует Steam, Heroic (GOG, Epic, Amazon) и другие лаунчеры
- 📦 **Установка зависимостей** — автоматически устанавливает winetricks, VCRedist, .NET и другие зависимости
- 🛠️ **Поддерживаемые инструменты моддинга:**
  - SKSE64 (Script Extender для Skyrim)
  - Vortex Mod Manager / Mod Organizer 2
  - Simple Mod Framework (Hitman 3)
  - Cheat Engine / WeMod / Трейнеры
  - ReShade, OpenIV, ScriptHookV
- 🍷 **Управление префиксами Wine** — автоматическое обнаружение и настройка префиксов Wine
- 🌍 **Многоязычный интерфейс** — английский, португальский, испанский, итальянский и русский
- 🔔 **Умные оповещения об ошибках** — фильтрует шум Wine и показывает только реальные ошибки

### 📦 Зависимости

```bash
# Arch Linux / CachyOS
sudo pacman -S gtk4 wine winetricks

# Ubuntu / Debian
sudo apt install libgtk-4-dev wine winetricks
```

### 🚀 Сборка и запуск

```bash
git clone https://github.com/vash-polzovatel/gamepilot.git
cd gamepilot
cargo run
```

### 🎮 Поддерживаемые игры

| Игра | Инструменты |
|-------|-----------|
| The Elder Scrolls V: Skyrim AE | SKSE64, Vortex, MO2 |
| Grand Theft Auto V | OpenIV, ScriptHookV |
| Dishonored / Dishonored 2 | Cheat Engine, WeMod, ReShade |
| HITMAN World of Assassination | Simple Mod Framework |

### 💬 Обратная связь и предложения

Прочтите [FEEDBACK.md](FEEDBACK.md) или используйте автоматический скрипт:
```bash
./scripts/submit_feedback.sh
```

---

<div align="center">

**GamePiLot** — Made with ❤️ for the Linux gaming community

[📋 Changelog](CHANGELOG.md) · [💬 Feedback](FEEDBACK.md) · [🐛 Issues](../../issues)

</div>
