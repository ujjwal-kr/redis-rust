pub fn encode_resp_simple_string(s: &str) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    encoded.push(b'+');
    encoded.extend(s.as_bytes());
    encoded.extend(&[b'\r', b'\n']);
    encoded
}

pub fn encode_resp_error_string(s: &str) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    encoded.push(b'-');
    encoded.extend(s.as_bytes());
    encoded.extend(&[b'\r', b'\n']);
    encoded
}

pub fn encode_resp_integer(number: i64) -> Vec<u8> {
    let mut encoded: Vec<u8> = vec![];
    encoded.push(b':');
    encoded.extend(number.to_be_bytes());
    encoded.extend(&[b'\r', b'\n']);
    encoded
}