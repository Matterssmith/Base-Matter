"""
Materia-AI offline tests. No API key, network, GitHub or production sessions.
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import importlib.util
from pathlib import Path
import tempfile
import unittest
import subprocess

SPEC = importlib.util.spec_from_file_location("matter_ai", Path(__file__).resolve().parents[1] / "tools" / "matter_ai.py")
matter = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(matter)


class MatterAITests(unittest.TestCase):
    def test_all_27_branches(self):
        self.assertEqual(len(matter.WORKSTREAMS), 27)
        self.assertIn("matter-rust", matter.WORKSTREAMS)
        self.assertIn("msel", matter.WORKSTREAMS)
        self.assertIn("social", matter.WORKSTREAMS)
        self.assertIn("bot", matter.WORKSTREAMS)
        self.assertTrue(matter.WORKSTREAMS["bot"][1].startswith("workstreams/27-"))

    def test_file_boundaries_and_secret_paths(self):
        with tempfile.TemporaryDirectory() as td:
            old = matter.ROOT
            try:
                matter.ROOT = Path(td)
                project = matter.ROOT / "matter-rust"
                subprocess.run(["git", "init", "-q", "-b", "workstreams/07-matter-rust", str(project)], check=True)
                (project / "docs").mkdir()
                self.assertEqual(matter.file_path("matter-rust", "docs/file.md"), project / "docs/file.md")
                for candidate in ("../private.txt", "/etc/passwd", ".env", ".git/config",
                                  "logs/session/identity.db", ".ssh/id_rsa", "vault/data/private.json"):
                    with self.subTest(candidate=candidate):
                        with self.assertRaises(ValueError):
                            matter.file_path("matter-rust", candidate)
                with self.assertRaises(ValueError):
                    matter.file_path("other", "README.md")
            finally:
                matter.ROOT = old

    def test_bot_matter_restricted(self):
        result = matter.write_file("bot", "README.md", "hello")
        self.assertEqual(result["error"], "bot_matter_bloqueado")
        result = matter.run_workstream("bot", ["git", "status"])
        self.assertEqual(result["error"], "bot_matter_bloqueado")

    def test_commands_non_shell(self):
        with tempfile.TemporaryDirectory() as td:
            old = matter.ROOT
            try:
                matter.ROOT = Path(td)
                project = matter.ROOT / "matter-rust"
                subprocess.run(["git", "init", "-q", "-b", "workstreams/07-matter-rust", str(project)], check=True)
                for argv in (["sh", "-c", "rm -rf /"], ["bash", "x.sh"],
                             ["git", "reset", "--hard"], ["git", "push"],
                             ["python3", "-c", "print(1)"], ["curl", "https://x.test"]):
                    with self.subTest(argv=argv):
                        self.assertIn("error", matter.run_workstream("matter-rust", argv))
            finally:
                matter.ROOT = old

    def test_redacts_potential_tokens(self):
        self.assertIn("ocultada", matter.redact("OPENAI_API_KEY=sk-sensitive"))
        self.assertEqual(matter.redact("cargo test: OK"), "cargo test: OK")


if __name__ == "__main__":
    unittest.main()
