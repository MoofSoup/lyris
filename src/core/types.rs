// types that are referenced throughout core

use std::any::TypeId;
use std::marker::PhantomData;
use super::runtime::Runtime;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PhysicalBuffer(pub(crate) usize);

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct ComponentId(pub(crate) usize);

#[derive(Clone, Copy)]
pub struct ContextHandle{
    pub(crate) component_id: ComponentId,
    pub buffer_ids_start: BufferIdx,
    pub slot_ids_start: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct SystemBuffers{
    pub(crate) input: Option<SystemComponent>,
    pub(crate) output: Option<SystemComponent>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct LogicalBuffer(pub(crate) usize);

// represents the index in the array of physical buffers
#[derive(Clone, Copy)]
pub struct BufferIdx(pub(crate) usize);

impl Add<usize> for BufferIdx {
    type Output = BufferIdx;

    fn add(self, rhs: usize) -> Self::Output {
        BufferIdx(self.0 + rhs)
    }
}

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

// Context provides safe wrapper around unsafe runtime access
pub struct Context<'a, E: Clone + Copy + 'static> {
    pub(crate) runtime: &'a Runtime<E>,
    pub(crate) handle: ContextHandle,
    pub(crate) buffer_size: usize,
}

impl<'a, E: Clone + Copy> Context<'a, E> {
    pub fn buffer_size(&self) -> usize { todo!(); }

    pub fn sample_rate(){ todo!(); }

    pub fn get_events(&self) -> &[E] {
        &self.runtime.current_events
    }

}

// newtype for impl debug/display
pub struct Update<E: Clone + Copy + 'static>(pub(crate) Box<dyn FnOnce(&mut Runtime<E>)>);
unsafe impl<E: Clone + Copy> Send for Update<E> {}