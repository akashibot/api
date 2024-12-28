//! Base configuration options
//!
//! - `COOLIFY_URL`: Is this instance of gamba:: currently running on Coolify.

use crate::Env;
use crates_io_env_vars::var;

pub struct Base {
    pub env: Env,
}

impl Base {
    pub fn from_environment() -> anyhow::Result<Self> {
        // https://coolify.io/docs/knowledge-base/environment-variables#coolify-url
        let env = match var("COOLIFY_URL")? {
            Some(_) => Env::Production,
            _ => Env::Development,
        };

        Ok(Self { env })
    }
}
