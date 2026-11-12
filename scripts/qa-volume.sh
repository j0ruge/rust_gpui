#!/usr/bin/env bash
set -euo pipefail

# Somente dados artificiais, sempre em diretório novo e isolado.
repo_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
if [[ ! -x "$repo_dir/target/release/rust_gpui" ]]; then
    echo 'Compile primeiro com cargo build --locked --release.' >&2
    exit 1
fi

task_data_dir=$(mktemp -d /tmp/rust-gpui-volume.XXXXXX)
python3 - "$task_data_dir" <<'PY'
import sqlite3
import sys
from pathlib import Path

connection = sqlite3.connect(Path(sys.argv[1]) / "tasks.sqlite3")
connection.executescript("""
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'completed'))
);
PRAGMA user_version=1;
""")
connection.executemany(
    "INSERT INTO tasks (title,status) VALUES (?,?)",
    [(f"Tarefa pessoal {i:04}", "completed" if i % 2 == 0 else "pending")
     for i in range(1, 1001)],
)
connection.commit()
connection.close()
PY

printf 'Diretório isolado com 1.000 tarefas artificiais: %s\n' "$task_data_dir"
exec env RUST_GPUI_DATA_DIR="$task_data_dir" "$repo_dir/target/release/rust_gpui"
