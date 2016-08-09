/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;

/* local imports */

use types::Database;
use errors::RawAPIError;

/* ////////////////////////////////////////////////////////////////////////// */

pub const CONFIG_ROOT_DECK_ID_KEY: &'static str = "root_deck_id";

pub struct Config {
    pub setting: String,
    pub value: String,
}

pub fn get_config(database: Database, setting_key: String) -> Result<Option<Config>, RawAPIError> {

    if setting_key.trim().len() <= 0 {
        return Err(RawAPIError::BadInput("configs::get_config", "setting is empty string"));
    }

    let query = "
        SELECT
            setting,
            value
        FROM Configs
        WHERE
            setting = :setting
        LIMIT 1;
    ";

    let params: &[(&str, &ToSql)] = &[(":setting", &setting_key)];

    db_read_lock!(db_conn; database);
    let db_conn: &Connection = db_conn;

    let results = db_conn.query_row_named(query, params, |row| -> Config {
        return Config {
            setting: row.get(0),
            value: row.get(1),
        };
    });

    match results {
        Err(sqlite_error) => {

            match sqlite_error {
                SqliteError::QueryReturnedNoRows => {
                    return Ok(None);
                }
                _ => {}
            };

            return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
        }
        Ok(config) => {
            return Ok(Some(config));
        }
    };

}

// on success, return the config set into the db
pub fn set_config(database: Database, setting: String, value: String) -> Result<Config, RawAPIError> {

    if setting.trim().len() <= 0 {
        return Err(RawAPIError::BadInput("configs::get_config", "setting is empty string"));
    }

    let query = "
        INSERT OR REPLACE INTO Configs (setting, value)
        VALUES (:setting, :value);
    ";

    let params: &[(&str, &ToSql)] = &[(":setting", &setting), (":value", &value)];

    db_write_lock!(db_conn; database);
    let db_conn: &Connection = db_conn;

    match db_conn.execute_named(query, &params[..]) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
        }
        _ => {
            /* query sucessfully executed */
        }
    }

    let config = Config {
        setting: setting.clone(),
        value: value.clone(),
    };

    return Ok(config);

}

#[test]
fn configs_test() {

    use database;

    database::get_database("test/assets/configs_test.db".to_string());

    // TODO: complete

}
