"""Test safe, isolated alpha resume launcher with fake binaries only.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from pathlib import Path
import os
import shlex
import sqlite3
import subprocess
import tempfile
import unittest

SOURCE = Path(__file__).resolve().parents[1] / "scripts/whatsapp-alpha-existing.sh"


class AlphaResumeLauncherTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.top = Path(self.tmp.name)
        self.repo = self.top / "repo"
        scripts = self.repo / "scripts"
        scripts.mkdir(parents=True)
        self.script = scripts / SOURCE.name
        self.script.write_bytes(SOURCE.read_bytes())
        subprocess.run(["git", "init", "-q", "-b", "alpha", str(self.repo)], check=True)
        self.home = self.top / "home"
        self.state = self.home / ".local/share/base-matter/branches/alpha/whatsapp-test"
        self.state.mkdir(parents=True, mode=0o700)
        self.state.chmod(0o700)
        for name in ("session.db", "activation.db", "replay.db"):
            with sqlite3.connect(self.state / name) as db:
                db.execute("CREATE TABLE sample (id INTEGER PRIMARY KEY)")
                db.execute("INSERT INTO sample VALUES (1)")
        self.config = self.state / "runtime.env"
        data = {
            "MATTER_WA_PHONE": "5215551234567",
            "MATTER_WA_OWNER_JID": "5215551234567@s.whatsapp.net",
            "MATTER_WA_OWNER_ID": "owner-alpha",
            "MATTER_WA_SOCKET_ID": "wa-alpha",
            "MATTER_WA_BRIDGE_ADDR": "127.0.0.1:39821",
            "MATTER_WA_BRIDGE_KEY_HEX": "12" * 32,
            "MATTER_WA_REPLAY_KEY_HEX": "ab" * 32,
            "MATTER_WA_SESSION_DB": str(self.state / "session.db"),
            "MATTER_WA_ACTIVATION_DB": str(self.state / "activation.db"),
            "MATTER_WA_REPLAY_DB": str(self.state / "replay.db"),
        }
        self.config.write_text("".join(
            f"export {key}={shlex.quote(value)}\n"
            for key, value in data.items()
        ))
        self.config.chmod(0o600)
        self.control = self.top / "control"
        self.provider = self.top / "provider"
        self.control.write_text(
            "#!/bin/bash\n"
            "if [[ \"$1\" == --serve ]]; then sleep 30; fi\n"
            "printf 'CONTROL_INVOKED=%s\\n' \"$1\"\n"
        )
        self.provider.write_text(
            "#!/bin/bash\n"
            "printf 'PROVIDER_INVOKED=%s\\n' \"$1\"\n"
        )
        self.control.chmod(0o700)
        self.provider.chmod(0o700)
        self.env = os.environ.copy()
        self.env["HOME"] = str(self.home)

    def run_launcher(self, mode):
        return subprocess.run(
            ["bash", str(self.script), mode, str(self.control), str(self.provider)],
            env=self.env, capture_output=True, text=True, timeout=20,
        )

    def test_preflight_uses_only_checks_and_does_not_create_backups(self):
        result = self.run_launcher("--check")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("ALPHA_PREFLIGHT=OK", result.stdout)
        self.assertIn("ALPHA_VAULT=ISOLATED_NOT_ENROLLED", result.stdout)
        self.assertIn("CONTROL_INVOKED=--check", result.stdout)
        self.assertIn("PROVIDER_INVOKED=--check", result.stdout)
        self.assertFalse((self.state / "offline-backups").exists())

    def test_resume_snapshots_existing_databases_without_pairing(self):
        before = (self.state / "session.db").read_bytes()
        result = self.run_launcher("--resume-only")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("ALPHA_SNAPSHOT=VERIFIED_PRIVATE", result.stdout)
        self.assertIn("PROVIDER_INVOKED=--resume-only", result.stdout)
        self.assertNotIn("PROVIDER_INVOKED=--init", result.stdout)
        self.assertEqual(before, (self.state / "session.db").read_bytes())
        snapshots = list((self.state / "offline-backups").glob("*/session.db"))
        self.assertEqual(len(snapshots), 1)
        with sqlite3.connect(snapshots[0]) as db:
            self.assertEqual(db.execute("PRAGMA quick_check(1)").fetchone(), ("ok",))
        self.assertEqual(snapshots[0].stat().st_mode & 0o777, 0o600)

    def test_original_socket_is_rejected_before_any_connection(self):
        text = self.config.read_text().replace("wa-alpha", "wa-test-1")
        self.config.write_text(text)
        self.config.chmod(0o600)
        result = self.run_launcher("--resume-only")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("ALPHA_REFUSED=", result.stderr)
        self.assertFalse((self.state / "offline-backups").exists())

    def test_mixed_original_session_path_is_rejected(self):
        text = self.config.read_text().replace(
            str(self.state / "session.db"),
            str(self.home / ".local/share/base-matter/whatsapp-test/session.db"),
        )
        self.config.write_text(text)
        self.config.chmod(0o600)
        result = self.run_launcher("--check")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("mixed databases", result.stderr)

    def test_no_silent_configuration_recreation(self):
        self.config.unlink()
        result = self.run_launcher("--check")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(self.config.exists())


if __name__ == "__main__":
    unittest.main()
