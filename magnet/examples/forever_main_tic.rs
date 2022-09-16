use magnet::{Framework, SharedMemory};
use crossbeam_channel::{bounded};
use crossbeam_utils::sync::ShardedLock; 

fn main() {
    println!("\r\n\r\n");
    println!("\t**** Neodymimum::Magnet Framework.");
    println!("\t**** Example forever_main_tic.rs\r\n");
    let (s, r) = bounded(0); 

    let nd = Framework::new();

    let mut memory : SharedMemory = SharedMemory::new();
    let num_iters = ShardedLock::new(0);

    memory.max_count = num_iters;
    
    nd.interrupt_config(s, memory);
    nd.interrupt_handle(r);
    
    loop {
    }
}