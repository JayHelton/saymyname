use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub struct Question {
    name: String,
    qtype: u16,
    qclass: u16,
}

pub type Questions = Vec<Question>;

impl Question {
    pub fn parse(buf: &[u8], question_count: u16) -> (usize, Questions) {
        let mut questions = vec![];
        let mut remainder_offset = 0;
        for _ in 0..question_count {
            let mut segments = vec![];
            let mut offset = 0;
            for i in 0..buf.len() {
                if buf[i] == 0x00 {
                    offset = i as u8;
                    break;
                }
                if i == 0 {
                    offset = buf[i];
                } else {
                    if i > offset as usize {
                        offset = buf[i] + offset + 1;
                        segments.push(0x2E);
                    } else {
                        segments.push(buf[i]);
                    }
                }
            }
            remainder_offset = offset as usize + 1;
            let t = BigEndian::read_u16(&buf[remainder_offset..remainder_offset + 2]);
            remainder_offset = remainder_offset + 2;
            let c = BigEndian::read_u16(&buf[remainder_offset..remainder_offset + 2]);
            remainder_offset = remainder_offset + 2;
            questions.push(Question {
                name: std::str::from_utf8(&segments).unwrap().to_string(),
                qtype: t,
                qclass: c,
            })
        }
        (remainder_offset, questions)
    }
}
