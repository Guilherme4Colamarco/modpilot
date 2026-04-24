#!/usr/bin/env bash
# submit_feedback.sh — Envia feedback/sugestão via commit + Pull Request automático
# Requer: git, gh (GitHub CLI) — instale com: sudo pacman -S github-cli

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  ██████╗  █████╗ ███╗   ███╗███████╗██████╗ ██╗██╗      ██████╗ ████████╗"
echo "  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██╔══██╗██║██║     ██╔═══██╗╚══██╔══╝"
echo "  ██║  ███╗███████║██╔████╔██║█████╗  ██████╔╝██║██║     ██║   ██║   ██║   "
echo "  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ██╔═══╝ ██║██║     ██║   ██║   ██║   "
echo "  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗██║     ██║███████╗╚██████╔╝   ██║   "
echo "   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝     ╚═╝╚══════╝ ╚═════╝    ╚═╝   "
echo -e "${RESET}"
echo -e "${BOLD}  Ferramenta de Feedback & Sugestões${RESET}"
echo ""

# ── 1. Verificar se estamos em um repositório git ──────────────────────────────
if ! git rev-parse --is-inside-work-tree &>/dev/null; then
  echo -e "${RED}✗ Este diretório não é um repositório Git.${RESET}"
  exit 1
fi

# ── 2. Verificar configuração do git ──────────────────────────────────────────
GIT_USER=$(git config --global user.name 2>/dev/null || echo "")
GIT_EMAIL=$(git config --global user.email 2>/dev/null || echo "")

if [[ -z "$GIT_USER" || -z "$GIT_EMAIL" ]]; then
  echo -e "${RED}✗ Git não está configurado com nome/email.${RESET}"
  echo ""
  echo -e "Configure com:"
  echo -e "  ${YELLOW}git config --global user.name \"Seu Nome\"${RESET}"
  echo -e "  ${YELLOW}git config --global user.email \"seu@email.com\"${RESET}"
  exit 1
fi

echo -e "${GREEN}✓ Git configurado como:${RESET} ${BOLD}${GIT_USER}${RESET} <${GIT_EMAIL}>"
echo ""

# ── 3. Verificar gh CLI ────────────────────────────────────────────────────────
if ! command -v gh &>/dev/null; then
  echo -e "${RED}✗ GitHub CLI (gh) não encontrado.${RESET}"
  echo -e "  Instale com: ${YELLOW}sudo pacman -S github-cli${RESET} (ou equivalente)"
  echo -e "  Depois autentique: ${YELLOW}gh auth login${RESET}"
  exit 1
fi

if ! gh auth status &>/dev/null; then
  echo -e "${RED}✗ GitHub CLI não está autenticado.${RESET}"
  echo -e "  Execute: ${YELLOW}gh auth login${RESET}"
  exit 1
fi

echo -e "${GREEN}✓ GitHub CLI autenticado.${RESET}"
echo ""

# ── 4. Coletar informações do usuário ─────────────────────────────────────────
echo -e "${BOLD}📝 Tipo de contribuição:${RESET}"
echo "  1) 🐛 Bug Report"
echo "  2) 💡 Sugestão de Funcionalidade"
echo "  3) 📝 Outro"
echo ""
read -rp "  Escolha [1-3]: " TYPE_CHOICE

case "$TYPE_CHOICE" in
  1) LABEL="bug";         TYPE_PREFIX="fix";;
  2) LABEL="enhancement"; TYPE_PREFIX="feat";;
  3) LABEL="feedback";    TYPE_PREFIX="chore";;
  *) LABEL="feedback";    TYPE_PREFIX="chore";;
esac

echo ""
read -rp "$(echo -e "${BOLD}Título do PR${RESET} (curto e descritivo): ")" PR_TITLE

if [[ -z "$PR_TITLE" ]]; then
  echo -e "${RED}✗ Título não pode ser vazio.${RESET}"
  exit 1
fi

echo ""
echo -e "${BOLD}Descrição detalhada${RESET} (pressione Enter duas vezes para finalizar):"
DESCRIPTION=""
while IFS= read -r line; do
  [[ -z "$line" && -z "${PREV_LINE}" ]] && break
  DESCRIPTION+="${line}"$'\n'
  PREV_LINE="$line"
done
echo ""

# ── 5. Criar branch a partir do título ────────────────────────────────────────
BRANCH_NAME="${TYPE_PREFIX}/$(echo "$PR_TITLE" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | cut -c1-50)"
BRANCH_NAME="${BRANCH_NAME%-}"  # remove trailing dash

CURRENT_BRANCH=$(git branch --show-current)
DEFAULT_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | awk '{print $NF}' || echo "main")

echo -e "${CYAN}→ Criando branch:${RESET} ${BOLD}${BRANCH_NAME}${RESET}"
git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME"

# ── 6. Adicionar mudanças e commitar ──────────────────────────────────────────
git add -A

if git diff --cached --quiet; then
  echo -e "${YELLOW}⚠ Nenhuma mudança detectada para commitar.${RESET}"
  echo -e "  Edite o FEEDBACK.md com seu feedback antes de rodar este script."
  git checkout "$CURRENT_BRANCH"
  git branch -d "$BRANCH_NAME" 2>/dev/null || true
  exit 1
fi

COMMIT_MSG="${TYPE_PREFIX}: ${PR_TITLE}"
echo -e "${CYAN}→ Commitando:${RESET} ${BOLD}${COMMIT_MSG}${RESET}"
git commit -m "$COMMIT_MSG" -m "$DESCRIPTION"

# ── 7. Push da branch ─────────────────────────────────────────────────────────
echo -e "${CYAN}→ Enviando branch para o GitHub...${RESET}"
git push -u origin "$BRANCH_NAME"

# ── 8. Criar Pull Request ─────────────────────────────────────────────────────
echo -e "${CYAN}→ Criando Pull Request...${RESET}"

PR_BODY="## ${PR_TITLE}

${DESCRIPTION}

---
*Enviado por **${GIT_USER}** via \`submit_feedback.sh\` do GamePiLot*"

PR_URL=$(gh pr create \
  --title "$PR_TITLE" \
  --body "$PR_BODY" \
  --base "$DEFAULT_BRANCH" \
  --head "$BRANCH_NAME" \
  --label "$LABEL" 2>/dev/null || \
  gh pr create \
  --title "$PR_TITLE" \
  --body "$PR_BODY" \
  --base "$DEFAULT_BRANCH" \
  --head "$BRANCH_NAME")

echo ""
echo -e "${GREEN}${BOLD}✅ Pull Request criado com sucesso!${RESET}"
echo -e "   ${PR_URL}"
echo ""

# Voltar para a branch original
git checkout "$CURRENT_BRANCH"
