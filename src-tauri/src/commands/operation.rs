use crate::models::{NewOperation, Operation};
use crate::AppState;
use rusqlite::params;
use rusqlite::Row;
use tauri::State;

fn row_to_operation(row: &Row) -> rusqlite::Result<Operation> {
    Ok(Operation {
        id: row.get(0)?,
        account_id: row.get(1)?,
        label: row.get(2)?,
        date: row.get(3)?,
        operation_type: row.get(4)?,
        amount: row.get(5)?,
        status: row.get(6)?,
        date_cleared: row.get(7)?,
        category_id: row.get(8)?,
        assignment_id: row.get(9)?,
        payee: row.get(10)?,
        project_id: row.get(11)?,
        attachment: row.get(12)?,
        created_at: row.get(13)?,
    })
}

const SELECT_FIELDS: &str = "SELECT id, account_id, label, date, operation_type, amount, status, date_cleared, category_id, assignment_id, payee, project_id, attachment, created_at
FROM operations";

#[tauri::command]
pub fn create_operation(
    state: State<AppState>,
    payload: NewOperation,
) -> Result<Operation, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute(
        "INSERT INTO operations (account_id, label, date, operation_type, amount, status, date_cleared, category_id, assignment_id, payee)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            payload.account_id,
            payload.label,
            payload.date,
            payload.operation_type,
            payload.amount,
            payload.status,
            payload.date_cleared,
            payload.category_id,
            payload.assignment_id,
            payload.payee
        ])
    .map_err(|e| e.to_string())?;

    let id = db.last_insert_rowid();

    db.query_row(
        &format!("{SELECT_FIELDS} WHERE id = ?1"),
        params![id],
        row_to_operation,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_operations(state: State<AppState>, account_id: i64) -> Result<Vec<Operation>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = db
        .prepare(&format!(
            "{SELECT_FIELDS} WHERE account_id = ?1 ORDER BY date ASC"
        ))
        .map_err(|e| e.to_string())?;

    let operations = stmt
        .query_map(params![account_id], row_to_operation)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(operations)
}

#[tauri::command]
pub fn update_operation(
    state: State<AppState>,
    id: i64,
    payload: NewOperation,
) -> Result<Operation, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute(
        "UPDATE operations SET
        account_id = ?1,
        label = ?2,
        date = ?3,
        operation_type = ?4,
        amount = ?5,
        status = ?6,
        date_cleared = ?7,
        category_id = ?8,
        assignment_id = ?9,
        payee = ?10,
        WHERE id = ?13",
        params![
            payload.account_id,
            payload.label,
            payload.date,
            payload.operation_type,
            payload.amount,
            payload.status,
            payload.date_cleared,
            payload.category_id,
            payload.assignment_id,
            payload.payee,
            id
        ],
    )
    .map_err(|e| e.to_string())?;

    db.query_row(
        &format!("{SELECT_FIELDS} WHERE id = ?1"),
        params![id],
        row_to_operation,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_operation(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.execute("DELETE FROM operations WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
