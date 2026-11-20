"""
Offline tests: Sakura account installer and safe Matter checkpoint/branch flow.
No root, network, OpenAI API or GitHub credentials required.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

TOOL_DIR = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOL_DIR))
import matter_ai
import matter_branches as branch


class BranchRecoveryTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.old_root = matter_ai.ROOT
        self.old_checkpoints = branch.CHECKPOINT_ROOT
        self.addCleanup(setattr, matter_ai, "ROOT", self.old_root)
        self.addCleanup(setattr, branch, "CHECKPOINT_ROOT", self.old_checkpoints)
        matter_ai.ROOT = Path(self.tmp.name) / "workspaces"
        matter_ai.ROOT.mkdir()
        branch.CHECKPOINT_ROOT = Path(self.tmp.name) / "checkpoints"
        self.root = matter_ai.ROOT / "matter-rust"
        subprocess.run(
            ["git", "init", "-q", "-b", "workstreams/07-matter-rust", str(self.root)],
            check=True, stdout=subprocess.DEVNULL
        )
        subprocess.run(
            ["git", "-C", str(self.root), "config", "user.name", "Test Matter"],
            check=True
        )
        subprocess.run(
            ["git", "-C", str(self.root), "config", "user.email", "test@example.invalid"],
            check=True
        )
        subprocess.run(
            ["git", "-C", str(self.root), "remote", "add", "origin",
             "https://github.com/Matterssmith/Base-Matter.git"],
            check=True
        )
        (self.root / "README.md").write_text("Initial project\n", encoding="utf-8")
        subprocess.run(
            ["git", "-C", str(self.root), "add", "README.md"], check=True
        )
        subprocess.run(
            ["git", "-C", str(self.root), "commit", "-q", "-m", "initial"], check=True
        )

    def test_exact_work_branch_and_remote(self):
        self.assertEqual(branch.directory_for("matter-rust"), self.root.resolve())
        subprocess.run(["git", "-C", str(self.root), "remote", "set-url", "origin",
                        "https://example.com/not-matter.git"], check=True)
        with self.assertRaises(ValueError):
            branch.directory_for("matter-rust")

    def test_safe_change_listing_and_secret_guard(self):
        (self.root / "README.md").write_text("Modified source\n", encoding="utf-8")
        (self.root / "source.rs").write_text("// Safe Rust\n", encoding="utf-8")
        self.assertEqual(branch.files_to_save(self.root), ["README.md", "source.rs"])
        (self.root / "source.rs").write_text("sk-" + "X" * 30, encoding="utf-8")
        with self.assertRaises(ValueError):
            branch.files_to_save(self.root)

    def test_sensitive_filenames_blocked(self):
        for filename in (".env", "secret.key", ".ssh/id_ed25519",
                         "vault/roles.json", ".github/workflows/deploy.yml"):
            with self.subTest(filename=filename):
                with self.assertRaises(ValueError):
                    branch.safe_file(self.root, filename)

    def test_local_checkpoint_not_equal_git_backup(self):
        (self.root / "README.md").write_text("Uncommitted state\n", encoding="utf-8")
        file = branch.checkpoint("matter-rust", "working-tree", ["README.md"])
        self.assertTrue(file.exists())
        self.assertEqual(file.parent, branch.CHECKPOINT_ROOT / "matter-rust")
        self.assertIn('"stage": "working-tree"', file.read_text(encoding="utf-8"))
        self.assertNotIn("Uncommitted state", file.read_text(encoding="utf-8"))
        self.assertTrue((self.root / "README.md").read_text(encoding="utf-8") == "Uncommitted state\n")

    def test_local_save_requires_confirmation(self):
        (self.root / "README.md").write_text("Changed\n", encoding="utf-8")
        with patch.object(branch, "confirm", return_value=False):
            result = branch.save("matter-rust", local_only=True)
        self.assertEqual(result, 1)
        head = subprocess.run(["git", "-C", str(self.root), "log", "-1", "--pretty=%s"],
                              check=True, capture_output=True, text=True)
        self.assertEqual(head.stdout.strip(), "initial")
        with patch.object(branch, "confirm", return_value=True):
            self.assertEqual(branch.save("matter-rust", local_only=True), 0)
        head = subprocess.run(["git", "-C", str(self.root), "log", "-1", "--pretty=%s"],
                              check=True, capture_output=True, text=True)
        self.assertIn("checkpoint(matter-rust)", head.stdout)
        self.assertFalse((self.root / ".git" / "index.lock").exists())

    def test_reject_invalid_promotions_without_network(self):
        for target in ("../stable", "alpha..hack", "workstreams/27-bot",
                       "heads/stable", "feature/stable"):
            with self.subTest(target=target), self.assertRaises(ValueError):
                branch.promote("matter-rust", target)

    def test_batch_save_only_detected_workspaces(self):
        with patch.object(branch, "save", return_value=0) as saver:
            self.assertEqual(branch.execute("save", ["all", "--local"]), 0)
        saver.assert_called_once_with("matter-rust", True)

    def test_bot_saving_never_allowed(self):
        with self.assertRaises(ValueError):
            branch.directory_for("bot")
        self.assertEqual(branch.execute("save", ["bot"]), 1)


if __name__ == "__main__":
    unittest.main()
