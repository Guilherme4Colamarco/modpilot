---
name: uniube-whatsapp-monitor
description: Monitora os grupos de WhatsApp do curso CST IA da Uniube — extrai mensagens, verifica tarefas, avisos de provas e alertas acadêmicos. Use quando o usuário pedir para "monitorar grupos", "verificar WhatsApp", "tem mensagem no grupo?", "o que rolou no grupo?", ou qualquer variação de monitoramento dos grupos de WhatsApp relacionados ao curso.
roles: [all]
integrations: [browseros-mcp]
---

# Uniube WhatsApp Monitor

Monitora os grupos de WhatsApp do curso CST IA da Uniube via browser e gera relatório estruturado com tarefas, avisos e alertas acadêmicos.

## Contexto

- **Grupos a monitorar:**
  1. **CST IA Uniube** — Grupo principal do curso (contato direto com colegas)
  2. **Inteligência Artificial e Ciência de Dados** — Grupo da disciplina específica
- **Web:** https://web.whatsapp.com/
- **Status esperado:** Usuário normalmente já está logado

## Grupos Conhecidos

O usuário participa de 2 grupos principais do curso CST IA:

### Grupo 1: CST IA Uniube
- **Última atividade:** ~16/04/2026 (quarta-feira)
- **Participantes:** Dener, Florisvaldo, José, Kayky, William, Loh, Victor Marcelino e outros
- **Tipo:** Grupo geral do curso

### Grupo 2: Inteligência Artificial e Ciência de Dados
- **Última atividade:** ~10/04/2026 (quinta-feira)
- **Participantes:** Thiago Henrique, José Victor e outros
- **Tipo:** Grupo da disciplina específica

## Fluxo de Execução

### 1. Abrir WhatsApp Web

```
1. Abrir nova página: https://web.whatsapp.com/
2. Aguardar carregamento (sleep 3s)
3. Tirar snapshot — verificar se já está logado
```

### 2. Acessar grupo CST IA Uniube

```
1. No menu lateral, encontrar "CST IA Uniube" (pode buscar com ctrl+f)
2. Clicar no grupo
3. Aguardar (sleep 2s)
```

### 3. Extrair mensagens

Usar `evaluate_script` para pegar mensagens:

```javascript
(() => {
  const out = [];
  document.querySelectorAll('span[class*="copyable-text"]').forEach(el => {
    const pre = el.getAttribute('data-pre-plain-text') || '';
    const text = el.innerText?.trim() || '';
    if (text) out.push((pre ? pre.replace(/[\[\]]/g,'').trim() + ' → ' : '') + text);
  });
  return out.slice(0, 50).join('\n');
})();
```

### 4. Repetir para grupo IA e Ciência de Dados

```
1. Voltar à lista de conversas
2. Clicar em "Inteligência Artificial e Ciência de Dados"
3. Extrair mensagens (mesmo método)
```

### 5. Analisar e categorizar

- **Tarefas:** Trabalhos, exercícios, entregas
- **Provas/Avals:** Datas de prova, محلالات
- **Avisos:** Alertas do professor, mudanças de schedule
- **Perguntas:** Dúvidas dos colegas

### 6. Gerar relatório

Salvar em `04-projects/uniube/MONITOR-WHATSAPP-YYYY-MM-DD.md`.

## Template do Relatório

```markdown
---
type: monitoring
created: YYYY-MM-DD
monitoring_date: YYYY-MM-DD
tags: ["#uniube", "#whatsapp", "#monitoramento"]
---

# Monitoramento WhatsApp — CST IA Uniube — DD/MM/YYYY

**Data/Hora:** [data e hora]
**Grupos monitorados:** [N]

---

## CST IA Uniube

**Última mensagem:** [data]
**Mensagens collectadas:** [N]

### Tarefas Identificadas
- [ ] [tarefa 1]
- [ ] [tarefa 2]

### Provas/Avals
- [prova 1]: [sala/local] — [data se conhecida]

### Avisos
- [aviso 1]
- [aviso 2]

---

## Inteligência Artificial e Ciência de Dados

**Última mensagem:** [data]
**Mensagens collectadas:** [N]

### Tarefas Identificadas
- [ ] [tarefa 1]

### Avisos
- [aviso 1]

---

## Resumo de Ações

- [ ] [a��ão 1] 📅 YYYY-MM-DD
- [ ] [ação 2] 📅 YYYY-MM-DD
```

## Informações Já Conhecidas (para referência rápida)

### Última Varredura (20/04/2026)

**CST IA Uniube:**
- Prova do Luciano → sala 2C01
- Trabalho manuscrito do Flavió → entregar segunda (no Disco Virtual)
- Exercícios Java 12++ = exercício 13

**Inteligência Artificial e Ciência de Dados:**
- Aula de Inteligência começa dia 23/04
- Exercícios de Java enviados
- Frequência em 99,1%

## Dicas de Navegação

- WhatsApp Web usa sessão activa — se logado, não precisa escanear QR
- Mensagens mais antigas precisam de scroll para carregar
- Use `take_screenshot` para visualizar contexto visual
- IDs de elementos são dinâmicos — sempre snapshot antes de clicar
- Buscar no grupo com ctrl+f funciona na lista de conversas

## Integração com Outras Skills

- Se houver tarefas/avisos importantes → sugerir `/braindump`
- Resultados disponíveis para `/weekly-checkin`
- Salvar relatório permite contexto sem re-acesso