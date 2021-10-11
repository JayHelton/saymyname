use std::str::from_utf8_mut;

use byteorder::{BigEndian, ByteOrder};

use super::question::Question;

#[derive(Debug)]
pub struct Answer {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
    pub tty: u32,
    pub rd_len: u16,
    pub r_data: String,
}

pub type Answers = Vec<Answer>;

impl Answer {
    pub fn compose(answers: Answers) -> Vec<u8> {
        answers.iter().fold(Vec::new(), |mut acc, q| {
            let segments = q.name.split(".").collect::<Vec<&str>>();
            for i in 0..segments.len() {
                let size = segments[i].len() as u8;
                acc.push(size);
                let mut bytes = segments[i].as_bytes().to_vec();
                acc.append(&mut bytes);

                // if we are on the last element of the segment, push a 0 byte
                // to denote the end of the name
                if i >= segments.len() - 1 {
                    acc.push(0x00);
                }
            }
            let mut tb = q.qtype.to_be_bytes().to_vec();
            acc.append(&mut tb);
            let mut qb = q.qclass.to_be_bytes().to_vec();
            acc.append(&mut qb);
            let mut ttyb = q.tty.to_be_bytes().to_vec();
            acc.append(&mut ttyb);
            let mut rdlenb = q.rd_len.to_be_bytes().to_vec();
            acc.append(&mut rdlenb);
            let ip_segments = q.r_data.split(".").collect::<Vec<&str>>();
            for i in 0..ip_segments.len() {
                let bytes = ip_segments[i].parse::<u8>().unwrap();
                acc.push(bytes);
            }
            acc
        })
    }
}
