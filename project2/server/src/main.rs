use axum::Router;
use std::net::SocketAddr;
use server::db::{Database, DBConnector};
use server::requests::routes::create_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db: Database = get_db().await;

    let app: Router = create_routes(db).await;

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_db() -> Database {
    let connector: DBConnector = DBConnector {
        db_url: "http://localhost:8529".to_string(),
        db_name: "project2".to_string(),
        db_username: "root".to_string(),
        db_password: "root".to_string(),
    };

    let db: Database = Database::new(connector).await.expect("Failed to connect to database");

    db
}
