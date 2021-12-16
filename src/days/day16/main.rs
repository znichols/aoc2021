use num_bigint::BigUint;
use num_traits::Num;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Packet {
    version: u8,
    t: u8,
    val: Option<u64>,
    sub_packets: Vec<Packet>,
}

fn read_packet(packet_binary: &str) -> Result<(Packet, usize), Box<dyn Error>> {
    let version = u8::from_str_radix(&packet_binary[..3], 2)?;
    let t = u8::from_str_radix(&packet_binary[3..6], 2)?;

    match t {
        4 => {
            let mut bit_groups: Vec<String> = Vec::new();
            let mut i = 6;
            let mut done = false;
            while !done {
                if packet_binary.chars().nth(i) == Some('0') {
                    done = true;
                }
                bit_groups.push(packet_binary[i + 1..i + 5].to_string());
                i += 5;
            }
            let val = u64::from_str_radix(&bit_groups.join(""), 2)?;
            Ok((
                Packet {
                    version,
                    t,
                    val: Some(val),
                    sub_packets: Vec::new(),
                },
                i,
            ))
        }
        _ => {
            let len_type = packet_binary.chars().nth(6).unwrap();
            let mut sub_packets: Vec<Packet> = Vec::new();
            let mut i = 0;
            if len_type == '0' {
                let subpacket_len = usize::from_str_radix(&packet_binary[7..22], 2)?;
                i = 22;
                while i - 22 < subpacket_len {
                    let s = read_packet(&packet_binary[i..])?;
                    i += s.1;
                    sub_packets.push(s.0);
                }
            } else {
                let subpacket_count = usize::from_str_radix(&packet_binary[7..18], 2)?;
                i = 18;
                for _ in 0..subpacket_count {
                    let s = read_packet(&packet_binary[i..])?;
                    i += s.1;
                    sub_packets.push(s.0);
                }
            }
            Ok((
                Packet {
                    version,
                    t,
                    val: None,
                    sub_packets,
                },
                i,
            ))
        }
    }
}

fn version_sum(packet: &Packet) -> usize {
    let mut s = packet.version as usize;
    for p in &packet.sub_packets {
        s += version_sum(p);
    }
    s
}

fn resolve(packet: &Packet) -> u64 {
    let subpacket_vals = packet.sub_packets.iter().map(resolve);
    match packet.t {
        0 => subpacket_vals.sum(),
        1 => subpacket_vals.product(),
        2 => subpacket_vals.min().unwrap(),
        3 => subpacket_vals.max().unwrap(),
        4 => packet.val.unwrap(),
        5 => {
            let v = subpacket_vals.collect::<Vec<u64>>();
            (v[0] > v[1]) as u64
        }
        6 => {
            let v = subpacket_vals.collect::<Vec<u64>>();
            (v[0] < v[1]) as u64
        }
        _ => {
            let v = subpacket_vals.collect::<Vec<u64>>();
            (v[0] == v[1]) as u64
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    for line in input.lines() {
        let int_from_hex = BigUint::from_str_radix(line, 16)?;
        let packet_binary = format!("{0:01$b}", int_from_hex, line.len() * 4);

        let packet_info = read_packet(&packet_binary)?;
        println!(
            "{}, {}",
            version_sum(&packet_info.0),
            resolve(&packet_info.0)
        );
    }

    Ok(())
}
