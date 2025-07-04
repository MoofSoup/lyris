use super::types::*;
use crate::{Clerk, Runtime, Router};
use std::collections::HashMap;
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;

pub(crate) struct Builder<E: Clone + Copy + Debug + 'static>{
    components: Vec<(TypeId, &'static str, StoredComponent<E>)>,
    next_component_id: usize,
    buffer_size: usize,
    slots: Vec<UnsafeCell<f32>>,
}

impl<E: Clone + Copy + Debug> Builder<E> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            next_component_id: 0,
            buffer_size: 512,
            slots: Vec::new(),
        }
    }
    
    pub fn add_component<S: Schema>(
        mut self,
        instance_name: &'static str,
        component_fn: fn(&Runtime<E>, ContextHandle)
    ) -> Self {
        let component_id = ComponentId(self.next_component_id);
        self.next_component_id += 1;
        
        let handle = ContextHandle {
            component_id,
            buffer_ids_start: BufferIdx(0), // Will be set during build
            slot_ids_start: self.slots.len()
        };

        for _slot in 0.. S::slot_count() {
            self.slots.push(UnsafeCell::new(0.0));
        };

        let stored = UserComponent {
            component: component_fn,
            context_handle: handle,
            field_count: S::BUFFERS_COUNT,
            instance_name,
            schema_type: TypeId::of::<S>(),
        };

        self.components.push((TypeId::of::<S>(), instance_name, StoredComponent::User(stored)));
        self
    }

    pub fn buffer_length(mut self, length: usize) -> Self {
        self.buffer_size = length;
        self
    }
    
    pub fn build(self) -> (Runtime<E>, Router<E>) {
        let (update_tx, update_rx) = lockfree::channel::spsc::create();
        let (event_tx, event_rx) = lockfree::channel::spsc::create();
        
        let mut components = HashMap::new();

        let input_component = StoredComponent::System(SystemComponent {
            component_id: ComponentId(0), // system components have hard coded input
            instance_name: "__system_input__",
            buffer_idx: BufferIdx(0), // will be configured during routing!
        });
        components.insert((TypeId::of::<SystemBuffer>(), "__system_input__"), input_component);

        let output_component = StoredComponent::System(SystemComponent {
            component_id: ComponentId(1), 
            instance_name: "__system_output__",
            buffer_idx: BufferIdx(0), // will be configured during routing!
        });
        components.insert((TypeId::of::<SystemBuffer>(), "__system_output__"), output_component);
        
        for (type_id, name, stored) in self.components {
            components.insert((type_id, name), stored);
        }
        
        let clerk = Arc::new(Mutex::new(Clerk::new(components, self.buffer_size, update_tx, event_tx)));
        
        let router = Router {
            clerk: Arc::clone(&clerk),
        };
        
        let runtime = Runtime::new(update_rx, event_rx, self.slots, self.buffer_size);
        
        (runtime, router)
    }

}