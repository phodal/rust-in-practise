use crossbeam_channel::{unbounded, RecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (rpc_sender, rpc_receiver) = unbounded();

    thread::spawn(move || {
       loop {
           let _result = rpc_sender.send("hello");
           thread::sleep(Duration::from_secs(1));
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
