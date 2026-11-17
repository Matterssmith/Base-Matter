"""Offline regressions for the explicit ecosystem rename plan.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import importlib.util
import pathlib
import unittest

SOURCE = pathlib.Path(__file__).resolve().parents[1] / "scripts" / "rename_ecosystem.py"
SPEC = importlib.util.spec_from_file_location("rename_ecosystem", SOURCE)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class RenamePlanTest(unittest.TestCase):
    def test_all_repositories_end_with_matter(self):
        self.assertEqual(len(MODULE.RENAMES), 22)
        self.assertEqual(len(set(MODULE.RENAMES.values())), 22)
        self.assertTrue(all(name.endswith("-Matter") for name in MODULE.RENAMES.values()))
        self.assertTrue(all(name.startswith("Matter-") for name in MODULE.RENAMES))

    def test_required_platform_and_core_naming(self):
        self.assertEqual(MODULE.RENAMES["Matter-Base"], "Base-Matter")
        self.assertEqual(MODULE.RENAMES["Matter-Vault"], "Vault-Matter")
        self.assertEqual(MODULE.RENAMES["Matter-Baileys"], "WhatsApp-Matter")
        self.assertEqual(MODULE.RENAMES["Matter-Telegraf"], "Telegram-Matter")
        self.assertEqual(MODULE.RENAMES["Matter-Bot"], "Bot-Matter")

    def test_no_unrequested_repo_names(self):
        self.assertNotIn("matterssmith.github.io", MODULE.RENAMES)
        self.assertNotIn("demo-repository", MODULE.RENAMES)

    def test_selective_plan_and_invalid_name(self):
        self.assertEqual(
            MODULE.plan(["Matter-Vault"]),
            [("Matter-Vault", "Vault-Matter")],
        )
        with self.assertRaises(ValueError):
            MODULE.plan(["Not-Installed-Matter"])

    def test_preview_is_safe_by_default(self):
        self.assertEqual(MODULE.main([]), 0)
        with self.assertRaises(SystemExit):
            MODULE.main(["--apply"])


if __name__ == "__main__":
    unittest.main()
