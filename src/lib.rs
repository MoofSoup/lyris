mod core;

pub use {
    // Core building blocks
    core::Builder,
    core::Runtime,
    core::Router,

    // the whole processor module
    core::processor,

    
    // Routing helpers
    core::router::PortHandle,
    core::processor::input,
    core::processor::output, 
    

    core::processor::Processor,
    core::processor::Port,
    core::processor::PortType,
};