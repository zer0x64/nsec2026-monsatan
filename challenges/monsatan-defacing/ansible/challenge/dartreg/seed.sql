-- seed.sql — Populate the database with test data.
--
-- This file is self-contained: it creates all required tables before inserting
-- data, mirroring what the server's init_db() does. Safe to run against a
-- brand-new database file (sqlite3 will create it if it doesn't exist).
--
-- Usage:
--   sqlite3 dartreg.db < seed.sql

-- ── Schema (mirrors db::init_db) ───────────────────────────────────────────
CREATE TABLE IF NOT EXISTS packages (
    name TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS versions (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    package_name     TEXT    NOT NULL REFERENCES packages(name),
    version          TEXT    NOT NULL,
    archive_filename TEXT    NOT NULL,
    archive_sha256   TEXT    NOT NULL,
    pubspec_json     TEXT    NOT NULL,
    created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(package_name, version)
);

CREATE TABLE IF NOT EXISTS pending_uploads (
    upload_id        TEXT PRIMARY KEY,
    archive_filename TEXT NOT NULL,
    created_at       DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS advisories (
    id           TEXT PRIMARY KEY,
    package_name TEXT     NOT NULL REFERENCES packages(name),
    osv_json     TEXT     NOT NULL,
    updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ── Seed data ──────────────────────────────────────────────────────────────
--
-- Timestamps are chosen so that the timeline makes narrative sense:
--   2024-01-01  test_package 1.0.0 published          (vulnerable version)
--   2024-01-10  vulnerability discovered, advisory published
--   2024-01-15  test_package 1.0.1 published           (fixes the vulnerability)
--   2024-01-15  advisory updated to mark 1.0.1 as the fix

-- ── Package ────────────────────────────────────────────────────────────────
INSERT OR IGNORE INTO packages (name) VALUES ('test_package');

-- ── Versions ───────────────────────────────────────────────────────────────

-- 1.0.0 — the vulnerable version.
INSERT OR IGNORE INTO versions (
    package_name,
    version,
    archive_filename,
    archive_sha256,
    pubspec_json,
    created_at
) VALUES (
    'test_package',
    '1.0.0',
    '2e3c5613-ea0d-41dd-aaef-defa3db4420c.tar.gz',
    'c4b2b31d54cb402f87a48effd4f0bd581a0fbc9a71b31ea30c60ed6b8d7d7800',
    '{
        "name": "test_package",
        "version": "1.0.0",
        "description": "A placeholder Dart package used for testing the registry.",
        "homepage": "https://example.com/test_package",
        "environment": { "sdk": ">=3.0.0 <4.0.0" }
    }',
    '2024-01-01T00:00:00Z'
);

-- 1.0.1 — the patch release that fixes the vulnerability.
INSERT OR IGNORE INTO versions (
    package_name,
    version,
    archive_filename,
    archive_sha256,
    pubspec_json,
    created_at
) VALUES (
    'test_package',
    '1.0.1',
    '9c9cce66-2734-4254-b5d9-e386977c44e9.tar.gz',
    'c77bd0c3635204aa679d98443d91a7797a2346a87fe4085f6c99e2b8aedad27f',
    '{
        "name": "test_package",
        "version": "1.0.1",
        "description": "A placeholder Dart package used for testing the registry.",
        "homepage": "https://example.com/test_package",
        "environment": { "sdk": ">=3.0.0 <4.0.0" }
    }',
    '2024-01-15T00:00:00Z'
);

-- ── Advisory ───────────────────────────────────────────────────────────────
-- OSV format: https://ossf.github.io/osv-schema/
-- Only 1.0.0 is listed in affected[].versions; 1.0.1 is the fix.
INSERT OR IGNORE INTO advisories (
    id,
    package_name,
    osv_json,
    updated_at
) VALUES (
    'FLAG-{8a8c0784c7f3e9ee548aa4d8bab836e1}',
    'test_package',
    '{
        "schema_version": "1.2.0",
        "id": "FLAG-{8a8c0784c7f3e9ee548aa4d8bab836e1}",
        "summary": "Placeholder vulnerability in test_package allows arbitrary code execution",
        "details": "A placeholder vulnerability exists in test_package 1.0.0 due to improper input validation in the placeholder subsystem. A remote attacker could exploit this to execute arbitrary code. The issue was introduced in 1.0.0 and is fully remediated in 1.0.1.",
        "published": "2024-01-10T00:00:00Z",
        "modified": "2024-01-15T00:00:00Z",
        "aliases": [],
        "affected": [
            {
                "package": {
                    "name": "test_package",
                    "ecosystem": "Pub"
                },
                "versions": ["1.0.0"]
            }
        ],
        "references": [
            {
                "type": "WEB",
                "url": "https://example.com/advisories/GHSA-test-1234-abcd"
            },
            {
                "type": "FIX",
                "url": "https://example.com/test_package/commit/placeholder-fix-commit-hash"
            }
        ],
        "severity": [
            {
                "type": "CVSS_V3",
                "score": "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"
            }
        ]
    }',
    '2024-01-15T00:00:00Z'
);
