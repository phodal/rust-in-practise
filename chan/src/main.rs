use crossbeam_channel::{unbounded, RecvError};
use std::thread;

fn main() {
    let (rpc_sender, rpc_receiver) = unbounded();

    thread::spawn(move || {
       loop {
           rpc_sender.send("hello");
       }
    });

    thread::spawn(move || {
       loop {
           match rpc_receiver.recv() {
               Ok(str) => {
                   println!("{:?}", str);
               }
               Err(_) => {}
           }
       }
    });

    loop {

    }
}
