use core::fmt::Debug;
use sqlx::AnyConnection;

use crate::database::{SqlxDatabase, Version};
use ockam_core::{async_trait, Result};

/// Individual rust migration
#[async_trait]
pub trait RustMigration: Debug + Send + Sync {
    /// Name of the migration used to track which one was already applied
    fn name(&self) -> &str;

    /// Version if format "yyyymmddnumber"
    fn version(&self) -> Version;

    /// Execute the migration
    async fn migrate(
        &self,
        legacy_sqlite_database: Option<SqlxDatabase>,
        connection: &mut AnyConnection,
    ) -> Result<()>;
}
