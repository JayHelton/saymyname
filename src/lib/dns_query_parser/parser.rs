use super::{
    header::Header,
    question::{Question, Questions},
};

#[derive(Debug)]
pub struct DnsMessage {
    header: Header,
    questions: Questions,
}

pub struct Parser;

impl Parser {
    /// This method parses the u8 array into a serialized object.
    /// In really should return errors based on the https://datatracker.ietf.org/doc/html/rfc1035
    /// but i didnt want to mess with that.
    fn parse_header(buf: &[u8]) -> Header {
        Header::parse(buf)
    }

    fn parse_questions(buf: &[u8], question_count: u16) -> (usize, Questions) {
        Question::parse(buf, question_count)
    }

    pub fn parse(buf: &[u8]) -> DnsMessage {
        // Header is only the first 12 bytes
        let header = Parser::parse_header(&buf[..12]);
        // TODO(jayhelton) when parsing a response, we would need to capture an offset to know
        // where the questions ended. We would then start from there to parse the answers given by
        // a nameserver.
        let (_offset, questions) = Parser::parse_questions(&buf[12..], header.question_count);
        DnsMessage { header, questions }
    }

    pub fn compose(msg: DnsMessage) -> Vec<u8> {
        let mut composed = vec![];
        let mut header = Header::compose(msg.header);
        composed.append(&mut header);
        composed
    }
}
