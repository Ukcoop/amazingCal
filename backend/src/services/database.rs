use std::io;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    sqlite::{SqlitePoolOptions, SqliteRow},
    {Error, FromRow, Pool, Postgres, Sqlite},
};

enum DbType {
    Postgress,
    Sqlite,
}

pub struct Database {
    db_type: DbType,
    postgres_pool: Option<Pool<Postgres>>,
    sqlite_pool: Option<Pool<Sqlite>>,
}

pub fn convert_sqlx_error<T>(result: Result<T, Error>) -> Result<T, io::Error> {
    result.map_err(|e| match e {
        Error::Io(io_error) => io_error,
        _ => io::Error::new(io::ErrorKind::Other, format!("error: {}", e)),
    })
}

impl Database {
    pub async fn new_db(in_memory: bool, database_url: String) -> Result<Database, sqlx::Error> {
        return match in_memory {
            true => Ok(Database {
                db_type: DbType::Sqlite,
                postgres_pool: None,
                sqlite_pool: Some(
                    SqlitePoolOptions::new()
                        .max_connections(5)
                        .connect(":memory:")
                        .await?,
                ),
            }),
            false => Ok(Database {
                db_type: DbType::Postgress,
                postgres_pool: Some(
                    PgPoolOptions::new()
                        .max_connections(5)
                        .connect(&database_url)
                        .await?,
                ),
                sqlite_pool: None,
            }),
        };
    }

    async fn read_db_sqlite<T>(&self, query: &str) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        if let Some(pool) = &self.sqlite_pool {
            let rows = sqlx::query_as::<_, T>(query).fetch_all(pool).await?;
            return Ok(rows);
        }
        Err(Error::RowNotFound)
    }

    async fn read_db_postgres<T>(&self, query: &str) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        if let Some(pool) = &self.postgres_pool {
            let rows = sqlx::query_as::<_, T>(query).fetch_all(pool).await?;
            return Ok(rows);
        }
        Err(Error::RowNotFound)
    }

    async fn write_db_sqlite(&self, query: &str, data: Vec<String>) -> Result<(), Error> {
        if let Some(pool) = &self.sqlite_pool {
            let mut query_builder = sqlx::query(query);
            for item in data.iter() {
                query_builder = query_builder.bind(item);
            }
            query_builder.execute(pool).await?;
            return Ok(());
        }
        Err(Error::RowNotFound)
    }

    async fn write_db_postgres(&self, query: &str, data: Vec<String>) -> Result<(), Error> {
        if let Some(pool) = &self.postgres_pool {
            let mut query_builder = sqlx::query(query);
            for item in data.iter() {
                query_builder = query_builder.bind(item);
            }
            query_builder.execute(pool).await?;
            return Ok(());
        }
        Err(Error::RowNotFound)
    }

    pub async fn read_db<T>(&self, query: &str) -> Result<Vec<T>, Error>
    where
        T: Send + Unpin,
        for<'r> T: FromRow<'r, SqliteRow>,
        for<'r> T: FromRow<'r, PgRow>,
    {
        return match self.db_type {
            DbType::Sqlite => self.read_db_sqlite::<T>(query).await,
            DbType::Postgress => self.read_db_postgres::<T>(query).await,
        };
    }

    pub async fn write_db(&self, query: &str, data: Vec<String>) -> Result<(), Error> {
        return match self.db_type {
            DbType::Sqlite => self.write_db_sqlite(query, data).await,
            DbType::Postgress => self.write_db_postgres(query, data).await,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_db() {
        match Database::new_db(true, "".to_string()).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };
    }

    #[tokio::test]
    async fn test_write_db() {
        let database: Database = match Database::new_db(true, "".to_string()).await {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        let create_db_query = "
            CREATE TABLE IF NOT EXISTS messages (
                username TEXT NOT NULL,
                message TEXT NOT NULL
            )";

        match database.write_db(create_db_query, vec![]).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add table. {}", e)
            }
        };

        let data: Vec<String> = vec!["testy".to_string(), "Hello, i am testy!".to_string()];

        match database
            .write_db(
                "INSERT INTO messages (username, message) VALUES ($1, $2)",
                data,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to write to table. {}", e)
            }
        };
    }

    #[tokio::test]
    async fn test_read_db() {
        let database: Database = match Database::new_db(true, "".to_string()).await {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        let create_db_query = "
            CREATE TABLE IF NOT EXISTS messages (
                username TEXT NOT NULL,
                message TEXT NOT NULL
            )";

        match database.write_db(create_db_query, vec![]).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add table. {}", e)
            }
        };

        let data: Vec<String> = vec!["testy".to_string(), "Hello, i am testy!".to_string()];

        match database
            .write_db(
                "INSERT INTO messages (username, message) VALUES ($1, $2)",
                data,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to write to table. {}", e)
            }
        };

        #[derive(FromRow, Clone, Debug, PartialEq)]
        pub struct Message {
            pub username: String,
            pub message: String,
        }

        match database
            .read_db::<Message>("SELECT username, message FROM messages")
            .await
        {
            Ok(result) => {
                assert_eq!(result[0].username, "testy".to_string());
            }
            Err(e) => {
                panic!("Error: failed to read from table. {}", e)
            }
        };
    }
}
