-- ============================================================
-- Money Cockpit — Migration n°001 : INIT
-- ============================================================

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

-- ============================================================
-- REFERENCES with default values
-- ============================================================

CREATE TABLE IF NOT EXISTS ref_banks (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS ref_account_types (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS ref_categories (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    subcategory TEXT    NOT NULL,
    UNIQUE (name, subcategory)
);

CREATE TABLE IF NOT EXISTS ref_assignments (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL UNIQUE
);

INSERT OR IGNORE INTO ref_account_types (name) VALUES
    ('Checking account'),
    ('Savings account'),
    ('Savings plan'),
    ('Securities account'),
    ('Stock savings plan'),
    ('Life insurance'),
    ('Mortgage'),
    ('Other');

INSERT OR IGNORE INTO ref_assignments (name) VALUES
    ('Pets'),
    ('Family'),
    ('Child'),
    ('Partner'),
    ('Personal'),
    ('Car'),
    ('Home');

INSERT OR IGNORE INTO ref_categories (name, subcategory) VALUES
    ('Income', 'Salary'),
    ('Income', 'Freelance'),
    ('Income', 'Refunds'),
    ('Income', 'Benefits & allowances'),
    ('Income', 'Other income'),

    ('Food', 'Groceries'),
    ('Food', 'Restaurants'),
    ('Food', 'Cafes & bars'),
    ('Food', 'Food delivery'),

    ('Housing', 'Rent / Mortgage'),
    ('Housing', 'Condo fees'),
    ('Housing', 'Home insurance'),
    ('Housing', 'Electricity / Gas'),
    ('Housing', 'Internet / Landline'),
    ('Housing', 'Renovation'),
    ('Housing', 'Property taxes'),

    ('Transport', 'Fuel'),
    ('Transport', 'Car insurance'),
    ('Transport', 'Car maintenance'),
    ('Transport', 'Public transport'),
    ('Transport', 'Parking / Tolls'),

    ('Health', 'Medication'),
    ('Health', 'Medical consultations'),
    ('Health', 'Health insurance'),
    ('Health', 'Social security'),

    ('Subscriptions', 'Video streaming'),
    ('Subscriptions', 'Music streaming'),
    ('Subscriptions', 'Software / SaaS'),
    ('Subscriptions', 'Mobile plan'),
    ('Subscriptions', 'AI'),

    ('Leisure', 'Cinema / Shows'),
    ('Leisure', 'Sport'),
    ('Leisure', 'Travel'),
    ('Leisure', 'Games'),

    ('Family & Kids', 'Education'),
    ('Family & Kids', 'Childcare'),
    ('Family & Kids', 'Toys / Kids clothing'),

    ('Savings & Investment', 'Savings transfer'),
    ('Savings & Investment', 'Investment'),

    ('Gifts & Donations', 'Donations'),
    ('Gifts & Donations', 'Gifts'),

    ('Other', 'Uncategorized');

-- ============================================================
-- Bank account
-- ============================================================

CREATE TABLE IF NOT EXISTS accounts (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    title               TEXT    NOT NULL,
    bank_id             INTEGER REFERENCES ref_banks(id)         ON DELETE SET NULL,
    account_type_id     INTEGER REFERENCES ref_account_types(id) ON DELETE SET NULL,
    bic                 TEXT,
    iban                TEXT,
    initial_balance     REAL    NOT NULL DEFAULT 0,
    created_at          TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS operations (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    label           TEXT    NOT NULL,
    date            TEXT    NOT NULL,
    type            TEXT    NOT NULL CHECK (type IN ('debit', 'credit')),
    amount          REAL    NOT NULL CHECK (amount > 0),
    status          TEXT    NOT NULL DEFAULT 'cleared'
                            CHECK (status IN ('planned', 'pending', 'cleared')),
    date_cleared    TEXT,
    category_id     INTEGER REFERENCES ref_categories(id)   ON DELETE SET NULL,
    assignment_id   INTEGER REFERENCES ref_assignments(id)  ON DELETE SET NULL,
    payee           TEXT,
    project_id      INTEGER,
    attachment      TEXT,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ============================================================
-- Manage SQL migrations
-- ============================================================

CREATE TABLE IF NOT EXISTS schema_migrations (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
