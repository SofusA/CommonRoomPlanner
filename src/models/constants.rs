use std::env;

pub fn database_secret() -> Result<String, String> {
    match env::var("DATABASE_SECRET") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing database secret: {}", err)),
    };
}

pub fn database_url() -> Result<String, String> {
    match env::var("DATABASE_URL") {
        Ok(res) => return Ok(res),
        Err(err) => return Err(format!("Error parsing database url: {}", err)),
    };
}
