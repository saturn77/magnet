//! An Example Real Time Motor Controller GUI using the
//! slint-ui framework and the magnet create to provide
//! background threading. The slint-ui design language
//! is used to construct an aesthically good looking 
//! GUI, using modern themed colors and fonts to give
//! the look and feel of a modern GUI. 
//! 
//! Overall this example GUI has two major demonstration points:
//! 1. The use of background thread
//! 2. The employment of bi-directional callbacks between
//!    the slint-ui and the main() loop code
//! 
//! Using the magnet::Framework to generate background 
//! threads provides for ergonomic code construction. 
//! This is the overall aim of the magnet crate - to provide 
//! the essential background threading whenever building 
//! a real time GUI and relieve the user from having to
//! construct this code. 
//! 
//! Attaching a communication function such as serialUSB
//! to the thread to read / write to the embedded controller
//! is a tpyical use case of the background thread.
//! 
use crossbeam_channel::{unbounded};
use std::time;

slint::slint!(import {Neodymium} from "ui/top_level.slint";);

fn main() {
    let one_sec = time::Duration::from_millis(1000);
    let mut clicked : i32 = 0; 
    println!("\r\n\r\n");
    println!("\t**** Neodymimum::Magnet Framework.");
    println!("\t**** Example Motor GUI with Slint-UI\r\n");

    let (s, r) = unbounded(); 

    let mut tic : i32 = 0; 
    
    
   // let neo_strong = neo.clone_strong();

    std::thread::spawn( move || loop { 
        // if tic > 5 {
        //     println!("Reached end (of sender thread)...");
        //     break;
        // }
        println!("Tic = {:?}", tic); 
        s.send(tic).unwrap(); 
        tic += 1;
        std::thread::sleep(one_sec);
    });
    
    let neo = Neodymium::new();
    let neo_weak = neo.as_weak();
    let neo_weak2 = neo.as_weak();

    std::thread::spawn( move || loop {
        match r.try_recv() {
            Ok(value) => { 
                //println!("Main Toc = {:?}", value);
                let msg2 : String = format!("Main TicToc =  {:?} \r", value);
                println!("{:?}", msg2);
                let msgx = slint::SharedString::from(&msg2); 
                let msgy = slint::SharedString::from(&msg2);
                let neo_weak_copy = neo_weak.clone();

                slint::invoke_from_event_loop( move || neo_weak_copy.unwrap().set_stringTic(msgx)).unwrap();
                let neo_weak_copy = neo_weak.clone();
                slint::invoke_from_event_loop( move || neo_weak_copy.unwrap().invoke_logger(msgy)).unwrap();
            }
            _ => (),
        } 
    });

    neo.on_alpha(move || {
        let neo = neo_weak2.clone(); 

        // Constructing the string to share with top_level.slint
        let msg1 : String = format!("Hello from Rust for the {:?} nth time !",clicked);
        let msg = slint::SharedString::from(&msg1);

        // Setting the StringX property in top_level.slint
        slint::invoke_from_event_loop( move || neo.unwrap().set_stringX(msg)).unwrap();
        let neo = neo_weak2.clone();
        slint::invoke_from_event_loop(move || neo.unwrap().set_alpha_num(clicked)).unwrap();
        //neo.set_stringX(msg);
        //neo.set_alpha_num(clicked); 
        clicked+=1;
    });

    neo.run();

    //println!("{:?}", msg2);



    



}
