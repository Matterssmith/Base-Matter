use crate::core::config::CoreConfig;
use crate::core::context::MatterContext;

#[derive(Clone)]
pub struct MatterRuntime {
    pub config: CoreConfig,
    pub context: MatterContext,
}

impl MatterRuntime {
    pub fn new(config: CoreConfig) -> Self {
        Self {
            config,
            context: MatterContext::new(),
        }
    }

    pub fn initialize(&self) {
        tracing::info!(
            application = %self.config.name,
            environment = %self.config.environment,
            "Matter runtime initialized"
        );
    }

    pub fn context(&self) -> &MatterContext {
        &self.context
    }

    pub fn config(&self) -> &CoreConfig {
        &self.config
    }
}