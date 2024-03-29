use arangors::{Connection, Database as ArangoDatabase, uclient::surf::SurfClient, GenericConnection};

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
