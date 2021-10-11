use byteorder::{BigEndian, ByteOrder};

// some of these fields are actually way smaller than the data type provided
#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub query: bool,
    pub opcode: u16,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub authenticated_data: bool,
    pub checking_disabled: bool,
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
            query: flag_bytes & flags::QUERY == 0,
            opcode: (flag_bytes & flags::OPCODE_MASK) >> flags::OPCODE_MASK.leading_zeros(),
            authoritative: flag_bytes & flags::AUTHORITATIVE != 0,
            truncated: flag_bytes & flags::TRUNCATED != 0,
            recursion_desired: flag_bytes & flags::RECURSION_DESIRED != 0,
            recursion_available: flag_bytes & flags::RECURSION_AVAILABLE != 0,
            authenticated_data: flag_bytes & flags::AUTHENTICATED_DATA != 0,
            checking_disabled: flag_bytes & flags::CHECKING_DISABLED != 0,
            response_code: flag_bytes & flags::RESPONSE_CODE_MASK,
            question_count: BigEndian::read_u16(&buf[4..6]),
            answer_count: BigEndian::read_u16(&buf[6..8]),
            ns_count: BigEndian::read_u16(&buf[8..10]),
            additional_count: BigEndian::read_u16(&buf[10..12]),
        }
    }
}
