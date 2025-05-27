//! consts for fasthttp
use crate::header::{
    ACCEPT_ENCODING, ACCEPT_LANGUAGE, ACCEPT_RANGES, AUTHORIZATION, CONNECTION, CONTENT_ENCODING,
    CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, COOKIE, DATE, HOST, IF_MODIFIED_SINCE,
    LAST_MODIFIED, LOCATION, ORIGIN, RANGE, REFERER, SERVER, SET_COOKIE, TRANSFER_ENCODING,
    USER_AGENT,
};
use crate::HeaderName;
use lazy_static::lazy_static;
use trie_rs::{Trie, TrieBuilder};

pub(crate) struct StandardHeaders(Trie<u8>);

impl StandardHeaders {
    fn new() -> StandardHeaders {
        let mut builder = TrieBuilder::new();

        // RFC标准header
        builder.push(ACCEPT_RANGES);
        builder.push(ACCEPT_LANGUAGE);
        builder.push(ACCEPT_ENCODING);
        builder.push(AUTHORIZATION);
        builder.push("expect");
        builder.push(CONTENT_TYPE);
        builder.push(CONTENT_ENCODING);
        builder.push(CONTENT_RANGE);
        builder.push(CONTENT_LENGTH);
        builder.push(COOKIE);
        builder.push(CONNECTION);
        builder.push(DATE);
        builder.push(HOST);
        builder.push(IF_MODIFIED_SINCE);
        builder.push(LOCATION);
        builder.push(LAST_MODIFIED);
        builder.push(REFERER);
        builder.push(RANGE);
        builder.push(SERVER);
        builder.push(SET_COOKIE);
        builder.push(TRANSFER_ENCODING);
        builder.push(USER_AGENT);
        builder.push(ORIGIN);

        // 内部标准header
        builder.push("x-common-params");
        builder.push("use-ppe");
        builder.push("tt-logid");
        builder.push("tt-env");

        let trie = builder.build();

        StandardHeaders(trie)
    }

    pub fn is_match(&self, header_name: &HeaderName) -> bool {
        let lowercase_header = header_name.as_str().to_ascii_lowercase();
        self.0.exact_match(lowercase_header)
    }
}

pub(crate) struct NonAppendHeaders(Trie<u8>);

impl NonAppendHeaders {
    fn new() -> NonAppendHeaders {
        let mut builder = TrieBuilder::new();

        // 不可有多个值的特殊 std header
        // 调用 append 等同于 insert
        builder.push(CONTENT_LENGTH);
        builder.push(CONNECTION);
        builder.push(TRANSFER_ENCODING);
        builder.push(DATE);

        let trie = builder.build();

        NonAppendHeaders(trie)
    }

    pub fn is_match(&self, header_name: &HeaderName) -> bool {
        let lowercase_header = header_name.as_str().to_ascii_lowercase();
        self.0.exact_match(lowercase_header)
    }
}

lazy_static! {
    pub(super) static ref STANDARD_HEADERS: StandardHeaders = StandardHeaders::new();
    pub(super) static ref NON_APPEND_HEADERS: NonAppendHeaders = NonAppendHeaders::new();
}

/// check if is standard header
pub fn is_standard_header(header_name: &HeaderName) -> bool {
    STANDARD_HEADERS.is_match(header_name)
}

/// check if is non-append standard header
pub fn is_non_append_header(header_name: &HeaderName) -> bool {
    NON_APPEND_HEADERS.is_match(header_name)
}

#[test]
fn test_is_std_header() {
    assert!(is_standard_header(&"Content-Type".parse().unwrap()));
    assert!(is_standard_header(&"content-type".parse().unwrap()));
    assert!(is_standard_header(&"CONTENT-TYPE".parse().unwrap()));
    assert_eq!(is_standard_header(&"content_type".parse().unwrap()), false);
}
