use std::collections::HashMap;
use std::net::UdpSocket;
use std::panic;
use std::sync::mpsc;
use std::thread;
use std::time;

struct Packet {
    addr: std::net::SocketAddr,
    timestamp: time::SystemTime,
    data: Vec<u8>,
}

#[allow(unreachable_code)]
fn recv_loop(socket: UdpSocket, channel_sender: mpsc::Sender<Packet>) -> std::io::Result<()> {
    {
        const MTU: usize = 1454;

        // receive packets and push them on the channel
        loop {
            let mut buf = [0; MTU];
            let (amt, src) = socket.recv_from(&mut buf)?;
            let buf = &mut buf[..amt];

            //println!("Received {} bytes from {}", amt, src);
            //println!("{:?}", buf);

            let packet = Packet {
                addr: src,
                timestamp: time::SystemTime::now(),
                data: buf.to_vec(),
            };

            channel_sender.send(packet).unwrap();
        }
    } // the socket is closed here
    Ok(())
}

#[allow(unreachable_code)]
fn update_loop(_socket: UdpSocket, channel_receiver: mpsc::Receiver<Packet>) -> std::io::Result<()> {
    {
        const ONE_SECOND: time::Duration = time::Duration::from_millis(1000);
        const USER_TIMEOUT: time::Duration = time::Duration::from_millis(5000);

        let mut packet_count = 0;
        let mut users_map: HashMap<std::net::SocketAddr, Packet> = HashMap::new();

        loop {
            thread::sleep(ONE_SECOND);

            // update users_map with the most recent packets
            while let Ok(packet) = channel_receiver.try_recv() {
                println!("packet.addr = {}, packet.data = {:?}", packet.addr, packet.data);
                users_map.insert(packet.addr, packet);
                packet_count = packet_count + 1;
            }

            // remove any users that have timed out.
            let now = time::SystemTime::now();
            let mut addrs_to_remove: Vec<std::net::SocketAddr> = Vec::new();
            for (addr, packet) in users_map.iter() {
                let elapsed = now.duration_since(packet.timestamp).unwrap();
                if elapsed > USER_TIMEOUT {
                    addrs_to_remove.push(*addr);
                }
            }
            for addr in &addrs_to_remove {
                println!("Removing stale user {}", addr);
                users_map.remove(&addr);
            }

            // TODO: for each user send all other users that have been updated.
            //socket.send_to(buf, &src)?;

            println!("Tick! packet_count = {}, users = {}", packet_count, users_map.len());
        }
    } // the socket is closed here
    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    let (channel_tx, channel_rx): (mpsc::Sender<Packet>, mpsc::Receiver<Packet>) = mpsc::channel();

    let recv_socket = socket.try_clone()?;
    let recv_thread = thread::spawn(|| recv_loop(recv_socket, channel_tx));

    let update_socket = socket.try_clone()?;
    let update_thread = thread::spawn(|| update_loop(update_socket, channel_rx));

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
