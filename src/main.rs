use memory::MemoryMap;
use processor::Processor;

mod memory;
mod processor;
mod utils;

fn main() {
    println!("Hello, world!");
}

struct Computer {
    memory_map: MemoryMap,
    processor: Processor
}
impl Computer {
    
}
