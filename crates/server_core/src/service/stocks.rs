#[derive(Clone)]
pub struct StocksService {
    pub database: sea_orm::DatabaseConnection,
}

impl StocksService {
    pub fn new(database: sea_orm::DatabaseConnection) -> Self {
        Self { database }
    }
}
