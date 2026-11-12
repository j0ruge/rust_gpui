#!/usr/bin/env bash
# Exercita a janela GPUI em Xvfb, sem usar teclado/mouse do desktop do usuário.
# Uso: bash scripts/qa-headless.sh [opções de qa_headless.py]
set -euo pipefail

repo_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
for tool in xvfb-run Xvfb xauth openbox xdotool wmctrl import tesseract python3; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'Ferramenta de QA ausente: %s\n' "$tool" >&2
        exit 1
    fi
done
python3 -c 'from PIL import Image'

qa_icd=${RUST_GPUI_QA_ICD_FILE:-}
if [[ -z "$qa_icd" ]]; then
    shopt -s nullglob
    qa_candidates=(/usr/share/vulkan/icd.d/lvp_icd*.json)
    if (( ${#qa_candidates[@]} == 0 )); then
        echo 'Lavapipe não encontrado. Defina RUST_GPUI_QA_ICD_FILE para o ICD de software.' >&2
        exit 1
    fi
    qa_icd=${qa_candidates[0]}
fi
if [[ ! -f "$qa_icd" ]]; then
    printf 'ICD de QA não encontrado: %s\n' "$qa_icd" >&2
    exit 1
fi

exec xvfb-run -a -s '-screen 0 1280x960x24 -nolisten tcp' \
    env -u WAYLAND_DISPLAY -u VK_DRIVER_FILES VK_ICD_FILENAMES="$qa_icd" \
    RUST_GPUI_QA_HEADLESS=1 python3 "$repo_dir/scripts/qa_headless.py" "$@"
