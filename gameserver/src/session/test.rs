use super::*;

#[test]
fn parses_split_token_login() {
    let mut data = Vec::new();
    data.extend_from_slice(&7u16.to_be_bytes());
    data.extend_from_slice(b"1_12345");
    data.extend_from_slice(&3u16.to_be_bytes());
    data.extend_from_slice(b"tok");

    assert_eq!(
        parse_login_request(&data).unwrap(),
        LoginRequest {
            account_id: "1_12345".into(),
            token: "tok".into(),
        }
    );
}

#[test]
fn parses_inline_token_login() {
    let account = b"1_12345#tok";
    let mut data = Vec::new();
    data.extend_from_slice(&(account.len() as u16).to_be_bytes());
    data.extend_from_slice(account);

    assert_eq!(
        parse_login_request(&data).unwrap(),
        LoginRequest {
            account_id: "1_12345".into(),
            token: "tok".into(),
        }
    );
}

#[test]
fn login_reply_matches_live_wire_shape() {
    assert_eq!(
        login_reply_payload(0x17eb591e),
        [0, 0, 0, 0, 0, 0, 0x17, 0xeb, 0x59, 0x1e]
    );
}
