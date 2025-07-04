use std::sync::{Arc, Mutex};
use super::types::*;
use super::clerk::Clerk;

// Router provides cloneable interface for routing
pub struct Router<E: Clone + Copy + 'static> {
    pub(crate) clerk: Arc<Mutex<Clerk<E>>>,
}

impl<E: Clone + Copy + 'static> Clone for Router<E> {
    fn clone(&self) -> Self {
        Self {
            clerk: Arc::clone(&self.clerk),
        }
    }
}

impl<E: Clone + Copy + 'static + std::fmt::Debug> Router<E> {
    pub fn route<R1: Resource + 'static, R2: Resource + 'static>(&self, from: BufferHandle<R1>, to: BufferHandle<R2>) -> Result<(), RoutingErr> {
        let mut clerk = self.clerk.lock().unwrap();
        clerk.add_route(from, to)
    }
    
    pub fn send_event(&self, event: E) {
        self.clerk.lock().unwrap().send_event(event);
    }
}