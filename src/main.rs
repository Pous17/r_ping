extern crate libc;
use std::io;
use std::io::Error;
use std::net::{ToSocketAddrs, IpAddr};

use crate::addrinfo::getaddrinfo;


// refer to this https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol
static ICPM_HEADER_SIZE: usize = 8;
static ICPM_ECHO_REQUEST: u8 = 8;

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

    // check if sum is overflowing the 32-bit memory slot (if so, wrap around)
    // whenever sum is overflowing, sum >> 16 (shift 16 bits to the right) will be 1, otherwise 0
    while (sum >> 16) > 0 {
        // 16 upper bits added to 16 lower bits until sum is 16 bits long
        sum = (sum & 0xffff) + (sum >> 16);
    } 
    
    // negating (flipping every bit)
    !sum as u16
}

fn get_ip() -> io::Result<Vec<IpAddr>> {
    let host = 

    let mut addrs = Vec::new()
    for addr in 
}


fn main() {
    // crete a raw socket
    // af_net is ipv4, sock_raw is raw socket, ipproto_icmp is icmp protocol
    let socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP) };

    //if the socket has a negative value the socket have failed to be created
    if socket < 0 {
        panic!("Failed to create socket: {:?}", Error::last_os_error());
    };
    
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

    let ping_ip = get_ip().unwrap(  );

    // build the packet
    let mut packet = vec![0; ICPM_HEADER_SIZE];
    let mut icpm_packet = unsafe { &mut *(packet.as_mut_ptr() as *mut Icpm) };
    icpm_packet.icpm_type = ICPM_ECHO_REQUEST;
    icpm_packet.icpm_code = 0;
    icpm_packet.icpm_identifier = 9;
    icpm_packet.icpm_sequence = 1;

    // calculate checksum
    icpm_packet.icpm_checksum = calculate_checksum(&packet);

    // TODO might need to support ipv6 later 
    // send the packet
    let dest_socket = libc::sockaddr_in {
        sin_family: libc::AF_INET as u8,
        sin_port: 0,
        sin_addr: libc::in_addr {
            s_addr: ping_ip
                .to_string()
                .parse::<u32>()
                .expect("Failed to parse destination IP")
                .to_be(), // to big endian
        },
        sin_zero: [0; 8],
        sin_len: 0,
    };

    let result = unsafe {
        libc::sendto (
            socket,
            packet.as_ptr() as *const libc::c_void,
            packet.len(),
            0,
            &dest_socket as *const _ as *const libc::sockaddr,
            std::mem::size_of_val(&dest_socket) as libc::socklen_t,
        )
    };

    if result < 0 {
        panic!("Failed to send packet: {:?}", Error::last_os_error());
    }

    println!("ICMP Echo Request sent to {}", ping_ip);
}
