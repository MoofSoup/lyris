# Lyris
At Ceres, we believe in making the impossible fun. That is why we made Lyris, an ergonomics-first, data driven framework for digital signal processing. Lyris lets you focus on what matters: writing digital signal processors. 

```rust
#[processor]
pub mod filter{

	#[derive(Default)]
	struct FilterState {
		z1: f32,
	}

	fn filter(
		
		audio_in: Input,     			// derefs into Option<&[f32]>, 
		cutoff: Input,       			// derefs into Option<&[f32]>
		audio_out: Output,   			// deref muts into &mut [f32]
		state: State<FilterState>,		// derefs into &mut FilterState
		events: Events,         		// Option<&[Event]>, events are copyable
		
	) { 
		for (i, &sample) in audio_in.unwrap_or(&[]).enumerate() {
			audio_out[i] = state.z1;
			state.z1 = sample * 0.1 + state.z1 * 0.9;
		}
	}
}
```

```rust
let (mut runtime, router) = Builder::<Event>::new()
	.add(saw::new())
	.add(filter::new())
	.buffer_length(1024)
	.build(); 

router.route( 
	saw::audio_out(), 
	filter::audio_in(), 
); 

router.route( 
	filter::audio_out(), 
	lyris::output() 
);

runtime.process(None, output_buffer);
``` 

## The Future:
The future of Lyris contains:

- The `#[processor]` macro
- The multi-instancing API
- Polyphonic runtimes

A loose date for these features is September of 2025 (as I need a break from this project for a minute XD)

At Ceres, I am working on a feedback based drum synthesizer VST, inspired by SOPHIE. For updates, to contribute, or just to make cool projects and share your work, please join the official [Ceres Discord](https://discord.gg/QgVPEETetC)

Thanks for checking out Lyris!
~ Sylvia Soup <3