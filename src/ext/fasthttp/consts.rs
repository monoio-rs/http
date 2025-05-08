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
        // FIXME: 是否需要将 "Content-type" 也加入到 trie 中？
        let mut builder = TrieBuilder::new();

        // RFC标准header
        builder.push(ACCEPT_RANGES);
        builder.push(ACCEPT_RANGES.as_str().to_ascii_lowercase());
        builder.push(ACCEPT_LANGUAGE);
        builder.push(ACCEPT_LANGUAGE.as_str().to_ascii_lowercase());
        builder.push(ACCEPT_ENCODING);
        builder.push(ACCEPT_ENCODING.as_str().to_ascii_lowercase());
        builder.push(AUTHORIZATION);
        builder.push(AUTHORIZATION.as_str().to_ascii_lowercase());
        builder.push("Expect");
        builder.push("expect");
        builder.push(CONTENT_TYPE);
        builder.push(CONTENT_TYPE.as_str().to_ascii_lowercase());
        builder.push(CONTENT_ENCODING);
        builder.push(CONTENT_ENCODING.as_str().to_ascii_lowercase());
        builder.push(CONTENT_RANGE);
        builder.push(CONTENT_RANGE.as_str().to_ascii_lowercase());
        builder.push(CONTENT_LENGTH);
        builder.push(CONTENT_LENGTH.as_str().to_ascii_lowercase());
        builder.push(COOKIE);
        builder.push(COOKIE.as_str().to_ascii_lowercase());
        builder.push(CONNECTION);
        builder.push(CONNECTION.as_str().to_ascii_lowercase());
        builder.push(DATE);
        builder.push(DATE.as_str().to_ascii_lowercase());
        builder.push(HOST);
        builder.push(HOST.as_str().to_ascii_lowercase());
        builder.push(IF_MODIFIED_SINCE);
        builder.push(IF_MODIFIED_SINCE.as_str().to_ascii_lowercase());
        builder.push(LOCATION);
        builder.push(LOCATION.as_str().to_ascii_lowercase());
        builder.push(LAST_MODIFIED);
        builder.push(LAST_MODIFIED.as_str().to_ascii_lowercase());
        builder.push(REFERER);
        builder.push(REFERER.as_str().to_ascii_lowercase());
        builder.push(RANGE);
        builder.push(RANGE.as_str().to_ascii_lowercase());
        builder.push(SERVER);
        builder.push(SERVER.as_str().to_ascii_lowercase());
        builder.push(SET_COOKIE);
        builder.push(SET_COOKIE.as_str().to_ascii_lowercase());
        builder.push(TRANSFER_ENCODING);
        builder.push(TRANSFER_ENCODING.as_str().to_ascii_lowercase());
        builder.push(USER_AGENT);
        builder.push(USER_AGENT.as_str().to_ascii_lowercase());
        builder.push(ORIGIN);
        builder.push(ORIGIN.as_str().to_ascii_lowercase());

        // 内部标准header
        builder.push("X-Common-Params");
        builder.push("x-common-params");
        builder.push("Use-Ppe");
        builder.push("use-ppe");
        builder.push("Tt-Logid");
        builder.push("tt-logid");
        builder.push("Tt-Env");
        builder.push("tt-env");

        let trie = builder.build();

        StandardHeaders(trie)
    }

    pub fn is_std_header(&self, header_name: &HeaderName) -> bool
    {
        self.0.exact_match(header_name)
    }
}

lazy_static! {
    pub(crate) static ref STANDARD_HEADERS: StandardHeaders = StandardHeaders::new();
}
