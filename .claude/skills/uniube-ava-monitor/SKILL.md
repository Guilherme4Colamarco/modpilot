---
name: uniube-ava-monitor
description: Monitora o AVA da Uniube (ava.uniube.br) via BrowserOS MCP — verifica atividades pendentes, comunicações, notas, frequência e avisos em todas as disciplinas do semestre ativo. Use quando o usuário pedir para "monitorar o AVA", "verificar atividades", "checar o AVA", "tem algo no AVA?", "o que tem no AVA hoje?", ou qualquer variação de monitoramento do ambiente virtual de aprendizagem da Uniube.
roles: [all]
integrations: [browseros-mcp]
---

# Uniube AVA Monitor

Monitora o AVA da Uniube via automação de browser (BrowserOS MCP) e gera relatório estruturado de atividades, comunicações e avisos.

## Contexto

- **URL:** https://ava.uniube.br → redireciona para https://ava3.uniube.br
- **RA:** Ler de `04-projects/uniube/PROJECT-OVERVIEW.md`
- **Credenciais:** O browser normalmente já tem a sessão salva (campo RA preenchido)
- **Disciplinas:** 8 matérias (ver PROJECT-OVERVIEW.md)

## Fluxo de Execução

### 1. Abrir e fazer login

```
1. Abrir nova página: https://ava.uniube.br
2. Aguardar carregamento (sleep 3s)
3. Tirar snapshot — verificar se já está logado ou se precisa de login
4. Se precisa login: campos "Usuário" e "Senha" já preenchidos pelo browser → clicar LOGIN
5. Aguardar (sleep 4s) e confirmar login bem-sucedido via screenshot
```

### 2. Acessar o curso

```
1. Clicar em "Meus cursos" no menu
2. Aguardar (sleep 2s)
3. Clicar em "Acessar" no card do curso INTELIGÊNCIA ARTIFICIAL E CIÊNCIA DE DADOS
4. Aguardar (sleep 3s)
```

### 3. Coletar dados das disciplinas

Na página de Disciplinas, extrair via `get_page_content`:
- Nome de cada disciplina (Teórica/Prática)
- Frequência (%)
- Atividades pendentes (formato `X/Y`)
- Notas disponíveis

### 4. Verificar Comunicações

```
1. Clicar em "Comunicação" no menu lateral
2. Filtrar por "Não Visualizado" para ver mensagens não lidas
3. Verificar abas: Gestão, Docente, Mentor
4. Coletar títulos, remetentes e datas das mensagens não lidas
```

### 5. Verificar Notificações

```
1. Clicar no ícone de notificações (sino) no topo
2. Ler o texto de cada notificação pendente
```

### 6. Verificar Questões Abertas (opcional)

Na página inicial do curso, verificar os contadores de:
- Questões discursivas abertas
- Questões objetivas abertas

### 7. Gerar relatório

Salvar em `04-projects/uniube/MONITOR-AVA-YYYY-MM-DD.md` usando o template abaixo.

## Template do Relatório

```markdown
---
type: monitoring
created: YYYY-MM-DD
monitoring_date: YYYY-MM-DD
tags: ["#uniube", "#ava", "#monitoramento"]
---

# Monitoramento AVA Uniube — DD/MM/YYYY

**Data/Hora:** [data e hora do monitoramento]
**Status Geral:** [✓ Em dia / ⚠️ Atenção necessária]
**Notificações:** [N ativas]

---

## Resumo das Disciplinas

| Disciplina | Tipo | Frequência | Atividades | Status |
|---|---|---|---|---|
| [nome] | Prática/Teórica | [%] | [X/Y] | ✓/⚠️ |

**Atividades pendentes total:** [N]

---

## Comunicações Não Lidas

[lista de mensagens ou "Nenhuma mensagem não lida"]

---

## Notificações Ativas

[lista de notificações ou "Nenhuma notificação"]

---

## Questões Abertas

- Discursivas: [N]
- Objetivas: [N]

---

## Ações Recomendadas

- [ ] [ação 1] 📅 YYYY-MM-DD
- [ ] [ação 2] 📅 YYYY-MM-DD
```

## Dicas de Navegação

- O AVA usa IDs dinâmicos nos elementos — sempre tirar `take_snapshot` antes de clicar
- Após navegação, aguardar `sleep 2-3s` antes do próximo snapshot
- Se a sessão expirar, as credenciais salvas no browser reautenticam automaticamente
- Menu lateral esquerdo: ícones mudam conforme o contexto (homepage vs. dentro do curso)
- Dentro do curso: menu superior com Disciplinas, Acadêmico, Comunicação, etc.

## Integração com Outras Skills

- Após monitorar, sugerir `/braindump` se houver atividades urgentes
- Resultados ficam disponíveis para `/weekly-checkin` e `/comprehensive-analysis`
- Salvar relatório permite que outros agentes (Qwen, Gemini) leiam o contexto sem precisar acessar o AVA novamente
