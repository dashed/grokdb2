/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};

/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::DatabaseName;
use chrono::offset::local::Local;

/* local imports */

use context::{self, Context};
use errors::RawAPIError;

/* ////////////////////////////////////////////////////////////////////////// */

#[derive(Debug, Serialize)]
pub struct BackupResponse {
    filename: String,
    dest_path: String
}

#[inline]
fn generate_backup_response() -> BackupResponse {

    let filename = {
        // TODO: fix
        let name = "backup";

        let filename = format!("{name}-{timestamp}.db",
            name = name,
            timestamp = Local::now().format("%a-%b-%e--%H-%M-%S-%Y").to_string());

        filename
    };

    let dest_path = {
        // TODO: fix
        let dest_path = "./backup/";

        let path = format!("{}/{}", dest_path, filename);
        let path = Path::new(&path);


        let normalized: PathBuf = path.iter().collect();
        let normalized = normalized.to_str().unwrap();

        normalized.to_string()
    };

    return BackupResponse {
        filename: filename,
        dest_path: dest_path
    };
}

#[inline]
pub fn backup_database(context: Rc<RefCell<Context>>) -> Result<BackupResponse, RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let backup_ticket = generate_backup_response();

    match db_conn.backup(DatabaseName::Main, &backup_ticket.dest_path, None) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLiteError("backup::backup_database", sqlite_error));
        },
        _ => {
            // backup complete
        }
    };

    return Ok(backup_ticket);
}
