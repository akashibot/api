use crates_io_env_vars::{required_var_parsed, var_parsed};

pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

impl DatabaseConfig {
    const DEFAULT_POOL_SIZE: u32 = 3;

    pub fn from_environment() -> anyhow::Result<Self> {
        let pool_size = var_parsed("DB_ASYNC_POOL_SIZE")?.unwrap_or(Self::DEFAULT_POOL_SIZE);
        let url = required_var_parsed("DATABASE_URL")?;

        Ok(Self { url, pool_size })
    }
}
