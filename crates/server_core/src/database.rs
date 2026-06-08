use crate::utils::auth::{generate_random_password, hash_password};
use core_db::entity::{refresh_token, stocks, users};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, Database, DatabaseConnection,
    DbBackend, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Schema, Statement,
};

pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "PRAGMA foreign_keys = ON".to_string(),
    ))
    .await?;
    Ok(db)
}

pub async fn init(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    let schema = Schema::new(DbBackend::Sqlite);
    let backend = db.get_database_backend();

    let tables = [
        schema.create_table_from_entity(users::Entity),
        schema.create_table_from_entity(refresh_token::Entity),
        schema.create_table_from_entity(stocks::Entity),
    ];

    for mut stmt in tables {
        stmt.if_not_exists();
        let sql = backend.build(&stmt);
        db.execute(sql).await?;
    }

    let admin_count: u64 = users::Entity::find()
        .filter(users::Column::Name.eq("Admin"))
        .count(db)
        .await?;

    if admin_count == 0 {
        let password =
            std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| generate_random_password());

        let now = chrono::Utc::now().naive_utc();
        let password_hash = hash_password(&password)?;

        users::ActiveModel {
            name: ActiveValue::Set("admin".to_string()),
            avatar: ActiveValue::Set(String::new()),
            password_hash: ActiveValue::Set(password_hash),
            role: ActiveValue::Set(users::Role::Admin),
            settings: ActiveValue::Set(core_domain::UserSettings::default().value()),
            created_at: ActiveValue::Set(now),
            ..Default::default()
        }
        .insert(db)
        .await?;

        println!("================================");
        println!("  Default admin created");
        println!("  Username: admin");
        println!("  Password: {}", password);
        println!("================================");
    }

    Ok(())
}
