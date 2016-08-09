/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;

/* local imports */

use context::Context;
use errors::RawAPIError;

/* ////////////////////////////////////////////////////////////////////////// */

pub const CONFIG_ROOT_DECK_ID_KEY: &'static str = "root_deck_id";

#[derive(PartialEq, Debug)]
pub struct Config {
    pub setting: String,
    pub value: String,
}

pub fn get_config(context: Context, setting_key: String) -> Result<Option<Config>, RawAPIError> {

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

    db_read_lock!(db_conn; context.database);
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
pub fn set_config(context: Context, setting: String, value: String) -> Result<Config, RawAPIError> {

    if setting.trim().len() <= 0 {
        return Err(RawAPIError::BadInput("configs::get_config", "setting is empty string"));
    }

    let query = "
        INSERT OR REPLACE INTO Configs (setting, value)
        VALUES (:setting, :value);
    ";

    let params: &[(&str, &ToSql)] = &[(":setting", &setting), (":value", &value)];

    db_write_lock!(db_conn; context.database);
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

    /* imports */

    use std::fs;
    use database;
    use api::configs;

    /* setup */

    let file_path = "test/assets/configs_test.db".to_string();

    let db_connection = database::get_database(file_path.clone());

    // config doesn't exist

    {
        match configs::get_config(Context::new(db_connection.clone()), "config_key_1".to_string()).unwrap() {
            Some(_) => assert!(false),
            None => assert!(true)
        };
    };

    // set a config

    {
        let actual = configs::set_config(Context::new(db_connection.clone()),
            "config_key_2".to_string(),
            "value_1".to_string()).unwrap();

        let expected = Config {
            setting: "config_key_2".to_string(),
            value: "value_1".to_string()
        };
        assert_eq!(actual, expected);
    };

    // retrieve a config

    {
        match configs::get_config(Context::new(db_connection.clone()), "config_key_2".to_string()).unwrap() {
            Some(actual) => {
                let expected = Config {
                    setting: "config_key_2".to_string(),
                    value: "value_1".to_string()
                };
                assert_eq!(actual, expected);
            },
            None => assert!(false)
        };
    };

    // overwrite a config

    {
        let actual = configs::set_config(Context::new(db_connection.clone()),
            "config_key_2".to_string(),
            "value_2".to_string()).unwrap();

        let expected = Config {
            setting: "config_key_2".to_string(),
            value: "value_2".to_string()
        };
        assert_eq!(actual, expected);
    };

    {
        match configs::get_config(Context::new(db_connection.clone()), "config_key_2".to_string()).unwrap() {
            Some(actual) => {
                let expected = Config {
                    setting: "config_key_2".to_string(),
                    value: "value_2".to_string()
                };
                assert_eq!(actual, expected);
            },
            None => assert!(false)
        };
    };

    /* teardown */

    fs::remove_file(file_path.clone()).unwrap();
}
