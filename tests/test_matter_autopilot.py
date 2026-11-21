"""
Matter-Autopilot offline tests. No network, API credentials or systemd required.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

TOOL_DIR = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOL_DIR))
import matter_ai
import matter_branches

if sys.platform.startswith("linux"):
    import matter_autopilot as autopilot
else:
    autopilot = None


@unittest.skipUnless(sys.platform.startswith("linux"), "systemd autopilot requires Ubuntu/Linux")
class AutopilotOfflineTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.folder = Path(self.temp.name)
        patches = [
            patch.object(matter_ai, "ROOT", self.folder / "workspaces"),
            patch.object(matter_branches, "CHECKPOINT_ROOT", self.folder / "checkpoints"),
            patch.object(autopilot, "ROOT", self.folder / "state"),
            patch.object(autopilot, "STATE_FILE", self.folder / "state" / "autopilot-state.json"),
            patch.object(autopilot, "LOCK_FILE", self.folder / "state" / "autopilot.lock"),
            patch.object(autopilot, "CONFIG_FILE", self.folder / "config" / "autopilot.json"),
            patch.object(autopilot, "INSTRUCTIONS_PATH", self.folder / "MATTER_AUTOPILOT_INSTRUCTIONS.md"),
            patch.object(matter_ai, "CONFIG_DIR", self.folder / "config"),
        ]
        for p in patches:
            p.start()
            self.addCleanup(p.stop)
        matter_ai.ROOT.mkdir()
        self.repo = matter_ai.ROOT / "matter-rust"
        subprocess.run(["git", "init", "-q", "-b", "workstreams/07-matter-rust",
                        str(self.repo)], check=True)
        for key, value in [("user.name", "Matter Test"), ("user.email", "test@example.invalid")]:
            subprocess.run(["git", "-C", str(self.repo), "config", key, value], check=True)
        subprocess.run(["git", "-C", str(self.repo), "remote", "add", "origin",
                        "https://github.com/Matterssmith/Base-Matter.git"], check=True)
        (self.repo / "README.md").write_text("Matter test\n", encoding="utf-8")
        subprocess.run(["git", "-C", str(self.repo), "add", "README.md"], check=True)
        subprocess.run(["git", "-C", str(self.repo), "commit", "-qm", "initial"], check=True)
        autopilot.INSTRUCTIONS_PATH.write_text(
            "# Instrucciones Matter\nLeer AGENTS.md y priorizar Rust.\n",
            encoding="utf-8"
        )
        self.settings = {
            **autopilot.DEFAULT,
            "projects": ["matter-rust"],
            "daily_cycles": 2,
            "max_api_calls_per_cycle": 3,
        }
        self.addCleanup(setattr, matter_ai, "AUTOPILOT_MODE", False)
        self.addCleanup(setattr, matter_ai, "AUTOPILOT_PROJECT", None)
        self.addCleanup(setattr, matter_ai, "AUTO_LOCAL_EDITS", False)

    def test_literal_instruction_and_ready_scope(self):
        self.assertEqual(autopilot.INSTRUCTION, "continúa el proyecto matter")
        st = autopilot.state()
        self.assertEqual(autopilot.eligible(self.settings, st), ["matter-rust"])
        self.assertIn("continúa el proyecto matter", autopilot.work_prompt("matter-rust", st))
        self.assertIn("workstreams/07-matter-rust", autopilot.work_prompt("matter-rust", st))

    def test_enabled_by_default_with_no_auto_push_and_md(self):
        self.assertTrue(autopilot.DEFAULT["enabled"])
        self.assertFalse(autopilot.DEFAULT["auto_push"])
        self.assertEqual(autopilot.DEFAULT["projects"], ["matter-rust"])
        doc = autopilot.read_instructions()
        self.assertIn("priorizar Rust", doc)
        prompt = autopilot.work_prompt("matter-rust", autopilot.state())
        self.assertIn("lee las instrucciones que están en", prompt)
        self.assertIn("priorizar Rust", prompt)

    def test_missing_or_secret_md_prevents_model_call(self):
        st = autopilot.state()
        autopilot.INSTRUCTIONS_PATH.unlink()
        with patch.object(matter_ai, "ask", side_effect=lambda prompt, history: (
            autopilot.work_prompt("matter-rust", st)
        )):
            result = autopilot.run_cycle(self.settings, st)
        self.assertEqual(result["status"], "error")
        self.assertIn("failed_RuntimeError", st["last_result"])
        autopilot.INSTRUCTIONS_PATH.write_text("OPENAI_API_KEY=sk-do-not-copy", encoding="utf-8")
        with self.assertRaises(RuntimeError):
            autopilot.read_instructions()

    def test_autonomous_edit_and_local_commit(self):
        st = autopilot.state()
        def fake_ask(_prompt, _history):
            written = matter_ai.write_file(
                "matter-rust", "docs/WORK_CONTINUITY.md",
                "# Continuidad Matter\nPrueba de código local.\n"
            )
            self.assertTrue(written.get("ok"), written)
            return "Archivo actualizado; pendiente de pruebas reales."
        with patch.object(matter_ai, "ask", side_effect=fake_ask):
            result = autopilot.run_cycle(self.settings, st)
        self.assertEqual(result["status"], "committed_local")
        self.assertEqual(st["daily_used"], 1)
        self.assertEqual(st["last_project"], "matter-rust")
        self.assertTrue(autopilot.STATE_FILE.is_file())
        recent = subprocess.run(["git", "-C", str(self.repo), "log", "-1", "--pretty=%s"],
                                check=True, capture_output=True, text=True)
        self.assertIn("matter-autopilot", recent.stdout)
        self.assertTrue((self.repo / "docs" / "WORK_CONTINUITY.md").is_file())
        self.assertTrue((self.folder / "checkpoints" / "matter-rust").is_dir())
        self.assertFalse(matter_ai.AUTOPILOT_MODE)

    def test_refuses_other_projects_and_protected_writes(self):
        st = autopilot.state()
        def fake_ask(_prompt, _history):
            denied = matter_ai.dispatch("write_file", {
                "project": "base", "path": "README.md", "content": "change"
            })
            self.assertEqual(denied["error"], "autopilot_solo_proyecto_asignado")
            for path in ["tools/matter_ai.py", ".github/workflows/ci.yml",
                         "config/roles.json", "AGENTS.md"]:
                denied = matter_ai.write_file("matter-rust", path, "malicious")
                self.assertEqual(denied["error"], "ruta_excluida_del_modo_autonomo")
            return "No se aplicaron cambios."
        with patch.object(matter_ai, "ask", side_effect=fake_ask):
            result = autopilot.run_cycle(self.settings, st)
        self.assertEqual(result["status"], "no_changes")
        self.assertEqual(st["daily_used"], 1)

    def test_no_api_when_budget_is_spent(self):
        st = autopilot.state()
        st["daily_used"] = 2
        with patch.object(matter_ai, "ask", side_effect=AssertionError("called unexpectedly")):
            result = autopilot.run_cycle(self.settings, st)
        self.assertEqual(result["status"], "daily_limit")

    def test_invalid_state_fails_closed(self):
        autopilot.initialize_dirs()
        autopilot.STATE_FILE.write_text("not valid json", encoding="utf-8")
        with self.assertRaises(ValueError):
            autopilot.state()

    def test_activate_noninteractive_is_local_only(self):
        with patch.object(matter_ai, "backend_name", return_value="openai"):
            self.assertEqual(autopilot.activate_local(), 2)
            self.assertFalse(autopilot.CONFIG_FILE.exists())
        class GoodResult:
            returncode = 0
        with patch.object(matter_ai, "backend_name", return_value="local"), \
             patch.object(autopilot, "systemctl", return_value=GoodResult()) as ctl:
            self.assertEqual(autopilot.activate_local(), 0)
            self.assertEqual(ctl.call_count, 2)
        self.assertTrue(autopilot.config()["enabled"])
        self.assertFalse(autopilot.config()["auto_push"])

    def test_unattended_publish_requires_explicit_configuration(self):
        parser_args = type("Settings", (), {})()
        parser_args.interval = 900
        parser_args.daily = 5
        parser_args.calls = 3
        parser_args.projects = "matter-rust"
        parser_args.push = True
        with patch.object(autopilot, "confirm", return_value=False):
            result = autopilot.configure(parser_args)
        self.assertEqual(result, 1)
        self.assertFalse(autopilot.CONFIG_FILE.exists())

    def test_autonomous_calls_must_be_readonly_git(self):
        matter_ai.AUTOPILOT_MODE = True
        matter_ai.AUTOPILOT_PROJECT = "matter-rust"
        result = matter_ai.dispatch("run_command", {
            "project": "matter-rust",
            "argv": ["git", "diff", "--output=/tmp/unsafe"]
        })
        self.assertEqual(result["error"], "autopilot_git_lectura_restringida; pruebas_manuales")
        self.assertEqual(matter_ai.dispatch(
            "publish_workstream", {"project": "matter-rust", "message": "no"}
        )["error"], "accion_reservada_para_sesion_interactiva")


@unittest.skipUnless(sys.platform.startswith("linux"), "Bash/systemd installer test requires Linux")
class InstallerAutostartTests(unittest.TestCase):
    def test_installer_enables_local_by_default_and_respects_stop(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            home = root / "Sakura"
            home.mkdir()
            (home / "Matter/workspaces/matter-rust").mkdir(parents=True)
            bin_folder = root / "bin"
            bin_folder.mkdir()
            fake_systemctl = bin_folder / "systemctl"
            fake_systemctl.write_text(
                '#!/bin/sh\nprintf "%s\n" "$*" >> "$HOME/systemctl-calls"\n',
                encoding="utf-8"
            )
            fake_systemctl.chmod(0o755)
            env = dict(os.environ)
            env["HOME"] = str(home)
            env["PATH"] = str(bin_folder) + os.pathsep + env.get("PATH", "")
            installer = TOOL_DIR / "install-matter-ai.sh"
            first = subprocess.run(
                ["bash", str(installer)], env=env, text=True,
                capture_output=True, timeout=30, check=False
            )
            self.assertEqual(first.returncode, 0, first.stdout + first.stderr)
            cfg = json.loads((home / ".config/matter-ai/config.json").read_text())
            self.assertEqual(cfg["backend"], "local")
            autopilot_cfg = home / ".config/matter-ai/autopilot.json"
            first_settings = json.loads(autopilot_cfg.read_text())
            self.assertTrue(first_settings["enabled"])
            self.assertFalse(first_settings["auto_push"])
            self.assertEqual(first_settings["projects"], ["matter-rust"])
            self.assertIn("enable --now matter-autopilot.service", (home / "systemctl-calls").read_text())
            self.assertTrue((home / ".local/share/matter-ai/MATTER_AUTOPILOT_INSTRUCTIONS.md").is_file())
            first_settings["enabled"] = False
            autopilot_cfg.write_text(json.dumps(first_settings), encoding="utf-8")
            previous_log = (home / "systemctl-calls").read_text()
            second = subprocess.run(
                ["bash", str(installer)], env=env, text=True,
                capture_output=True, timeout=30, check=False
            )
            self.assertEqual(second.returncode, 0, second.stdout + second.stderr)
            self.assertEqual((home / "systemctl-calls").read_text(), previous_log)
            self.assertFalse(json.loads(autopilot_cfg.read_text())["enabled"])


if __name__ == "__main__":
    unittest.main()
