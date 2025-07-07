use std::sync::{Arc, Mutex};
use std::any::TypeId;
use super::clerk::Clerk;
use super::processor::Port;
use std::marker::PhantomData;
use std::fmt::{Display, Formatter};

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
    pub fn route<P1: Port + 'static, P2: Port + 'static>(&self, from: PortHandle<P1>, to: PortHandle<P2>) -> Result<(), RoutingErr> {
        let mut clerk = self.clerk.lock().unwrap();
        clerk.add_route(from, to)
    }
    
    pub fn send_event(&self, event: E) {
        self.clerk.lock().unwrap().send_event(event);
    }
}

// Buffer handle for type-safe routing
#[derive(Clone)]
pub struct PortHandle<P: Port> {
    pub(crate) name: &'static str,
    pub(crate) field_idx: usize,
    pub(crate) port_type: TypeId,
    pub(crate) processor_type: TypeId,
    _marker: PhantomData<P>,
}

impl<P: Port> PortHandle<P> {
    pub fn new(name: &'static str, field_idx: usize, port_type: TypeId, processor_type: TypeId) -> Self {
        Self {
            name,
            field_idx,
            port_type,
            processor_type,
            _marker: PhantomData,
        }
    }
}
#[derive(Debug)]
pub enum RoutingErr {
    CycleDetected,
    ProcessorNotFound,
    PortNotFound,
    FromPortIsInput,
    ToPortIsOutput

}

impl Display for RoutingErr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RoutingErr::CycleDetected => write!(f, "Cycle detected in routing graph"),
            RoutingErr::ProcessorNotFound => write!(f, "Component not found"),
            RoutingErr::PortNotFound => write!(f, "Buffer not found"),
            RoutingErr::FromPortIsInput => write!(f, "\"from\" must be an Output port"),
            RoutingErr::ToPortIsOutput => write!(f, "\"to\" must be an Input port"),
        }
    }
}

impl std::error::Error for RoutingErr {}