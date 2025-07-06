mod core;
// mod components;

pub use {
    // Core building blocks
    core::Builder,
    core::Runtime,
    core::Router,
    
    // Marker Types
    core::processor::Input,
    core::processor::Output,
    core::processor::State,
    
    // Routing helpers
    core::router::PortHandle,
    core::processor::input,
    core::processor::output, 
    

    core::processor::Processor,
    core::processor::Port,
    core::processor::PortType,
};