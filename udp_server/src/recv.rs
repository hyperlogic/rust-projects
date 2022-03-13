use std::net::UdpSocket;
use std::sync::mpsc;
use std::time;

use crate::packet::Packet;

struct Stats {
    num_packets: usize,
    num_bytes: usize,
}

#[allow(unreachable_code)]
pub fn recv_loop(socket: UdpSocket, packet_sender: mpsc::Sender<Packet>) -> std::io::Result<()> {
    {
        const MTU: usize = 1454;
        let mut stats = Stats {
            num_packets: 0,
            num_bytes: 0,
        };

        // receive packets and send them to the packet_sender
        loop {
            let mut buf = [0; MTU];
            let (amt, src) = socket.recv_from(&mut buf)?;
            let buf = &mut buf[..amt];

            stats.num_packets += 1;
            stats.num_bytes += amt;

            let packet = Packet {
                addr: src,
                timestamp: time::SystemTime::now(),
                data: buf.to_vec(),
            };

            packet_sender.send(packet).unwrap();

            // hack
            if stats.num_packets % 10 == 0 {
                println!(
                    "Received {} bytes/packet",
                    stats.num_bytes / stats.num_packets
                );
            }
        }
    } // the socket is closed here

    Ok(())
}
