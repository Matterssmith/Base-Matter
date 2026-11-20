"""Offline regression tests: WhatsApp session presence without leaking identities.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import importlib.util
import sqlite3
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts/whatsapp-session-inspect.py"
SPEC = importlib.util.spec_from_file_location("wa_inspector", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class WhatsAppSessionInspectionTest(unittest.TestCase):
    def setUp(self):
        self.db = sqlite3.connect(":memory:")

    def tearDown(self):
        self.db.close()

    def tables(self):
        return {
            row[0]: row[0]
            for row in self.db.execute(
                "SELECT name FROM sqlite_master WHERE type='table'"
            )
        }

    def test_plural_table_does_not_count_as_authenticated_device(self):
        self.db.execute("CREATE TABLE devices (id INTEGER PRIMARY KEY)")
        self.db.execute("INSERT INTO devices VALUES (1)")
        self.assertEqual(
            MODULE.probe_device_identity(self.db, self.tables()),
            {"DEVICE_ROW_EXISTS": "unknown_schema"},
        )

    def test_unpaired_device_has_no_phone_or_account_signal(self):
        self.db.execute(
            "CREATE TABLE device (pn TEXT, lid TEXT, account BLOB, "
            "noise_key BLOB, identity_key BLOB, adv_secret_key BLOB)"
        )
        self.db.execute(
            "INSERT INTO device VALUES ('', '', NULL, X'01', X'02', X'03')"
        )
        flags = MODULE.probe_device_identity(self.db, self.tables())
        self.assertEqual(flags["DEVICE_ROW_EXISTS"], "yes")
        self.assertEqual(flags["DEVICE_HAS_PN"], "no")
        self.assertEqual(flags["DEVICE_HAS_LID"], "no")
        self.assertEqual(flags["DEVICE_HAS_ACCOUNT"], "no")
        self.assertEqual(flags["DEVICE_HAS_KEY_MATERIAL"], "yes")

    def test_paired_record_presence_is_not_live_authentication(self):
        self.db.execute(
            "CREATE TABLE device (pn TEXT, lid TEXT, account BLOB, "
            "noise_key BLOB, identity_key BLOB, adv_secret_key BLOB)"
        )
        self.db.execute(
            "INSERT INTO device VALUES ('secret-number', 'secret-id', "
            "X'AABB', X'01', X'02', X'03')"
        )
        flags = MODULE.probe_device_identity(self.db, self.tables())
        self.assertEqual(flags["DEVICE_HAS_ACCOUNT"], "yes")
        self.assertEqual(flags["SESSION_AUTHENTICATED"], "not_determined_offline")
        self.assertNotIn("secret", str(flags))


if __name__ == "__main__":
    unittest.main()
