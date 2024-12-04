use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbInfo {
    pub url: Option<String>,
    pub max_size: Option<u32>,
}

impl DbInfo {
    pub fn poll(&self) -> DbPool {
        let manager = ConnectionManager::<PgConnection>::new(
            self.url.clone().unwrap_or_else(|| String::new()),
        );
        r2d2::Pool::builder()
            .max_size(self.max_size.unwrap_or_else(|| 10))
            .build(manager)
            .expect("Failed create pool")
    }
}

impl std::fmt::Display for DbInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DbInfo[url={}, max_size={}]",
            self.url.clone().unwrap_or_else(|| String::from("")),
            self.max_size.unwrap_or_else(|| 10)
        )
    }
}

pub fn load_db() -> DbInfo {
    DbInfo {
        url: std::env::var("DATABASE_URL").ok(),
        max_size: std::env::var("MAX_CONNECTION")
            .ok()
            .map(|s| s.parse().ok().unwrap_or_else(|| 10)),
    }
}
