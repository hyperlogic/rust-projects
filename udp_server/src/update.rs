use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::packet::Packet;

type UserMap = HashMap<std::net::SocketAddr, Packet>;

fn copy_packets(users_map: &mut UserMap, packet_receiver: &mpsc::Receiver<Packet>) {
    // update users_map with the most recent packets
    while let Ok(packet) = packet_receiver.try_recv() {
        println!(
            "packet.addr = {}, packet.data = {:?}",
            packet.addr, packet.data
        );
        users_map.insert(packet.addr, packet);
    }
}

fn remove_stale_users(users_map: &mut UserMap, now: &time::SystemTime) {
    const USER_TIMEOUT: time::Duration = time::Duration::from_millis(5000);

    // remove any users that have timed out.
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
}

#[allow(unreachable_code)]
pub fn update_loop(
    socket: UdpSocket,
    packet_receiver: mpsc::Receiver<Packet>,
) -> std::io::Result<()> {
    {
        const UPDATE_FREQUENCY: u64 = 10;
        const LOOP_DURATION: time::Duration =
            time::Duration::from_micros(1000000 / UPDATE_FREQUENCY);

        let mut users_map: UserMap = HashMap::new();

        loop {
            let loop_start = time::SystemTime::now();

            copy_packets(&mut users_map, &packet_receiver);
            remove_stale_users(&mut users_map, &loop_start);

            for (my_addr, _) in users_map.iter() {
                for (addr, packet) in users_map.iter() {
                    if addr == my_addr {
                        continue;
                    }
                    socket.send_to(&packet.data, &my_addr)?;
                }
            }

            // TODO: for each user send all other users that have been updated.
            //socket.send_to(buf, &src)?;

            let elapsed = loop_start.elapsed().unwrap();

            println!("Tick! users = {}, elapsed = {:?}", users_map.len(), elapsed);

            if elapsed < LOOP_DURATION {
                thread::sleep(LOOP_DURATION - elapsed);
            }
        }
    } // the socket is closed here
    Ok(())
}
