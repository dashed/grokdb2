
/* 3rd-party imports */

use rusqlite::{Connection};
use rusqlite::types::ToSql;
use rusqlite::{Error as SqliteError};

/* local imports */

use contexts::{GlobalContext};
use errors::{RawAPIError};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Deserialize)]
pub struct SetConfigRequest {
    setting: String,
    value: String
}

pub struct Config {
    pub setting: String,
    pub value: String
}


// configs api
impl<'a> GlobalContext<'a> {

    // Some(config) => config was found
    // None => no config was found
    pub fn get_config(&self, setting: String) -> Result<Option<Config>, RawAPIError> {

        if setting.trim().len() <= 0 {
            return Err(RawAPIError::BadInput("configs::get_config", "setting is empty string"));
        }

        let query = "
            SELECT
                setting, value
            FROM Configs
            WHERE
                setting = :setting
            LIMIT 1;
        ";

        let params: &[(&str, &ToSql)] = &[
            (":setting", &setting)
        ];

        db_read_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row_named(query, params, |row| -> Config {
            return Config {
                setting: row.get(0),
                value: row.get(1)
            };
        });

        match results {
            Err(sqlite_error) => {

                match sqlite_error {
                    SqliteError::QueryReturnedNoRows => {
                        return Ok(None);
                    },
                    _ => {}
                };

                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            Ok(config) => {
                return Ok(Some(config));
            }
        };
    }

    // on success, return the config set into the db
    pub fn set_config(&self, setting: String, value: String) -> Result<Config, RawAPIError> {

        if setting.trim().len() <= 0 {
            return Err(RawAPIError::BadInput("configs::get_config", "setting is empty string"));
        }

        let query = "
            INSERT OR REPLACE INTO Configs (setting, value)
            VALUES (:setting, :value);
        ";

        let params: &[(&str, &ToSql)] = &[
            (":setting", &setting),
            (":value", &value),
        ];

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            _ => {/* query sucessfully executed */},
        }

        let config = Config {
            setting: setting.clone(),
            value: value.clone()
        };

        return Ok(config);

    }
}
