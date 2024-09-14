use std::sync::{Arc, Mutex};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Option<Surreal<surrealdb::engine::remote::ws::Client>>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn initiate_db(&self) -> surrealdb::Result<()> {
        // Connect to the server
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        // Store the connection in the struct
        let mut conn = self.connection.lock().unwrap();
        *conn = Some(db);

        Ok(())
    }

    pub async fn get_connection(&self) -> Option<Surreal<surrealdb::engine::remote::ws::Client>> {
        let conn = self.connection.lock().unwrap();
        conn.clone()
    }
}
