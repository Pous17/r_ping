extern crate libc;
use std::io::Error;
use std::net::{Ipv4Addr, ToSocketAddrs, IpAddr, TcpStream};

// refer to this https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol
static ICPM_HEADER_SIZE: usize = 8;
static ICPM_ECHO_REQUEST: u8 = 8;
static ICPM_ECHO_REPLY: u8 = 0;
static IP_HEADER_SIZE: usize = 20;

// structure of a Icpm packet
#[repr(C)]
struct Icpm {
    icpm_type: u8,
    icpm_code: u8,
    icpm_checksum: u16,
    icpm_identifier: u16,
    icpm_sequence: u16,
}

fn calculate_checksum(packet: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i: usize = 0;

    // compose 16-bit words from two adjacent 8-bit bytes in packet
    while i < packet.len() {
        let word = (packet[i] as u32) << 8 | packet[i + 1] as u32;
        sum = sum.wrapping_add(word);
        i += 2;
    }

    while (sum >> 16) > 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    } 

    !sum as u16
}

fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Usage: Cargo run <ip>");
    }

    let address: Vec<_> = args[1].to_socket_addrs()?.collect();
    
    // checks for https
    // let https = address.iter().any(|x| {
    //     if let Ok(socket_addr) = x {
    //         if let IpAddr::V4(_) = socket_addr.ip() {
    //             return socket_addr.port() == 443;
    //         }
    //     }
    //     false
    // });

    Ok("Hello".to_string())
}

fn main() {
    // crete a raw socket
    // af_net is ipv4, sock_raw is raw socket, ipproto_icmp is icmp protocol
    let socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP) };

    //if the socket has a negative value the socket have failed to be created
    if socket < 0 {
        panic!("Failed to create socket: {:?}", Error::last_os_error());
    };
    
    // TODO more doc to read about that
    let one: libc::c_int = 1;
    let result = unsafe {
        libc::setsockopt(
            socket,
            libc::IPPROTO_IP,
            libc::IP_HDRINCL,
            // create a raw pointer to a constant variable, with infered type, then casted to a C void pointer (pointer to a value of unknown type)
            &one as *const _ as *const libc::c_void,
            std::mem::size_of_val(&one) as libc::socklen_t,
        )
    };

    if result < 0 {
        panic!("Failed to set socket options: {:?}", Error::last_os_error());
    }

    let ping_ip = get_ip();

}

