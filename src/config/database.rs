use sqlx::Mssql;
use sqlx::MssqlPool;
use sqlx::Pool;
use std::env;
use std::fs;

pub async fn establish_connection() -> Pool<Mssql> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    Pool::<Mssql>::connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn run_migrations(pool: &MssqlPool) {
    let migrations_folder = "./migrations";
    let mut connection = pool.acquire().await.expect("Failed to acquire connection");

    let files = fs::read_dir(migrations_folder).expect("Failed to read migration directory");

    for file in files {
        if let Ok(file) = file {
            let filename = file.file_name();
            let filename_str = filename.to_string_lossy();

            if filename_str.ends_with(".sql") {
                let migration_script = fs::read_to_string(file.path())
                    .expect("Failed to read migration script");

                println!("Running migration: {}", filename_str);
                match sqlx::query(&migration_script).execute(&mut connection).await {
                    Ok(_) => println!("Migration {} applied successfully", filename_str),
                    Err(e) => {
                        println!("Error applying migration {}: {}", filename_str, e);
                    }
                }
            }
        }
    }

    println!("All migrations applied");
}