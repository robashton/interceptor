use crate::error::Result;
use crate::{Interceptor, InterceptorBuilder};
use std::sync::Arc;

/// MockBuilder is a mock Builder for testing.
pub struct MockBuilder {
    pub build:
        Box<dyn (Fn(&str) -> Result<Arc<dyn Interceptor + Send + Sync>>) + Send + Sync + 'static>,
}

impl MockBuilder {
    pub fn new<
        F: (Fn(&str) -> Result<Arc<dyn Interceptor + Send + Sync>>) + Send + Sync + 'static,
    >(
        f: F,
    ) -> Self {
        MockBuilder { build: Box::new(f) }
    }
}

impl InterceptorBuilder for MockBuilder {
    fn build(&self, id: &str) -> Result<Arc<dyn Interceptor + Send + Sync>> {
        (self.build)(id)
    }
}
