"""Regression tests for offline two-store comparison.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import importlib.util
import io
from contextlib import redirect_stdout
from pathlib import Path
import sqlite3
import tempfile
import unittest

PATH = Path(__file__).resolve().parents[1] / "scripts/whatsapp-stores-compare.py"
SPEC = importlib.util.spec_from_file_location("wa_store_compare", PATH)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

CREATE = """CREATE TABLE device (
    id INTEGER PRIMARY KEY, pn TEXT, lid TEXT, account BLOB,
    noise_key BLOB, identity_key BLOB, adv_secret_key BLOB, signed_pre_key BLOB
)"""


def make_db(path, pn="private-account", lid="private-lid", key=b"secret-key"):
    with sqlite3.connect(path) as db:
        db.execute(CREATE)
        db.execute(
            "INSERT INTO device VALUES (1, ?, ?, ?, ?, ?, ?, ?)",
            (pn, lid, b"secret-account", key, key, key, key),
        )


class CompareStoresTest(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        root = Path(self.dir.name)
        self.original = root / "original.db"
        self.alpha = root / "alpha.db"
        self.addCleanup(self.dir.cleanup)

    def test_identical_keys_and_identity_without_printing_values(self):
        make_db(self.original)
        make_db(self.alpha)
        result = MODULE.compare_stores(self.original, self.alpha)
        for name in ("SAME_PN", "SAME_LID", "SAME_DEVICE_KEYS",
                     "SAME_ACCOUNT_RECORD"):
            self.assertIs(result[name], True)
        self.assertNotIn("private", str(result))
        self.assertNotIn("secret", str(result))

    def test_same_phone_but_different_companion_keys(self):
        make_db(self.original)
        make_db(self.alpha, key=b"another-secret")
        result = MODULE.compare_stores(self.original, self.alpha)
        self.assertTrue(result["SAME_PN"])
        self.assertTrue(result["SAME_LID"])
        self.assertFalse(result["SAME_DEVICE_KEYS"])

    def test_different_devices_can_belong_to_same_account(self):
        make_db(
            self.original,
            pn="5215551234567:1@s.whatsapp.net",
            lid="private.user:1@lid",
        )
        make_db(
            self.alpha,
            pn="5215551234567:2@s.whatsapp.net",
            lid="private.user:2@lid",
            key=b"other-secret",
        )
        result = MODULE.compare_stores(self.original, self.alpha)
        self.assertFalse(result["SAME_PN"])
        self.assertFalse(result["SAME_LID"])
        self.assertFalse(result["SAME_DEVICE_KEYS"])
        self.assertEqual(result["SAME_PN_ACCOUNT"], "yes")
        self.assertEqual(result["SAME_LID_ACCOUNT"], "yes")

    def test_different_phone_accounts(self):
        make_db(self.original, pn="5215551234567:1@s.whatsapp.net")
        make_db(self.alpha, pn="5215559876543:2@s.whatsapp.net")
        result = MODULE.compare_stores(self.original, self.alpha)
        self.assertEqual(result["SAME_PN_ACCOUNT"], "no")

    def test_unrecognized_jid_format_is_unknown(self):
        make_db(self.original, pn="unexpected-jid-format")
        make_db(self.alpha, pn="unexpected-jid-format")
        result = MODULE.compare_stores(self.original, self.alpha)
        self.assertEqual(result["SAME_PN_ACCOUNT"], "unknown")

    def test_distinct_phone_is_not_interchangeable(self):
        make_db(self.original)
        make_db(self.alpha, pn="different-phone", lid="different-lid")
        result = MODULE.compare_stores(self.original, self.alpha)
        self.assertFalse(result["SAME_PN"])
        self.assertFalse(result["SAME_LID"])

    def test_comparison_never_modifies_either_file(self):
        make_db(self.original)
        make_db(self.alpha)
        before = (self.original.read_bytes(), self.alpha.read_bytes())
        MODULE.compare_stores(self.original, self.alpha)
        after = (self.original.read_bytes(), self.alpha.read_bytes())
        self.assertEqual(before, after)

    def test_missing_db_reports_without_secrets(self):
        make_db(self.original)
        capture = io.StringIO()
        with redirect_stdout(capture):
            MODULE.main()
        self.assertNotIn("secret", capture.getvalue())


if __name__ == "__main__":
    unittest.main()
