use arangors::{Connection, Database as ArangoDatabase, uclient::surf::SurfClient, GenericConnection};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone)]
pub struct Database {
    pub arango_db: ArangoDatabase<SurfClient>,
}

pub struct DBConnector {
    pub db_url: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

impl Database {
    pub async fn new(connector: DBConnector) -> Result<Self, arangors::error::ArangoError> {
        let arango_conn: GenericConnection<SurfClient> = Connection::establish_basic_auth(&connector.db_url, &connector.db_username, &connector.db_password).await.unwrap();
        let arango_db: ArangoDatabase<SurfClient> = arango_conn.db(&connector.db_name).await.unwrap();
        Ok(Database { arango_db })
    }
}

pub trait ArangoProvider {
    fn get_db(&self) -> &ArangoDatabase<SurfClient>;
}

impl ArangoProvider for Database {
    fn get_db(&self) -> &ArangoDatabase<SurfClient> {
        &self.arango_db
    }
}

pub fn create_update_query(collection: &str, column_name: &str, property_name: &String, fields: &HashMap<&str, Value>) -> String {

    // UPDATE { _key: "1465" } WITH { name: "Andre", ... } IN Item
    let mut query = format!("UPDATE {{ _{}: {} }} WITH {{ ", column_name, property_name);
    let mut first = true;

    for field in fields.keys() {
        if field.to_string() == "key".to_string() {
            continue;
        }
        if first {
            first = false;
        } else {
            query.push_str(", ");
        }
        query.push_str(&format!("{}: @{}", field, field));
    }

    query.push_str(&format!(" }} IN {} RETURN NEW", collection));

    query
}
