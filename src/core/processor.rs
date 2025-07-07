// this file defines types for the processor proc macro
pub use std::any::TypeId;
pub use std::cell::UnsafeCell;
pub use std::marker::PhantomData;
pub use std::any::Any;
pub use super::types::{ContextHandle, Context, BufferIdx};
pub use super::Runtime;
pub use super::router::PortHandle;
use std::ops::{Deref, DerefMut};


pub trait Port{
    fn port_type() -> PortType;
}
pub enum PortType{
    SystemInput,
    SystemOutput,
    Input,
    Output
}

// Processor argument marker types
pub struct Input<'a>(Option<&'a [f32]>);
pub struct Output<'a>(&'a mut [f32]);
pub struct State<'a, T: Default>(&'a mut T);
pub struct Events<'a, E>(&'a mut E);

impl<'a> Deref for Input<'a> {
    type Target = Option<&'a [f32]>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Deref for Output<'a> {
    type Target = [f32];
    
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DerefMut for Output<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0 
    }
}

impl<T: Default> Deref for State<'_, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.0  // Remove the & - self.0 is already &mut T, which coerces to &T
    }
}

impl<T: Default> DerefMut for State<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0  // Remove the &mut - self.0 is already &mut T
    }
}

impl<'a, E> Deref for Events<'a, E> {
    type Target = E;
    
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, E> DerefMut for Events<'a, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

// Marker types for processor fields
impl Port for Input<'_> {
    fn port_type() -> PortType {
        PortType::Input
    }
}
impl Port for Output<'_> {
    fn port_type() -> PortType {
        PortType::Output
    }
}

pub struct SystemInput;
impl Port for SystemInput {
    fn port_type() -> PortType {
        PortType::SystemInput
    }
}

pub struct SystemOutput;
impl Port for SystemOutput {
    fn port_type() -> PortType {
        PortType::SystemOutput
    }
}


pub trait Processor: 'static {
    type Handle: ProcessorHandle;
    fn buffers_count() -> usize;
    fn slot_count() -> usize;
    fn call<E: Clone + Copy>(runtime: &Runtime<E>, handle: ContextHandle);
    fn create_states() -> Vec<Box<UnsafeCell<dyn Any>>>;
    fn get_handle() -> Self::Handle;
}

pub trait ProcessorHandle {}

fn get_input<E: Clone + Copy + 'static>(runtime: &Runtime<E>, buffer_idx: BufferIdx) -> Input {

    let buffer_id = runtime.buffer_ids[buffer_idx.0]
        .expect("Input buffer not routed to physical buffer");
    
    if let Some(buffer_cell) = runtime.buffers.get(&buffer_id) {

        let buffer_ref = unsafe { &*buffer_cell.get() };
        return Input(Some(buffer_ref));

    } else {

        return Input(None);

    };
}

pub fn get_output<E: Clone + Copy + 'static>(runtime: &Runtime<E>, buffer_idx: BufferIdx) -> Output {
    let buffer_id = runtime.buffer_ids[buffer_idx.0]
        .expect("Output buffer not routed to physical buffer");
    
    let buffer_cell = runtime.buffers.get(&buffer_id)
        .expect("Physical buffer not found");
    
    // Safety: We assume the runtime ensures exclusive access during processor execution
    let buffer_ref = unsafe { &mut *buffer_cell.get() };
    
    Output(buffer_ref)
}

pub fn get_state<T: Default + 'static, E: Clone + Copy + 'static>(
    runtime: &Runtime<E>, 
    state_idx: usize
) -> State<T> {
    let state_cell = &runtime.states[state_idx];
    
    // Safety: We assume the runtime ensures exclusive access during processor execution
    let state_any = unsafe { &mut *state_cell.get() };
    
    let state_ref = state_any.downcast_mut::<T>()
        .expect("State type mismatch");
    
    State(state_ref)
}

// Component name wrapper for fluent API
pub struct ProcessorName<P: Processor>{
    pub name: &'static str,
    pub _phantom: PhantomData<P>,
}

pub fn output() -> PortHandle<SystemOutput> {
    PortHandle::new("__system_output__", 0, TypeId::of::<SystemOutput>())
}

pub fn input() -> PortHandle<SystemInput> {
    PortHandle::new("__system_input__", 0, TypeId::of::<SystemInput>())
}


// #[processor] (future macro)
mod test_filter{
    use super::*;

    #[derive(Default)]
    struct FilterState{
        z1: f32,
        z2: f32,
        z3: f32
    }
    fn filter(audio_in: Input, audio_out: Output, state: State<FilterState>){

    }

    /////////////// MACRO GENERATES ///////////////
    
    // unique key struct:
    struct Filter;

    // the macro can 
    struct FilterHandle {
        audio_in: PortHandle<Input<'static>>,
        audio_out: PortHandle<Output<'static>>,
    }

    impl ProcessorHandle for FilterHandle {}

    impl Processor for Filter {

        type Handle = FilterHandle;

        fn buffers_count() -> usize {2}
        fn slot_count() -> usize {1}

        fn call<E: Clone + Copy>(runtime: &Runtime<E>, ctx_handle: ContextHandle){
            let ctx = runtime.get_ctx(ctx_handle);
            let audio_in_idx = ctx.handle.buffer_ids_start + 0;
            let audio_out_idx = ctx.handle.buffer_ids_start + 1;
            let state_idx = ctx.handle.slot_ids_start + 0;

            let audio_in = get_input(runtime, audio_in_idx);
            let audio_out = get_output(runtime, audio_out_idx);
            let state = get_state::<FilterState, E>(runtime, state_idx);

            filter(audio_in, audio_out, state);

        }

        fn create_states() -> Vec<Box<UnsafeCell<dyn Any>>> {
            vec![
                Box::new(UnsafeCell::new(FilterState::default())),
            ]
        }

        fn get_handle() -> FilterHandle {
            FilterHandle {
                audio_in: PortHandle::new("audio_in", 0, TypeId::of::<Input>()),
                audio_out: PortHandle::new("audio_out", 1, TypeId::of::<Output>())
            }
        }
    }

    pub fn new() -> Filter{
        Filter
    }
}
