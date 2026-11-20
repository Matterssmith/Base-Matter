#!/usr/bin/env python3
"""Read-only, non-secret inventory of the existing WhatsApp pairing database.

Never prints phone, JID, keys, account rows, session blobs or SQL records.
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import os
import sqlite3
import stat
from pathlib import Path

ROOT = Path.home() / ".local/share/base-matter/whatsapp-test"
FILES = ("session.db", "session.db-wal", "session.db-shm",
         "activation.db", "replay.db", "runtime.env")

def probe_device_identity(connection, tables):
    """Return presence flags only. Never select or return secret values."""
    if "device" not in tables:
        return {"DEVICE_ROW_EXISTS": "unknown_schema"}
    count = connection.execute('SELECT count(*) FROM "device"').fetchone()[0]
    flags = {"DEVICE_ROW_EXISTS": "yes" if count > 0 else "no"}
    if not count:
        return flags
    columns = {
        row[1].lower()
        for row in connection.execute('PRAGMA table_info("device")')
    }
    required = {"pn", "lid", "account", "noise_key", "identity_key", "adv_secret_key"}
    if not required.issubset(columns):
        flags["DEVICE_SCHEMA"] = "unknown_version"
        return flags
    # Boolean aggregation, not sensitive field retrieval. No phone number,
    # JID, key or account blob is ever materialized in Python.
    row = connection.execute(
        """SELECT
           max(CASE WHEN pn <> '' THEN 1 ELSE 0 END),
           max(CASE WHEN lid <> '' THEN 1 ELSE 0 END),
           max(CASE WHEN account IS NOT NULL AND length(account) > 0
                    THEN 1 ELSE 0 END),
           max(CASE WHEN length(noise_key) > 0
                    AND length(identity_key) > 0
                    AND length(adv_secret_key) > 0
                    THEN 1 ELSE 0 END)
           FROM device"""
    ).fetchone()
    for label, present in zip(
        ("DEVICE_HAS_PN", "DEVICE_HAS_LID", "DEVICE_HAS_ACCOUNT",
         "DEVICE_HAS_KEY_MATERIAL"),
        row,
    ):
        flags[label] = "yes" if present else "no"
    return flags


def main():
    print("WhatsApp original state (metadata only; never prints credentials)")
    if ROOT.is_symlink() or not ROOT.is_dir():
        print("STATUS=missing_or_symlink_directory")
        return
    if ROOT.stat().st_mode & 0o077:
        print("STATUS=directory_permissions_too_open")
        return
    for name in FILES:
        path = ROOT / name
        if path.is_symlink():
            print(f"{name}: REJECTED_SYMLINK")
            return
        if not path.exists():
            print(f"{name}: ABSENT")
            continue
        info = path.stat()
        if not stat.S_ISREG(info.st_mode):
            print(f"{name}: REJECTED_NON_REGULAR")
            return
        print(f"{name}: PRESENT size_bytes={info.st_size}")
    path = ROOT / "session.db"
    if not path.is_file():
        print("SESSION=absent")
        return
    if path.stat().st_size < 100:
        print("SESSION=too_small_for_authentication")
        return
    # Read-only SQLite URI; no file copying, no PRAGMA mutations, no row dumps.
    try:
        connection = sqlite3.connect(path.as_uri() + "?mode=ro", uri=True, timeout=3)
        try:
            connection.execute("PRAGMA query_only=ON")
            integrity = connection.execute("PRAGMA quick_check(1)").fetchone()
            print("SQLITE_INTEGRITY=" + ("OK" if integrity == ("ok",) else "NOT_OK"))
            tables = {
                row[0].lower(): row[0]
                for row in connection.execute(
                    "SELECT name FROM sqlite_master WHERE type='table'"
                )
            }
            print("SQLITE_TABLES=" + str(len(tables)))
            # whatsapp-rust 0.7.0 persists the actual device identity in
            # the singular table "device". A plural "devices" table must not
            # be treated as proof that the linked identity is present.
            flags = probe_device_identity(connection, tables)
            for label, value in flags.items():
                print(label + "=" + value)
        finally:
            connection.close()
    except (sqlite3.Error, OSError) as err:
        print("SESSION_DB_READ_ERROR=" + type(err).__name__)
    print("SESSION_AUTHENTICATED=not_determined_offline")

if __name__ == "__main__":
    main()
