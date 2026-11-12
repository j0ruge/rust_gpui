#!/usr/bin/env python3
"""Regressões nativas de foco, validação e retry em um display Xvfb exclusivo."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import sqlite3
import subprocess
import tempfile
import time
import unicodedata

from PIL import Image


SCHEMA = """CREATE TABLE tasks (
id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL,
status TEXT NOT NULL CHECK (status IN ('pending','completed')));
PRAGMA user_version=1;"""
TASKS = [("Unica pendente", "pending"), ("Ja concluida", "completed")]
FOCUS_COLOR = (145, 186, 255)
CASES = (
    "focus_list", "focus_delete_button", "focus_checkbox", "focus_repeated_close",
    "focus_delete_last", "focus_reopen", "initial_invalid", "invalid_directory_retry", "initial_corrupt",
    "initial_locked_retry", "initial_second_instance", "invalid_title",
    "retry_create", "retry_rename", "hidden_selection", "retry_invalid_close",
)


def command(*args):
    """Executa uma ferramenta no display herdado, com falhas e timeout observáveis."""
    return subprocess.run(args, capture_output=True, text=True, check=True, timeout=15).stdout


def normalized(text):
    """Normaliza apenas o OCR para comparar rótulos, sem alterar dados da aplicação."""
    text = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode()
    return " ".join(re.sub(r"[^a-z0-9]+", " ", text.lower()).split())


def require_virtual_display():
    """Recusa execução direta no desktop; o wrapper cria um Xvfb novo via -a."""
    match = re.fullmatch(r":(\d+)(?:\.\d+)?", os.environ.get("DISPLAY", ""))
    if os.environ.get("RUST_GPUI_QA_HEADLESS") != "1" or not match:
        raise RuntimeError("Use bash scripts/qa-headless.sh; o desktop do usuário não é alvo de QA.")
    server_pid = int(Path(f"/tmp/.X{match[1]}-lock").read_text().strip())
    if Path(f"/proc/{server_pid}/comm").read_text().strip() != "Xvfb":
        raise RuntimeError("O display de QA não pertence a um servidor Xvfb.")
    return server_pid


class NativeQa:
    """Controla somente fixtures, processos e janelas do display virtual validado."""

    def __init__(self, binary, output):
        self.binary = binary
        self.output = output
        self.apps = []
        self.shot_number = 0

    def seed(self, directory, records=TASKS):
        """Cria um banco artificial novo, sem acessar os dados padrão do usuário."""
        directory.mkdir(parents=True)
        with sqlite3.connect(directory / "tasks.sqlite3") as connection:
            connection.executescript(SCHEMA)
            connection.executemany("INSERT INTO tasks(title,status) VALUES (?,?)", records)

    def launch(self, directory, label, override=None, error=False):
        """Abre o binário e aguarda a tela normal ou a tela de erro ser desenhada."""
        log = (self.output / f"{label}.log").open("w")
        environment = {**os.environ, "RUST_GPUI_DATA_DIR": str(directory) if override is None else override,
                       "VK_LOADER_DEBUG": "driver"}
        process = subprocess.Popen([str(self.binary)], env=environment, stdout=log, stderr=subprocess.STDOUT)
        log.close()
        self.apps.append(process)
        deadline = time.monotonic() + 20
        window = None
        while time.monotonic() < deadline:
            if process.poll() is not None:
                raise RuntimeError(f"Aplicação encerrou na abertura: {process.returncode}; veja {label}.log")
            found = subprocess.run(["xdotool", "search", "--pid", str(process.pid)], capture_output=True, text=True)
            if found.returncode == 0 and found.stdout.strip():
                window = found.stdout.splitlines()[0]
                break
            time.sleep(0.1)
        if window is None:
            raise TimeoutError(f"Janela não apareceu: {label}")
        command("wmctrl", "-ir", window, "-t", "0")
        command("wmctrl", "-ir", window, "-e", "0,10,10,900,640")
        command("xdotool", "windowactivate", "--sync", window)
        app = {"process": process, "window": window, "data": directory, "label": label}
        self.wait_text(app, "sair" if error else "nova tarefa")
        return app

    def shot(self, app):
        """Captura a janela virtual identificada pelo PID da fixture."""
        self.shot_number += 1
        path = self.output / f"{app['label']}-{self.shot_number:04}.png"
        command("import", "-window", app["window"], str(path))
        return path

    def text(self, app):
        """Lê rótulos da janela real por OCR; erro da ferramenta não vira resultado vazio."""
        return normalized(command("tesseract", str(self.shot(app)), "stdout", "--psm", "11"))

    def wait_text(self, app, expected):
        """Aguarda um marcador positivo da interface em vez de assumir que já houve render."""
        deadline = time.monotonic() + 8
        while time.monotonic() < deadline:
            text = self.text(app)
            if normalized(expected) in text:
                return text
            time.sleep(0.1)
        raise AssertionError(f"Marcador ausente: {expected!r}; tela: {text!r}")

    def keys(self, app, *keys):
        """Envia teclas apenas à fixture no Xvfb, preservando foco entre eventos."""
        active = command("xdotool", "getactivewindow").strip()
        if active != app["window"]:
            command("xdotool", "windowactivate", "--sync", app["window"])
        command("xdotool", "key", "--clearmodifiers", "--delay", "180", *keys)
        time.sleep(0.15)

    def type(self, app, text):
        """Digita texto artificial depois de verificar o alvo ativo no display virtual."""
        assert command("xdotool", "getactivewindow").strip() == app["window"]
        command("xdotool", "type", "--clearmodifiers", text)

    def rows(self, app):
        """Observa o SQLite real da fixture sem criar nem modificar o arquivo."""
        path = app["data"] / "tasks.sqlite3"
        with sqlite3.connect(f"file:{path}?mode=ro", uri=True) as connection:
            return connection.execute("SELECT id,title,status FROM tasks ORDER BY id").fetchall()

    def saved(self, app):
        """Exige Salvo como palavra inteira e rejeita o marcador Não salvo."""
        deadline = time.monotonic() + 8
        while time.monotonic() < deadline:
            text = self.text(app)
            if re.search(r"\bsalvo\b", text) and "nao salvo" not in text:
                return
            time.sleep(0.1)
        raise AssertionError(f"Commit não confirmado na UI: {text!r}")

    def list_focused(self, app):
        """Observa o contorno de foco visível, inclusive quando o aviso desloca a lista."""
        image = Image.open(self.shot(app)).convert("RGB")
        return any(image.getpixel((24, y)) == FOCUS_COLOR for y in range(260, 580))

    def focus_list(self, app):
        """Percorre Tab até o foco da lista e falha se o teclado não a alcançar."""
        self.keys(app, "ctrl+n")
        for _ in range(16):
            if self.list_focused(app):
                return
            self.keys(app, "Tab")
        raise AssertionError("Tab não alcançou a lista")

    def dialog(self, app):
        """Observa a superfície do modal sem depender do OCR sobre o backdrop."""
        return Image.open(self.shot(app)).convert("RGB").getpixel((220, 240)) == (34, 39, 47)

    def marker(self, app):
        """Prova que Ctrl+N recupera o teclado ao criar um registro novo com identidade própria."""
        assert not any("7777777" in row[1] for row in self.rows(app))
        self.keys(app, "ctrl+n")
        self.type(app, "7777777")
        self.keys(app, "Return")
        self.saved(app)
        assert any("7777777" in row[1] for row in self.rows(app)), "Ctrl+N não recuperou a entrada"

    def close(self, app):
        """Verifica fechamento pelo atalho da aplicação, sem usar o gerenciador de janelas."""
        self.keys(app, "ctrl+q")
        assert app["process"].wait(timeout=5) == 0

    def run_case(self, name):
        """Executa uma jornada e sempre encerra somente os processos que ela criou."""
        directory = self.output / name
        block = None
        try:
            if name in ("initial_invalid", "invalid_directory_retry", "initial_corrupt", "initial_locked_retry", "initial_second_instance"):
                if name in ("initial_invalid", "invalid_directory_retry"):
                    directory.mkdir()
                    app = self.launch(directory, name, override="relative", error=True)
                    if name == "invalid_directory_retry":
                        text = self.wait_text(app, "corrija o ambiente e reabra")
                        assert "tentar novamente" not in text, text
                    self.keys(app, "ctrl+n")
                    self.close(app)
                elif name == "initial_corrupt":
                    directory.mkdir()
                    (directory / "tasks.sqlite3").write_bytes(b"banco invalido\0\xff")
                    app = self.launch(directory, name, error=True)
                    self.close(app)
                elif name == "initial_locked_retry":
                    self.seed(directory)
                    block = sqlite3.connect(directory / "tasks.sqlite3")
                    block.execute("BEGIN EXCLUSIVE")
                    app = self.launch(directory, name, error=True)
                    self.keys(app, "Tab", "Return")
                    self.wait_text(app, "tentar novamente")
                    self.close(app)
                else:
                    self.seed(directory)
                    first = self.launch(directory, name + "-first")
                    app = self.launch(directory, name + "-second", error=True)
                    assert "tentar novamente" not in self.wait_text(app, "aberto para estes dados")
                    self.close(app)
                    assert first["process"].poll() is None
                return

            records = TASKS
            if name == "hidden_selection":
                records = [("Primeira pendente", "pending"), ("Outra pendente", "pending"), TASKS[1]]
            self.seed(directory, records)
            app = self.launch(directory, name)
            if name == "retry_invalid_close":
                block = sqlite3.connect(directory / "tasks.sqlite3")
                block.execute("BEGIN IMMEDIATE")
                self.type(app, "Antes")
                self.keys(app, "Return")
                self.wait_text(app, "tentar novamente")
                assert len(self.rows(app)) == 2, "A escrita bloqueada alterou o snapshot"
                self.keys(app, "ctrl+a", "BackSpace", "ctrl+q")
                assert self.dialog(app), "O fechamento não pediu decisão sobre a falha real"
                self.keys(app, "Return")
                assert app["process"].wait(timeout=5) == 0
                assert len(self.rows(app)) == 2, "Retry inválido alterou o banco"
                return
            if name == "invalid_title":
                self.keys(app, "Return")
                text = self.wait_text(app, "informe um titulo")
                assert "tentar novamente" not in text and normalized(str(directory)) not in text
                assert len(self.rows(app)) == 2
                self.close(app)
                return
            if name.startswith("retry_"):
                if name == "retry_rename":
                    self.focus_list(app)
                    self.keys(app, "Home", "Return")
                    self.wait_text(app, "editar tarefa")
                    self.keys(app, "ctrl+a")
                block = sqlite3.connect(directory / "tasks.sqlite3")
                block.execute("BEGIN IMMEDIATE")
                self.type(app, "Antes")
                self.keys(app, "Return")
                self.wait_text(app, "tentar novamente")
                self.keys(app, "ctrl+a")
                self.type(app, "Depois")
                block.rollback()
                block.close()
                block = None
                tabs = 6 if name == "retry_rename" else 5
                self.keys(app, *(["Tab"] * tabs), "Return")
                self.saved(app)
                titles = [row[1] for row in self.rows(app)]
                assert "Depois" in titles and "Antes" not in titles, f"Retry não usou o rascunho atual: {titles!r}"
                confirmed = self.rows(app)
                self.close(app)
                reopened = self.launch(directory, name + "-reopened")
                self.wait_text(reopened, "Depois")
                assert self.rows(reopened) == confirmed
                self.close(reopened)
                return
            if name == "hidden_selection":
                self.keys(app, "ctrl+2")
                self.focus_list(app)
                self.keys(app, "Home", "space")
                self.saved(app)
                confirmed = self.rows(app)
                self.keys(app, "Delete")
                assert not self.dialog(app), "Exclusão abriu para seleção oculta"
                self.keys(app, "Return")
                self.wait_text(app, "nova tarefa")
                self.type(app, "8888888")
                self.keys(app, "Return")
                self.marker(app)
                assert self.rows(app)[:len(confirmed)] == confirmed, "Enter alterou uma seleção oculta"
                self.close(app)
                return

            assert name in {
                "focus_list", "focus_delete_button", "focus_checkbox", "focus_repeated_close",
                "focus_delete_last", "focus_reopen",
            }, f"Jornada sem implementação: {name}"
            if name == "focus_repeated_close":
                block = sqlite3.connect(directory / "tasks.sqlite3")
                block.execute("BEGIN IMMEDIATE")
                self.focus_list(app)
                self.keys(app, "Home", "space")
                self.wait_text(app, "tentar novamente")
                self.keys(app, "ctrl+q")
                assert self.dialog(app)
                self.keys(app, "ctrl+q", "Escape")
                assert self.list_focused(app), "Diálogo perdeu o alvo de retorno"
                block.rollback()
                block.close()
                block = None
            else:
                if name == "focus_reopen":
                    self.keys(app, "ctrl+3")
                elif name != "focus_delete_button":
                    self.keys(app, "ctrl+2")
                self.focus_list(app)
                if name in ("focus_delete_button", "focus_delete_last"):
                    if name == "focus_delete_button":
                        self.keys(app, "Tab", "Tab", "Tab", "space")
                    else:
                        self.keys(app, "Home", "Delete")
                    assert self.dialog(app)
                    self.keys(app, "Tab", "space")
                elif name == "focus_checkbox":
                    self.keys(app, "Tab", "space")
                else:
                    self.keys(app, "Home", "space")
                self.saved(app)
                statuses = [(row[0], row[2]) for row in self.rows(app)]
                expected = [(1, "completed"), (2, "completed")]
                if name in ("focus_delete_button", "focus_delete_last"):
                    expected = [(2, "completed")]
                elif name == "focus_reopen":
                    expected = [(1, "pending"), (2, "pending")]
                assert statuses == expected, statuses
            self.marker(app)
            self.close(app)
        finally:
            if block is not None:
                block.rollback()
                block.close()
            for process in self.apps:
                if process.poll() is None:
                    process.terminate()
                    process.wait(timeout=5)
            self.apps.clear()


def main():
    """Roda as jornadas selecionadas; qualquer falha ou infraestrutura ausente resulta em exit 1."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=Path(__file__).resolve().parents[1] / "target/release/rust_gpui")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--case", action="append", choices=CASES)
    arguments = parser.parse_args()
    server_pid = require_virtual_display()
    binary = arguments.binary.resolve()
    if not binary.is_file():
        raise FileNotFoundError("Compile primeiro com cargo build --locked --release")
    output = arguments.output.resolve() if arguments.output else Path(tempfile.mkdtemp(prefix="rust-gpui-headless-"))
    output.mkdir(parents=True, exist_ok=True)
    pinned_binary = output / "app-under-test"
    shutil.copyfile(binary, pinned_binary)
    pinned_binary.chmod(0o700)
    chosen = arguments.case or list(CASES)
    report = {"binary": str(binary), "sha256": hashlib.sha256(pinned_binary.read_bytes()).hexdigest(),
              "display": os.environ["DISPLAY"], "xvfb_pid": server_pid, "cases": []}
    with (output / "openbox.log").open("w") as log:
        manager = subprocess.Popen(["openbox", "--sm-disable"], stdout=log, stderr=subprocess.STDOUT)
    try:
        deadline = time.monotonic() + 5
        while subprocess.run(["wmctrl", "-m"], capture_output=True).returncode:
            if time.monotonic() > deadline:
                raise TimeoutError("Openbox não iniciou no display virtual")
            time.sleep(0.1)
        qa = NativeQa(pinned_binary, output)
        for case in chosen:
            started = time.monotonic()
            try:
                qa.run_case(case)
                row = {"case": case, "status": "pass"}
            except Exception as error:
                row = {"case": case, "status": "fail", "error": str(error)}
            row["seconds"] = round(time.monotonic() - started, 2)
            report["cases"].append(row)
            print(json.dumps(row, ensure_ascii=False), flush=True)
            (output / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2))
        assert len(report["cases"]) == len(chosen) > 0
        passed = sum(row["status"] == "pass" for row in report["cases"])
        print(f"{passed}/{len(chosen)} jornadas aprovadas; evidências: {output}", flush=True)
        return 0 if passed == len(chosen) else 1
    finally:
        manager.terminate()
        manager.wait(timeout=5)


if __name__ == "__main__":
    raise SystemExit(main())
