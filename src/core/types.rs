// types that are referenced throughout core

use std::any::TypeId;
use std::marker::PhantomData;
use super::runtime::Runtime;
use std::fmt;

pub trait Resource{
    fn resource_type() -> ResourceType;
}
pub enum ResourceType{
    SystemBuffer,
    UseBuffer,
    F32,
}
// Marker types for processor fields
pub struct Input;
impl Resource for Input {
    fn resource_type() -> ResourceType {
        ResourceType::UseBuffer
    }
}
pub struct Output;
impl Resource for Output {
    fn resource_type() -> ResourceType {
        ResourceType::UseBuffer
    }
}

pub struct F32;
impl Resource for F32 {
    fn resource_type() -> ResourceType {
        ResourceType::F32
    }
}

pub trait Processor: 'static {
    fn buffers_count() -> usize;
    fn slot_count() -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PhysicalBuffer(pub(crate) usize);

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct ComponentId(pub(crate) usize);

#[derive(Clone, Copy)]
pub struct ContextHandle{
    pub(crate) component_id: ComponentId,
    pub(crate) buffer_ids_start: BufferIdx,
    pub(crate) slot_ids_start: usize,
}
pub(crate) struct SystemBuffer;
impl Resource for SystemBuffer {
    fn resource_type() -> ResourceType {
        ResourceType::SystemBuffer
    }
}


pub fn output() -> BufferHandle<SystemBuffer> {
    BufferHandle::new("__system_output__", 0, TypeId::of::<SystemBuffer>())
}

pub fn input() -> BufferHandle<SystemBuffer> {
    BufferHandle::new("__system_input__", 0, TypeId::of::<SystemBuffer>())
}

#[derive(Clone, Copy)]
pub(crate) struct SystemBuffers{
    pub(crate) input: Option<SystemComponent>,
    pub(crate) output: Option<SystemComponent>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct LogicalBuffer(pub(crate) usize);

// Context provides safe wrapper around unsafe runtime access
pub struct Context<'a, T: Processor, E: Clone + Copy + 'static> {
    pub(crate) runtime: &'a Runtime<E>,
    pub(crate) handle: ContextHandle,  
    pub(crate) _phantom: PhantomData<T>,
    pub(crate) buffer_size: usize,
}

impl<'a, T: Processor, E: Clone + Copy> Context<'a, T, E> {
    pub(crate) fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn get_events(&self) -> &[E] {
        &self.runtime.current_events
    }
}

// represents the index in the array of physical buffers
#[derive(Clone, Copy)]
pub(crate) struct BufferIdx(pub(crate) usize);

#[derive(Clone, Copy)]
pub struct UserComponent<E: Clone + Copy + 'static> {
    pub(crate) component: fn(&Runtime<E>, ContextHandle),
    pub(crate) context_handle: ContextHandle,
    pub(crate) field_count: usize,
    pub(crate) instance_name: &'static str,
    pub(crate) processor_type: TypeId,
}

#[derive(Clone, Copy)]
pub(crate) struct SystemComponent{
    pub(crate) component_id: ComponentId,
    pub(crate) buffer_idx: BufferIdx,
    pub(crate) instance_name: &'static str,
}

#[derive(Clone, Copy)]
pub enum StoredComponent<E: Clone + Copy + 'static>{
    User(UserComponent<E>),
    System(SystemComponent)
}

// Component name wrapper for fluent API
pub struct ProcessorName<S: Processor>{
    pub name: &'static str,
    pub _phantom: PhantomData<S>,
}

// Buffer handle for type-safe routing
#[derive(Clone)]
pub struct BufferHandle<R: Resource> {
    pub(crate) name: &'static str,
    pub(crate) field_idx: usize,
    pub(crate) processor_type: TypeId,
    _marker: PhantomData<R>,
}

impl<R: Resource> BufferHandle<R> {
    pub fn new(name: &'static str, field_idx: usize, processor_type: TypeId) -> Self {
        Self {
            name,
            field_idx,
            processor_type,
            _marker: PhantomData,
        }
    }
}
#[derive(Debug)]
pub enum RoutingErr {
    CycleDetected,
    ComponentNotFound,
    BufferNotFound,
}

impl fmt::Display for RoutingErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoutingErr::CycleDetected => write!(f, "Cycle detected in routing graph"),
            RoutingErr::ComponentNotFound => write!(f, "Component not found"),
            RoutingErr::BufferNotFound => write!(f, "Buffer not found"),
        }
    }
}

impl std::error::Error for RoutingErr {}

// newtype for impl debug/display
pub struct Update<E: Clone + Copy + 'static>(pub(crate) Box<dyn FnOnce(&mut Runtime<E>)>);
unsafe impl<E: Clone + Copy> Send for Update<E> {}