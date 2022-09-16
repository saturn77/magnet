## Neodymium Magnet ##

An Example Real Time Motor Controller GUI using the slint-ui framework.

<img src="magnet_gui/assets/Magnet_GUI.gif">

The slint-ui design languageis used to construct a modern looking GUI using appropriately themed colors and fonts and structured layout. 

This crate is intended to provide a good starting point or template for real time embedded
communications and control. Overall this repos has the following features and goals:

1. Use of the slint-ui framework for modern GUI construction
2. The use of background thread versus timers 
3. The employment of bi-directional callbacks between
   the slint-ui and the main() loop code
4. Repeated use of the Weak reference in Slint/Rust to accomplish 
   multiple callaback handling both in terms of responding to 

   a callback from Rust and invoking a callback from Rust
5. Providing a template like framework for rapidly developing
   an embedded communication and control project


The addition of real time plotting and supporting various communication protocols is envisioned. 


Pull requests are welcomed, particularly in adding different communication features to the GUI. 


## Running the design

From the main root of the workspace
```shell
 cargo run 
```
Note that these repository houses multiple crates. 
