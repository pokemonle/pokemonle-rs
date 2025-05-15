use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, MultiConnection, PgConnection, QueryResult, SqliteConnection,
};

#[derive(MultiConnection)]
pub enum DatabaseConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
}

pub type ConnectionPool = Pool<ConnectionManager<DatabaseConnection>>;

pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager<DatabaseConnection>>;
