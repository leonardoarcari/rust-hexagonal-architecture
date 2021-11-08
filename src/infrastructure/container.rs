use shaku::ModuleBuilder;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    adapter::output::AccountRepository,
    application::{HelloWorldUseCaseImpl, PingPongUseCaseImpl, SendMoneyService},
};

use super::db::{DataSourceImpl, DataSourceImplParameters};

pub type Inject<'r, I> = shaku_rocket::Inject<'r, HexagonalRocketModule, I>;

shaku::module! {
    pub HexagonalRocketModule {
        components = [PingPongUseCaseImpl,
                      HelloWorldUseCaseImpl,
                      DataSourceImpl,
                      SendMoneyService,
                      AccountRepository],

        providers = []
    }
}

pub async fn default_module(db_pool: PgPool) -> ModuleBuilder<HexagonalRocketModule> {
    HexagonalRocketModule::builder()
        .with_component_parameters::<DataSourceImpl>(DataSourceImplParameters { pool: db_pool })
}

pub async fn connect_db(uri: &str) -> sqlx::PgPool {
    // Create a connection pool
    PgPoolOptions::new()
        .max_connections(5)
        .connect(uri)
        .await
        .expect("Unable to connect to PostgreSQL")
}

#[cfg(test)]
pub mod tests {
    use anyhow::Result;
    use chrono::Utc;
    use rocket::tokio;

    use crate::{
        application::port::output::LoadAccountPort,
        domain::account::{tests::default_account, AccountId},
        infrastructure::tests::{self, testing_module},
    };

    use shaku::HasComponent;

    #[derive(Component)]
    #[shaku(interface = LoadAccountPort)]
    pub struct MockLoadAccountPort();

    #[rocket::async_trait]
    impl LoadAccountPort for MockLoadAccountPort {
        async fn load_account(
            &self,
            _account_id: AccountId,
            _baseline_date: chrono::DateTime<Utc>,
        ) -> anyhow::Result<crate::domain::account::Account> {
            Ok(default_account().id(AccountId(43)).build()?)
        }
    }

    #[tokio::test]
    async fn test_mock_load_account_port() -> Result<()> {
        tests::setup();
        let module = testing_module()
            .await
            .with_component_override::<dyn LoadAccountPort>(Box::new(MockLoadAccountPort()))
            .build();

        let port: &dyn LoadAccountPort = module.resolve_ref();
        let account = port.load_account(AccountId(42), Utc::now()).await?;
        assert!(account.id().is_some());
        assert_eq!(*account.id().unwrap(), AccountId(43));
        Ok(())
    }
}
