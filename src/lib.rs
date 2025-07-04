mod core;

pub use {
    // Core building blocks
    core::Builder,
    core::Runtime,
    core::Router,
    
    // Marker Types
    core::types::Input,
    core::types::Output,
    core::types::F32,
    
    // Routing helpers
    core::types::ProcessorName,
    core::types::BufferHandle,
    core::types::input,
    core::types::output,  // if this is your helper for output routing
    
    // Processor trait if still needed
    core::types::Processor,
    core::types::Resource,
    core::types::ResourceType,
};