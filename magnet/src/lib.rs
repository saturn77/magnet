use crossbeam_channel::{Sender, Receiver};
use crossbeam_utils::sync::ShardedLock;



#[derive(Debug, Copy, Clone)]
pub struct Framework {
    tic       : u64,
    pub is_stopped : bool,
    pub interrupt_period_msec : std::time::Duration, 
}

pub struct SharedMemory {
    pub max_count : ShardedLock<u64>,
    pub is_stopped : ShardedLock<bool>,

}

impl SharedMemory {
    pub fn new() -> SharedMemory {
        SharedMemory {
            max_count : ShardedLock::new(0),
            is_stopped : ShardedLock::new(false),
        }
    }
}

impl Framework {
    pub fn new() -> Framework {
        Framework {
            tic   : 0,
            is_stopped : false,
            interrupt_period_msec : std::time::Duration::from_millis(1000),
        }
    }

    pub fn interrupt_config(mut self, sender : Sender<u64>, mut memory : SharedMemory ) {
        println!("Setting up background thread ...");
        std::thread::spawn( move || loop { 
                if *memory.max_count.get_mut().unwrap() == 0 {
                    
                }
                else if self.tic > *memory.max_count.get_mut().unwrap() {
                    println!("Reached end...");
                    memory.is_stopped = ShardedLock::new(true);
                    self.is_stopped = true;
                    println!("self.is_stopped == {:?}", self.is_stopped); 
                    break;
                }
                println!("Tic = {:?}", &self.tic); 
                sender.send(self.tic).unwrap(); 
                self.tic += 1;
                std::thread::sleep(self.interrupt_period_msec);
        }); 
    }

    pub fn interrupt_handle(self, rx : Receiver<u64> ){
        std::thread::spawn(move || loop {  
                if let Ok(value) = rx.try_recv(){ 
                    println!("Toc = {:?}", value);     
            } 
        });
    }
}



