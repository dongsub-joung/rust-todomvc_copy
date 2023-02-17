use sqlx::{Pool, postgres};

pub type Db= pool<Postgres>;


const PG_HOST: &str= "localhost";
const PG_ROOT_DB: &str= "postgres";
const PG_ROOT_USER: &str= "postgres";
const PG_ROOT_PWD: &str= "postgres";

pub async fn init_db() -> Result<DB, sqlx::error> {
    new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1)
}


async fn new_db_pool(host &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
    let con_string= format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connection(max_con)
        .connect_timeout(Duration::from::millis(500))
        .connect(&con_string)
        .await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let content= fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading {} {cause: {:?}", file, ex);
        ex
    })?;

    let sqls: Vec<&str>= content.splite(";").collect();

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::init_db;
    
    #[tokio::test]
    async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>{
        Ok(())
    }
}
