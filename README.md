# ModPilot

Gerenciador de mods para jogos Steam e Heroic Games Launcher no Linux.

## Objetivo

Facilitar a instalação e gerenciamento de mods em jogos no Linux, com suporte a launchers como **Steam** e **Heroic** (Epic Games, GOG, Amazon Games), automatizando a configuração de prefixos Wine/Proton e a instalação de dependências via `winetricks`.

## Status

🚧 Em desenvolvimento — CLI funcional, GUI planejada.

## Funcionalidades planejadas

- [x] Menu interativo CLI
- [ ] Detecção automática de jogos Steam instalados
- [ ] Detecção automática de jogos Heroic (Epic, GOG, Amazon)
- [ ] Gerenciamento de mods por jogo
- [ ] Instalação de dependências Wine (`vcrun2022`, `dotnet48`, etc.)
- [ ] Interface gráfica GTK

## Como rodar

```bash
git clone https://github.com/Guilherme4Colamarco/modpilot.git
cd modpilot
cargo run
```

## Requisitos

- Rust (edition 2024)
- Steam e/ou Heroic Games Launcher instalados
- Wine / Proton (para instalação de dependências)

## Licença

MIT
