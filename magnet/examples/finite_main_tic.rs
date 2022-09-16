use crossbeam_channel::{unbounded};
use std::io::Result;
use std::time;

fn main() -> Result<()> {
    let one_sec = time::Duration::from_millis(1000);
    println!("\r\n\r\n");
    println!("\t**** Neodymimum::Magnet Framework.");
    println!("\t**** Example finite_main_tic.rs\r\n");
    let (s, r) = unbounded(); 
    let r_main = r.clone();


    let mut tic : i32 = 0; 

    // Directly set the number of iterations with the 
    // tic assignment 
    // This is a more basic example than the forever_main_tic
    // example which actually uses the framework 
    
    std::thread::spawn( move || loop { 
        if tic > 5 {
            println!("Reached end (of sender thread)...");
            break;
        }
        println!("Tic = {:?}", tic); 
        s.send(tic).unwrap(); 
        tic += 1;
        std::thread::sleep(one_sec);
    });

    loop {
        match r_main.try_recv() {
            Ok(value) => { 
                println!("Main Toc = {:?}", value);
                if value >= 5 {
                    println!("Reached end of iterations in main loop.");
                    return Ok(())
                }     
            }
            _ => (),
        } 
    }
    
    
}