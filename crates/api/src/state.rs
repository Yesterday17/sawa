use std::sync::Arc;

pub struct AppState<S> {
    pub service: Arc<S>,
}

impl<S> Clone for AppState<S> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}

impl<S> AppState<S> {
    pub fn new(service: S) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}
