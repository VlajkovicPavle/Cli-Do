use sqlx::{
    Pool, Sqlite, SqlitePool, migrate::MigrateDatabase, prelude::FromRow, query,
    sqlite::SqliteQueryResult,
};

// TODO
// DB STRING
#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow)]
pub struct Todo {
    pub title: String,
    pub body: String,
    pub id: i64,
}

#[derive(Default)]
pub struct CreateTodo {
    pub title: String,
    pub body: String,
}

pub struct Database {
    connection: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_url: &str) -> Database {
        Database {
            connection: {
                match initialize_db(db_url).await {
                    Ok(connection) => connection,
                    Err(error) => panic!("Failed to aquire connection {:?}", error),
                }
            },
        }
    }

    pub async fn insert(&self, todo: CreateTodo) -> Result<SqliteQueryResult, sqlx::Error> {
        let query_result = query("INSERT INTO todo (title,body) VALUES( ?, ?)")
            .bind(todo.title)
            .bind(todo.body)
            .execute(&self.connection)
            .await?;
        Ok(query_result)
    }

    pub async fn fetch_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let results = sqlx::query_as::<_, Todo>("SELECT title,body,id FROM todo")
            .fetch_all(&self.connection)
            .await?;
        Ok(results)
    }

    pub async fn delete(&self, todo: Todo) -> Result<SqliteQueryResult, sqlx::Error> {
        let result = sqlx::query("DELETE FROM todo WHERE id=$1")
            .bind(todo.id)
            .execute(&self.connection)
            .await?;
        Ok(result)
    }
}

async fn initialize_db(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Creation of DB successful"),
            Err(error) => panic!("Failed to create DB: {}", error),
        }
    } else {
        println!("Database exists!");
    }
    let db_connection = SqlitePool::connect(db_url).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todo(
              id INTEGER PRIMARY KEY NOT NULL, 
              title TEXT NOT NULL,
              body TEXT NOT NULL)",
    )
    .execute(&db_connection)
    .await?;
    Ok(db_connection)
}
