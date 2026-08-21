use crate::models::{Account, NewAccount};
use crate::AppState;
use rusqlite::params;
use rusqlite::Row;
use tauri::State;

fn row_to_account(row: &Row) -> rusqlite::Result<Account> {
    Ok(Account {
        id: row.get(0)?,
        title: row.get(1)?,
        bank_id: row.get(2)?,
        account_type_id: row.get(3)?,
        bic: row.get(4)?,
        iban: row.get(5)?,
        initial_balance: row.get(6)?,
        created_at: row.get(7)?,
    })
}

const SELECT_FIELDS: &str = "
    SELECT id, title, bank_id, account_type_id, bic, iban, initial_balance, created_at
    FROM accounts
";

#[tauri::command]
pub fn create_account(state: State<AppState>, payload: NewAccount) -> Result<Account, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO accounts (title, bank_id, account_type_id, bic, iban, initial_balance)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            payload.title,
            payload.bank_id,
            payload.account_type_id,
            payload.bic,
            payload.iban,
            payload.initial_balance
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    db.query_row(
        &format!("{SELECT_FIELDS} WHERE id = ?1"),
        params![id],
        row_to_account,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_accounts(state: State<AppState>) -> Result<Vec<Account>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = db
        .prepare(&format!("{SELECT_FIELDS} ORDER BY title ASC"))
        .map_err(|e| e.to_string())?;

    let accounts = stmt
        .query_map([], row_to_account)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(accounts)
}

#[tauri::command]
pub fn update_account(
    state: State<AppState>,
    id: i64,
    payload: NewAccount,
) -> Result<Account, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute(
        "UPDATE accounts SET title = ?1, bank_id = ?2, account_type_id = ?3, bic = ?4, iban = ?5, initial_balance = ?6
        WHERE id = ?7",
        params![
            payload.title,
            payload.bank_id,
            payload.account_type_id,
            payload.bic,
            payload.iban,
            payload.initial_balance,
            id
        ])
    .map_err(|e| e.to_string())?;

    db.query_row(
        &format!("{SELECT_FIELDS} WHERE id = ?1"),
        params![id],
        row_to_account,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_account(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute("DELETE FROM accounts WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
