# Ceres DSP
At Ceres DSP, we believe in making the impossible fun. That is why we made Lyris, an ergonomics-first, data driven framework for digital signal processing. Lyris lets you focus on what matters: writing digital signal processors. 

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

```rust
let (mut runtime, router) = Builder::<Event>::new()
	.add_component::<SawOsc>("saw", saw_component) 
	.add_component::<Filter>("filter 1", filter_component) 
	.buffer_length(1024)
	.build(); 

router.route( 
	SawOsc::named("saw").audio_out(), 
	Filter::named("filter 1").audio_in(), 
); 

router.route( 
	Filter::named("filter 1").audio_out(), 
	output() 
);
```
Currently, we are still fleshing out the `#[processor]` macro. 

## The Future:
We are working on a feedback based drum synthesizer VST, inspired by SOPHIE. For updates, to contribute, or just to make cool projects and share your work, please join the official [Ceres Discord](https://discord.gg/QgVPEETetC)


