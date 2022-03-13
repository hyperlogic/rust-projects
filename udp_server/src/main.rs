use std::net::UdpSocket;
use std::panic;
use std::sync::mpsc;
use std::thread;

mod packet;
use crate::packet::Packet;

mod recv;
mod update;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // connects recv thread to update thread.
    let (packet_sender, packet_receiver): (mpsc::Sender<Packet>, mpsc::Receiver<Packet>) =
        mpsc::channel();

    let recv_socket = socket.try_clone()?;
    let recv_thread = thread::spawn(|| crate::recv::recv_loop(recv_socket, packet_sender));

    let update_socket = socket.try_clone()?;
    let update_thread =
        thread::spawn(|| crate::update::update_loop(update_socket, packet_receiver));

    match recv_thread.join() {
        Ok(_) => println!("recv_thread complete"),
        Err(e) => panic::resume_unwind(e),
    }

    match update_thread.join() {
        Ok(_) => println!("update_thread complete"),
        Err(e) => panic::resume_unwind(e),
    }

    Ok(())
}
