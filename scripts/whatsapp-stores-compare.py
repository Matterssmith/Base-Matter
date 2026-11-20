#!/usr/bin/env python3
"""Compare two independent WhatsApp SQLite stores without exposing credentials.

Read-only. Does not start WhatsApp, request pairing, or change SQLite.
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from pathlib import Path
import sqlite3
import stat

ROOT = Path.home() / ".local/share/base-matter"
ORIGINAL = ROOT / "whatsapp-test/session.db"
ALPHA = ROOT / "branches/alpha/whatsapp-test/session.db"

# Comparison takes place inside SQLite: Python receives only Boolean results.
COMPARE = """
SELECT
    CASE WHEN a.pn = b.pn THEN 1 ELSE 0 END,
    CASE WHEN a.lid = b.lid THEN 1 ELSE 0 END,
    CASE WHEN a.noise_key = b.noise_key
           AND a.identity_key = b.identity_key
           AND a.adv_secret_key = b.adv_secret_key
           AND a.signed_pre_key = b.signed_pre_key
         THEN 1 ELSE 0 END,
    CASE WHEN a.account = b.account THEN 1 ELSE 0 END
FROM main.device AS a CROSS JOIN candidate.device AS b
WHERE a.id = 1 AND b.id = 1
"""

# Compare the account portion of PN / LID without reading the raw JIDs into
# Python. WhatsApp JIDs use user[:device]@s.whatsapp.net and
# user[:device]@lid; legacy PN may use user.device@s.whatsapp.net.
# The LID username itself may contain dots, so only strip ':device' for LID.
# Any unexpected domain or malformed user means unknown, not a mismatch.
ACCOUNT_SCOPE_COMPARE = """
WITH full_jids AS (
    SELECT a.pn AS a_pn, b.pn AS b_pn, a.lid AS a_lid, b.lid AS b_lid
    FROM main.device AS a CROSS JOIN candidate.device AS b
    WHERE a.id=1 AND b.id=1
),
parts AS (
    SELECT
        CASE WHEN a_pn GLOB '*@s.whatsapp.net'
             THEN substr(a_pn, 1, instr(a_pn,'@') - 1) END AS a_pn_part,
        CASE WHEN b_pn GLOB '*@s.whatsapp.net'
             THEN substr(b_pn, 1, instr(b_pn,'@') - 1) END AS b_pn_part,
        CASE WHEN a_lid GLOB '*@lid'
             THEN substr(a_lid, 1, instr(a_lid,'@') - 1) END AS a_lid_part,
        CASE WHEN b_lid GLOB '*@lid'
             THEN substr(b_lid, 1, instr(b_lid,'@') - 1) END AS b_lid_part
    FROM full_jids
),
users AS (
    SELECT
      CASE WHEN instr(a_pn_part,':')>0
           THEN substr(a_pn_part,1,instr(a_pn_part,':')-1)
           WHEN instr(a_pn_part,'.')>0
           THEN substr(a_pn_part,1,instr(a_pn_part,'.')-1)
           ELSE a_pn_part END AS a_pn_user,
      CASE WHEN instr(b_pn_part,':')>0
           THEN substr(b_pn_part,1,instr(b_pn_part,':')-1)
           WHEN instr(b_pn_part,'.')>0
           THEN substr(b_pn_part,1,instr(b_pn_part,'.')-1)
           ELSE b_pn_part END AS b_pn_user,
      CASE WHEN instr(a_lid_part,':')>0
           THEN substr(a_lid_part,1,instr(a_lid_part,':')-1)
           ELSE a_lid_part END AS a_lid_user,
      CASE WHEN instr(b_lid_part,':')>0
           THEN substr(b_lid_part,1,instr(b_lid_part,':')-1)
           ELSE b_lid_part END AS b_lid_user
    FROM parts
)
SELECT
  CASE WHEN length(a_pn_user) BETWEEN 8 AND 20
             AND length(b_pn_user) BETWEEN 8 AND 20
             AND a_pn_user NOT GLOB '*[^0-9]*'
             AND b_pn_user NOT GLOB '*[^0-9]*'
       THEN a_pn_user=b_pn_user END,
  CASE WHEN a_lid_user IS NOT NULL AND a_lid_user <> ''
             AND b_lid_user IS NOT NULL AND b_lid_user <> ''
       THEN a_lid_user=b_lid_user END
FROM users
"""


def ensure_regular(path):
    # Never follow a symlink to an unrelated credential store.
    current = path
    while current != current.parent:
        if current.is_symlink():
            raise ValueError("symlink_path_rejected")
        current = current.parent
    if not path.is_file() or not stat.S_ISREG(path.stat().st_mode):
        raise ValueError("not_regular_database")


def connection_ro(path):
    return sqlite3.connect(path.as_uri() + "?mode=ro", uri=True, timeout=3)


def inspect_store(path):
    """Return public metadata only, never SQLite row data."""
    ensure_regular(path)
    with connection_ro(path) as db:
        db.execute("PRAGMA query_only=ON")
        if db.execute("PRAGMA quick_check(1)").fetchone() != ("ok",):
            raise ValueError("integrity_check_failed")
        tables = {row[0] for row in db.execute(
            "SELECT name FROM sqlite_master WHERE type='table'"
        )}
        if "device" not in tables:
            raise ValueError("device_table_absent")
        row = db.execute("""
            SELECT COUNT(*),
                   COALESCE(MAX(CASE WHEN id=1 THEN 1 ELSE 0 END), 0)
            FROM device
        """).fetchone()
        return {"device_rows": row[0], "default_device": bool(row[1])}


def compare_stores(original, alpha):
    """Offline checks; DB values are compared inside SQLite, not returned."""
    first = inspect_store(original)
    second = inspect_store(alpha)
    result = {"ORIGINAL_DEVICE_ROWS": first["device_rows"],
              "ORIGINAL_DEFAULT_DEVICE": first["default_device"],
              "ALPHA_DEVICE_ROWS": second["device_rows"],
              "ALPHA_DEFAULT_DEVICE": second["default_device"]}
    if not first["default_device"] or not second["default_device"]:
        result["COMPARISON"] = "default_device_missing"
        return result
    with connection_ro(original) as db:
        db.execute("PRAGMA query_only=ON")
        # URI explicitly enforces read-only mode on the attached database.
        db.execute("ATTACH DATABASE ? AS candidate", (alpha.as_uri() + "?mode=ro",))
        try:
            flags = db.execute(COMPARE).fetchone()
            account_flags = db.execute(ACCOUNT_SCOPE_COMPARE).fetchone()
        finally:
            db.execute("DETACH DATABASE candidate")
    if flags is None:
        result["COMPARISON"] = "device_identity_unavailable"
    else:
        result.update({
            "SAME_PN": bool(flags[0]),
            "SAME_LID": bool(flags[1]),
            "SAME_DEVICE_KEYS": bool(flags[2]),
            "SAME_ACCOUNT_RECORD": bool(flags[3]),
        })
        result["COMPARISON"] = "complete"
        for label, flag in zip(
            ("SAME_PN_ACCOUNT", "SAME_LID_ACCOUNT"), account_flags
        ):
            result[label] = "unknown" if flag is None else (
                "yes" if flag else "no"
            )
    return result


def main():
    print("WhatsApp store comparison: read-only, metadata only")
    try:
        result = compare_stores(ORIGINAL, ALPHA)
    except (ValueError, OSError, sqlite3.Error) as err:
        # No exception body: SQLite may include paths/SQL literals in messages.
        print("COMPARISON_ERROR=" + type(err).__name__)
        return
    for key, value in result.items():
        if isinstance(value, bool):
            value = "yes" if value else "no"
        print(f"{key}={value}")
    print("SESSION_AUTHENTICATED=not_determined_offline")
    print("ACTION=do_not_copy_or_replace_either_database")


if __name__ == "__main__":
    main()
