# Processors

## Processors In Abstract

The processor is designed to be a pure function that transforms audio data with no side effects, only dependencies. As a consequence, we can add, remove, and reroute processors within the audio graph at runtime. Processors also feel very comfortable to write, as an author of a processor gets to think purely in terms of signal transformation. As processors are both pure, and execute in dependency order, we can guarantee safety when accessing shared runtime resources

### `#[processor]` Macro

the `#[processor]`macro enables the processors to be pure functions, implementing the necessary boilerplate to communicate resource requirements to the system

```rust
#[processor]
fn filter_component(
	
	audio_in: Input,     // Option<&[f32]>
	cutoff: Input,       // Option<&[f32]>
	audio_out: Output,   // &mut [f32]
	z1: F32,             // &mut f32
	events: Events,         // Option<&[Event]>
	
) { 
	for (i, &sample) in audio_in.unwrap_or(&[]).enumerate() {
		audio_out[i] = z1;
		*z1 = sample * 0.1 + *z1 * 0.9;
	}
}
```
### Marker Type Contracts

Processors declare their requirements through zero-sized marker types that serve as compile-time contracts:

```rust
pub struct Input; // "I need read access to an audio buffer"

pub struct Output; // "I need write access to an audio buffer"

pub struct F32; // "I need mutable access to a single f32 state"
```

These markers prevent processors from accessing anything beyond their declared interface.

## Processors In Context

Now that we have defined, what a processor is, we can work towards building a more holistic view of how processors fit into the larger system. We will start by looking at the `Processor` trait, and tug on the paths that it reveals, discovering:

- How the runtime passes state and dependencies to processors
- Why the builder needs to configure processor metadata 
- What optional inputs teaches us about the Router and dependency graph
- How the pure functional nature of processors lets the whole system make safety guarantees

### Processor Trait

There are two key questions raised by the processor trait:

- Where does the metadata (slot count, buffers count) get used, and why?

- What is the role of the context handle in passing arguments to a processor?

This simple interface teaches us that the Runtime needs a state persistence system, a scheduling system, and a buffer management system. 

```rust
trait Processor {
	fn buffers_count() -> usize; // buffer management
	fn slot_count() -> usize; // state persistence
	fn call(&Runtime, ContextHandle); // execution management
}
```

```rust

impl Processor for filter {
	
	fn slot_count() -> usize { 3 } // audio_in, cutoff, audio_out
	
	fn slot_count() -> usize { 1 } // z1 state
	
	fn call<E: Clone + Copy>(runtime: &Runtime<E>, handle: ContextHandle) {
		
		let ctx = runtime.get_ctx::<Filter>(handle);
		let audio_in = ctx.audio_in(); // Option<&[f32]>
		let z1 = ctx.z1(); // &mut f32
		let events = ctx.get_events();
		
		// Pure computation
		filter_component(audio_in, cutoff, audio_out, z1, events)
		
	}
	
}
```

### Execution Management
Let's examine specifically how the runtime passes state and dependencies to processors
#### Context Handle
The Context Handle defines a handle on runtime resources, and is used when constructing a context object

```rust
#[derive(Clone, Copy)]
pub(crate) struct ContextHandle {
	
	pub(crate) component_id: ComponentId,
	pub(crate) buffer_ids_start: BufferIdx, // Start of buffer allocation
	pub(crate) slot_ids_start: usize, // Start of state allocation
	
}
```
The context uses these indices to provide safe access to the processor's allocated regions without exposing the broader runtime state.

#### Context Implementation

The processor macro generates Context implementations that provide type-safe access to resources. These implementations reveal how the runtime manages memory and enforces safety:

```rust

impl<'a, E: Clone + Copy> Context<'a, Filter, E> {

// Each field gets a typed accessor that:
// 1. Calculates correct buffer/slot index
// 2. Safely accesses runtime memory
// 3. Returns appropriate type (Option<&[f32]>, &mut f32, etc)

	pub fn audio_in(&self) -> Option<&[f32]> {
	// Buffer index calculation shows the runtime manages a lookup table
	// mapping Physical Buffers to processor's fields
	// and that dependencies are managed at the buffer level
		let buffer_idx = self.handle.buffer_ids_start.0 + 0;
		
		if let Some(buffer_id) = self.runtime.buffer_ids[buffer_idx] {
			
			// As processors are pure functions, data races are impossible
			// the system guarantees correct execution order
			unsafe {
				let buffer_cell = self.runtime.buffers
					.get(&buffer_id)
					// fatal error reveals the system guarantees
					// buffer id is valid
					.expect("Fatal Error: Buffer not found");
				let res = &*buffer_cell.get();
				return Some(res)
			}
		}
		
		// Optional buffer access shows that
		// inputs may or may not receive a signal
		None
}

	pub fn z1(&'a self) -> &'a mut f32 {
		// State access shows how runtime manages persistent data
		let slot_idx = self.handle.slot_ids_start + 0;
		
		unsafe {
			let slot_cell = &self.runtime.slots[slot_idx];
			&mut *slot_cell.get()
		}
	}
}

```

### Buffer Management 
Let's examine why the builder needs to configure processor metadata, and how context handles are built. Revisiting the context handle:
```rust
#[derive(Clone, Copy)]
pub(crate) struct ContextHandle {
	
	pub(crate) component_id: ComponentId,
	pub(crate) buffer_ids_start: BufferIdx, // Start of buffer allocation
	pub(crate) slot_ids_start: usize, // Start of state allocation
	
}
```
This teaches us that the runtime manages a lookup table mapping processor dependencies to physical buffers. To fully understand how this lookup table is built and managed is outside of the scope of this document. It requires understanding the divide between`PhysicalBuffer`s and `LogicalBuffer`s, the dependency graph, and the `Router` API.

### State Management
The `ContextHandle` also teaches us the runtime manages a vec of state slots, which we can unpack. The lifecycle of a state slot is:
- `builder.add_component<my_processor>('name')` is called
- a `StoredComponent<MyEventType>` is created
- `StoredComponent<E>`  stores  a context handle,
- and in it stores the result of `my_processor::slot_count()`

### Safety Guarantees Through Pure Functions

The processor's pure function nature enables several critical safety guarantees:

1. **No Data Races**
	- Processors can only access declared resources
	- Execution order matches dependency order
	- No shared mutable state between processors

2. **Real-time Safety**
	- No locks during execution
	- Predictable memory access patterns
	- Buffer swaps atomic between ticks

3. **Hot-reload Safety**
	- State persists across reloads
	- Buffer connections can change safely
	- No in-flight data in buffers between ticks

These guarantees emerge from the combination of pure function processors and dependency-ordered execution. The system is safe not because we carefully guard unsafe blocks, but because the architecture makes unsafe behavior impossible by design.