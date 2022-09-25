use std::env;

pub fn endpoint() -> String {
    return match env::var("END_POINT") {
        Ok(val) => val.parse().expect("Error parsing endpoint"),
        Err(err) => panic!("Error reading endpoint: {}", err),
    };
}

pub fn database_secret() -> String {
    return match env::var("DB_SECRET") {
        Ok(val) => val.parse().expect("Error parsing database secret"),
        Err(err) => panic!("Error reading database secret: {}", err),
    };
}

pub fn database_table_name() -> String {
    return match env::var("DB_TABLE") {
        Ok(val) => val.parse().expect("Error parsing database table name"),
        Err(err) => panic!("Error reading database table name: {}", err),
    };
}

pub fn database_url() -> String {
    return match env::var("DB_URL") {
        Ok(val) => val.parse().expect("Error parsing database URL"),
        Err(err) => panic!("Error reading database URL: {}", err),
    };
}