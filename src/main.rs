use dotenv::dotenv;
use google_sheets4::hyper::Client;
use google_sheets4::hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::net::Ipv4Addr;
use std::{env, path::PathBuf};
use csv::{ReaderBuilder, StringRecord, Writer, WriterBuilder};
use warp::Filter;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};
use thiserror::Error;
use serde::{Deserialize, Serialize};

use std::convert::Infallible;

use google_sheets4::{
    api::{ClearValuesRequest, ValueRange},
    Sheets,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ExampleObject {
    name: String,
    number_of_foos: u64,
    number_of_bars: f64,
}

fn generate_sample_objects(n: u64) -> Vec<ExampleObject> {
    (0..n)
        .map(|i| ExampleObject {
            name: format!("Object {}", i),
            number_of_foos: i * 10,
            number_of_bars: i as f64 + 0.5,
        })
        .collect()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api = warp::path!("api" / "booking").and_then(get_response);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3001,
    };

    warp::serve(api).run((Ipv4Addr::LOCALHOST, port)).await
}

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let document_id: String = match env::var("DOCUMENT_ID") {
        Ok(val) => val.parse().expect("Document id not found"),
        Err(_) => return Ok("Error parsing document id".to_string()),
    };

    let tab_id: String = match env::var("TAB_NAME") {
        Ok(val) => val.parse().expect("Tab name not found"),
        Err(_) => return Ok("Error parsing tab name".to_string()),
    };

    let service_account = service_account_from_env().unwrap();

    let auth = ServiceAccountAuthenticator::builder(service_account)
        .build()
        .await
        .unwrap();

    let mut hub = Sheets::new(
        Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    let objects = generate_sample_objects(5);

    for obj in &objects {
        match append_row(&mut hub, document_id.as_str(), tab_id.as_str(), obj)
            .await
        {
            Ok(action) => action,
            Err(err) => return Ok(format!("Error performing action: {}", err)),
        }
    }

    let returned: Vec<ExampleObject> =
        match read_all(&mut hub, document_id.as_str(), tab_id.as_str()).await {
            Ok(action) => action,
            Err(err) => return Ok(format!("Error getting content: {}", err)),
        };

    let json_response = serde_json::to_string(&returned).expect("Failed serialising json");

    return Ok(json_response);
}

async fn append_row(
    sheets: &mut Sheets<HttpsConnector<HttpConnector>>,
    document_id: &str,
    tab_name: &str,
    obj: impl serde::Serialize,
) -> Result<(), SheetsError> {
    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    wtr.serialize(&obj)?;

    let data = String::from_utf8(wtr.into_inner()?)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()?;

    let req = ValueRange {
        major_dimension: None,
        range: Some(tab_name.to_string()),
        values: Some(
            records
                .into_iter()
                .map(|s| s.iter().map(|s| s.to_string()).collect())
                .collect(),
        ),
    };

    sheets
        .spreadsheets()
        .values_append(req, document_id, tab_name)
        .value_input_option("USER_ENTERED")
        .include_values_in_response(false)
        .doit()
        .await?;

    Ok(())
}

async fn read_all<T: DeserializeOwned>(
    sheets: &mut Sheets<HttpsConnector<HttpConnector>>,
    document_id: &str,
    tab_name: &str,
) -> Result<Vec<T>, SheetsError>  {
    let (_body, value_range) = sheets
        .spreadsheets()
        .values_get(document_id, tab_name)
        .doit()
        .await?;

    let rows = value_range.values.unwrap();
    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    for row in rows {
        if let Err(e) = wtr.write_record(&row) {
            println!("error writing row- {:?}", e);
        };
    }

    let data = String::from_utf8(wtr.into_inner()?)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());

    let mut records = vec![];
    for result in rdr.deserialize() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => {
                println!("error deserializing row: {:?}", e);
            }
        }
    }

    Ok(records)
}

fn service_account_from_env() -> Result<ServiceAccountKey, SheetsError> {
    let env = std::env::var("SERVICE_ACCOUNT_JSON")?;
    let key = serde_json::from_str(&env)?;
    Ok(key)
}

#[derive(Error, Debug)]
enum SheetsError {
    #[error("SERVICE_ACCOUNT_JSON not defined")]
    EnvVarNotFound(#[from] std::env::VarError),

    #[error("Invalid service account JSON")]
    InvalidServiceAccountJSON(#[from] serde_json::Error),

    #[error("Error with token cache path")]
    TokenCachePathError(#[from] std::io::Error),

    #[error(transparent)]
    SheetsError(#[from] google_sheets4::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error("Internal error")]
    InternalUTFError(#[from] std::string::FromUtf8Error),

    #[error("Internal error")]
    InternalWriterError(#[from] csv::IntoInnerError<Writer<Vec<u8>>>),
}