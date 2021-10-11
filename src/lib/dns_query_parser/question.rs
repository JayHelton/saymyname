use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
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

    pub fn compose(questions: Questions) -> Vec<u8> {
        questions.iter().fold(Vec::new(), |mut acc, q| {
            println!("{:?}", q);
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
            println!("{:?}", std::str::from_utf8(&acc));
            acc
        })
    }
}
