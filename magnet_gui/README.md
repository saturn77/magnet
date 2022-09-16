## "Magnet-GUI" Crate

An Example Real Time Motor Controller GUI using the slint-ui framework and the magnet create to provide
background threading. The slint-ui design languageis used to construct a modern looking 
GUI using appropriately themed colors and fonts and structured layout. 

Overall this example GUI has two major demonstration points:
1. The use of background thread
2. The employment of bi-directional callbacks between
   the slint-ui and the main() loop code
3. Repeated use of the Weak reference in Slint/Rust to accomplish 
   multiple callaback handling both in terms of responding to a
   a callback from Rust and invoking a callback from Rust


A typical use case of the background thred is to facilitate a communication function such as serial/USB or Ethernet to the embedded controller, often where a query/response methodology is employed. 


### Goals

A primary goals of this crate is to provide a good starting point or template for real time embedded
communications and control. The addition of real time plotting and supporting various communication protocols is envisioned. 


## Running the design

From the main root of the workspace
```shell
 cargo run 
```
Note that these repository houses multiple crates. 

