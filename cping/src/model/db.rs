use sqlx::{Pool, Postgres};

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";


pub async fn init_db() -> Result<Db, sqlx::Error> {


    return new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await;
}

pub type Db= Pool<Postgres>;
async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error>{
    let con_string= format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    
    return PgPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millies(500))
        .connect(&con_string)
        .await;
}

#[cfg(test)]
#[path= "../_test/mode_db.rs"]
mod tests;