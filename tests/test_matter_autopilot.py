"""
Matter-Autopilot offline tests. No network, API credentials or systemd required.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import json
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


if __name__ == "__main__":
    unittest.main()
