pub mod commit;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn execute(&self, arg: Option<String>) -> Result<()>;
}
