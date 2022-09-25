use std::env;

pub fn endpoint() -> Result<String, String> {
    match env::var("END_POINT") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing endpoint: {}", err)),
    };
}

pub fn database_secret() -> Result<String, String> {
    match env::var("DB_SECRET") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing database secret: {}", err)),
    };
}

pub fn database_table_name() -> Result<String, String> {
    match env::var("DB_TABLE") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing database table name: {}", err)),
    };
}

pub fn database_url() -> Result<String, String> {
    match env::var("DB_URL") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing database url: {}", err)),
    };
}