use std::sync::{Arc, Mutex};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct Database {
    pub connection: Arc<Mutex<Option<Surreal<surrealdb::engine::remote::ws::Client>>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn initiate_db(&self, config: AppConfig) -> surrealdb::Result<()> {
        // Connect to the server
        let db = Surreal::new::<Ws>(config.database_url).await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns(config.namespace).use_db(config.database).await?;

        // Store the connection in the struct
        let mut conn = self.connection.lock().unwrap();
        *conn = Some(db);

        Ok(())
    }

    pub async fn get_connection(&self) -> Option<Surreal<surrealdb::engine::remote::ws::Client>> {
        let conn = self.connection.lock().unwrap();
        conn.clone()
    }

    #[cfg(test)]
    pub async fn delete_all_test_data(&self) -> surrealdb::Result<()> {
        use crate::db::types::DatabaseError;

        let conn = self
            .get_connection()
            .await
            .ok_or(DatabaseError::ConnectionLost)
            .unwrap();

        conn.query("DELETE FROM appointment")
            .await
            .map_err(DatabaseError::from)
            .unwrap();
        conn.query("DELETE FROM patient")
            .await
            .map_err(DatabaseError::from)
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
pub mod database_tests {
    use crate::config::config_tests::get_test_config;

    use super::*;
    use tokio::runtime::Runtime;

    pub async fn mock_db() -> Database {
        let db = Database::new();
        db.initiate_db(get_test_config()).await.unwrap();
        db.delete_all_test_data().await.unwrap();
        db
    }

    #[test]
    fn test_new_database() {
        let db = Database::new();
        let conn = db.connection.lock().unwrap();
        assert!(conn.is_none());
    }

    #[test]
    fn test_initiate_db() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let db = Database::new();
            let result = db.initiate_db(get_test_config()).await;
            assert!(result.is_ok());

            let conn = db.connection.lock().unwrap();
            assert!(conn.is_some());
        });
    }

    #[test]
    fn test_get_connection() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let db = Database::new();
            db.initiate_db(get_test_config()).await.unwrap();

            let conn = db.get_connection().await;
            assert!(conn.is_some());
        });
    }
}
