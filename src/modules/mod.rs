use crate::core::context::MatterContext;
use anyhow::Result;

pub trait CoreModule: Send + Sync {
    fn name(&self) -> &str;

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn initialize(&self, context: &MatterContext) -> Result<()>;

    fn start(&self, _context: &MatterContext) -> Result<()> {
        Ok(())
    }

    fn stop(&self, _context: &MatterContext) -> Result<()> {
        Ok(())
    }
}
