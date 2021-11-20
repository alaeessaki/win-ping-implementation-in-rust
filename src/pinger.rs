use winping::{Buffer, Pinger};
use std::net::ToSocketAddrs;


pub fn do_pings(ip: &str) {
    let soc_adress = (ip, 80).to_socket_addrs().expect("Could not parse IP Address");
    for adrr in soc_adress.into_iter(){
        let pinger = Pinger::new().unwrap();
        let mut buffer = Buffer::new();
         
        for _ in 0..4 {
            match pinger.send(adrr.ip(), &mut buffer) {
                Ok(rtt) => println!("Response time {} ms.", rtt),
                Err(err) => println!("{}.", err),
            }
        }
    }
}
