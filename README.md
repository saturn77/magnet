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
   multiple callaback handling both in terms of responding to a![Magnet_GUI](https://user-images.githubusercontent.com/5698048/190684457-6e534550-0769-4a4d-b1b6-66228c39612f.gif)

   a callback from Rust and invoking a callback from Rust
5. Providing a template like framework for rapidly developing
   an embedded communication and control project![Magnet_GUI](https://user-images.githubusercontent.com/5698048/190684410-84d42246-ec35-443f-aba7-06dfaad88298.gif)


The addition of real time plotting and supporting various communication protocols is envisioned. 


Pull requests are welcomed, particularly in adding different communication features to the GUI. 


## Running the design

From the main root of the workspace
```shell
 cargo run 
```
Note that these repository houses multiple crates. 
