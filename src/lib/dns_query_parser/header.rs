use byteorder::{BigEndian, ByteOrder};

// some of these fields are actually way smaller than the data type provided
#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub query: u16,
    pub opcode: u16,
    pub authoritative: u16,
    pub truncated: u16,
    pub recursion_desired: u16,
    pub recursion_available: u16,
    pub authenticated_data: u16,
    pub checking_disabled: u16,
    pub response_code: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub ns_count: u16,
    pub additional_count: u16,
}

/// This module represents the flags that exist in the bits of the header.
/// We bitwise AND these constants to determine if any bits of the flags
/// have been set.
#[rustfmt::skip]
pub mod flags {
    pub const QUERY:               u16 = 0b1000_0000_0000_0000;
    pub const OPCODE_MASK:         u16 = 0b0111_1000_0000_0000;
    pub const AUTHORITATIVE:       u16 = 0b0000_0100_0000_0000;
    pub const TRUNCATED:           u16 = 0b0000_0010_0000_0000;
    pub const RECURSION_DESIRED:   u16 = 0b0000_0001_0000_0000;
    pub const RECURSION_AVAILABLE: u16 = 0b0000_0000_1000_0000;
    pub const AUTHENTICATED_DATA:  u16 = 0b0000_0000_0010_0000;
    pub const CHECKING_DISABLED:   u16 = 0b0000_0000_0001_0000;
    pub const RESPONSE_CODE_MASK:  u16 = 0b0000_0000_0000_1111;
}

impl Header {
    pub fn parse(buf: &[u8]) -> Header {
        let id = BigEndian::read_u16(&buf[..2]);
        let flag_bytes = BigEndian::read_u16(&buf[2..4]);
        Header {
            id,
            query: flag_bytes & flags::QUERY,
            opcode: (flag_bytes & flags::OPCODE_MASK) >> flags::OPCODE_MASK.leading_zeros(),
            authoritative: flag_bytes & flags::AUTHORITATIVE,
            truncated: flag_bytes & flags::TRUNCATED,
            recursion_desired: flag_bytes & flags::RECURSION_DESIRED,
            recursion_available: flag_bytes & flags::RECURSION_AVAILABLE,
            authenticated_data: flag_bytes & flags::AUTHENTICATED_DATA,
            checking_disabled: flag_bytes & flags::CHECKING_DISABLED,
            response_code: flag_bytes & flags::RESPONSE_CODE_MASK,
            question_count: BigEndian::read_u16(&buf[4..6]),
            answer_count: BigEndian::read_u16(&buf[6..8]),
            ns_count: BigEndian::read_u16(&buf[8..10]),
            additional_count: BigEndian::read_u16(&buf[10..12]),
        }
    }

    pub fn compose(header: Header) -> Vec<u8> {
        // this probably could be more efficient with the owned data
        // but oh well
        let mut composed = vec![];
        let mut id_b = header.id.to_be_bytes().to_vec();
        composed.append(&mut id_b);
        println!("{:?}", composed);
        let mut buffer = 0u16;
        buffer |= header.query;
        buffer |= header.opcode << flags::OPCODE_MASK.leading_zeros();
        buffer |= header.authoritative;
        buffer |= header.truncated;
        buffer |= header.recursion_desired;
        buffer |= header.recursion_available;
        buffer |= header.authenticated_data;
        buffer |= header.checking_disabled;
        buffer |= header.response_code;
        let mut flag_bytes = buffer.to_be_bytes().to_vec();
        composed.append(&mut flag_bytes);
        let mut qc_b = header.question_count.to_be_bytes().to_vec();
        composed.append(&mut qc_b);
        let mut ac_b = header.answer_count.to_be_bytes().to_vec();
        composed.append(&mut ac_b);
        let mut ns_b = header.ns_count.to_be_bytes().to_vec();
        composed.append(&mut ns_b);
        let mut a_b = header.additional_count.to_be_bytes().to_vec();
        composed.append(&mut a_b);
        composed
    }
}
