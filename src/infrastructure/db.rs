use shaku::Interface;
use sqlx::PgPool;
use std::ops::Deref;

pub trait DataSource: Interface + Deref<Target = PgPool> {
    fn get(&self) -> &PgPool;
}

#[derive(Component)]
#[shaku(interface = DataSource)]
pub struct DataSourceImpl {
    pool: PgPool,
}

impl DataSource for DataSourceImpl {
    fn get(&self) -> &PgPool {
        &self.pool
    }
}

impl Deref for DataSourceImpl {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
