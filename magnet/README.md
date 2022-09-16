## "Magnet" Crate

The Magnet crate / framework is intended to provide background thread operations to work in conjunction with a Rust based GUI framework such as *Slint* or *egui.* While Magnet's goal is to remain GUI platform agnostic, all of the examples in this workspace repository are constructed using **Slint** as the UI tool. 

There is always a challenge in setting up the threading in a GUI application, and the overall purpose of Magnet is to provide a minimal framework that is configurable for a wide range of applications. These include real time data acquisition or telemetry for embedded real time control applications in power systems, signal processing, and robotics. 

The library lib.rs file has two primary functions, which is a config and handler with signatures:
``` Rust 
 1. pub fn interrupt_config(mut self, sender : Sender<u64>, mut memory : SharedMemory )
 2. pub fn interrupt_handle(self, rx : Receiver<u64> )
```

The *SharedMemory* data type uses the Rust crossbeam library *ShardedLock* type, while the Sender\<T> and Receiver\<T> are crossbeam channels. Crossbeam is better for inter-thread communication than *std::sync::mpsc* and considering that anticipated growth of the framework this was chosen. 

The *interrupt_handle* function is meant to have a reader/writer to an external device as part of its overall functionality, such as reading or writing to a serial USB device or requesting TCP sockets or Modbus registers. 

### Goals

A primary goals of the framework is to reduce the exposure of threading business logic to the main()
function that is controlling the GUI, and hence allow the programmer to not have this clutter the primary 
functions for handling callbacks to the GUI. 

### Non-Goals

This framework is not meant to be an all encompassing runtime such as tokio. 

## Examples

There example which demonstrates configuring the background thread to run in a finite sense is
```shell
 cargo run --example finite_main_tic 
```
Note that in this example, the crossbeam channel reciever in the Framework is not really used,
and a reciever is implemented in the top level file. 


while the example which shows running continuously  is 
```shell
 cargo run --example forever_main_tic
```
and in the continuously running example the num_iters is set to zero. 

Also note that one can simply be in the neodyne root, as the project is setup as workspace holding the
magnet framework module and the motor_gui module. 