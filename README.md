# ModPilot

Gerenciador de mods para jogos Steam no Linux.

## Objetivo

Facilitar a instalação e gerenciamento de mods em jogos Steam no Linux, automatizando a configuração de prefixos Wine/Proton e a instalação de dependências como `winetricks`.

## Status

🚧 Em desenvolvimento — CLI funcional, GUI planejada.

## Funcionalidades planejadas

- [x] Menu interativo CLI
- [ ] Detecção automática de jogos Steam instalados
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
- Steam instalado
- Wine / Proton (para instalação de dependências)

## Licença

MIT
