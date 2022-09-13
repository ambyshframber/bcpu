#![feature(result_flattening)]
#![feature(exclusive_range_pattern)]
#![feature(bigint_helper_methods)]
#![feature(type_name_of_val)]

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
