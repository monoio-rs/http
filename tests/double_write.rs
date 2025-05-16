#[cfg(feature = "double-write")]
#[cfg(not(feature = "fasthttp"))]
mod double_write_tests {
    use http::{HeaderMap, HeaderName, HeaderValue};

    #[test]
    fn feature_double_write() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("foo"),
            HeaderValue::from_static("bar"),
        );

        headers.insert(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("Bar"),
        );

        assert_eq!(headers.len(), 2);

        headers.append(
            HeaderName::from_bytes("foo1".as_bytes()).unwrap(),
            HeaderValue::from_static("baz1"),
        );
        headers.append(
            HeaderName::from_bytes("Foo1".as_bytes()).unwrap(),
            HeaderValue::from_static("baz2"),
        );
        headers.append(
            HeaderName::from_bytes("Foo1".as_bytes()).unwrap(),
            HeaderValue::from_static("baz3"),
        );

        assert_eq!(headers.get("foo"), Some(&HeaderValue::from_static("bar")));
        assert_eq!(headers.get("Foo"), Some(&HeaderValue::from_static("Bar")));
        assert_eq!(
            headers.get(HeaderName::from_bytes("Foo".as_bytes()).unwrap()),
            Some(&HeaderValue::from_static("Bar"))
        );
        assert_eq!(headers.get("foo1"), Some(&HeaderValue::from_static("baz1")));
        assert_eq!(
            headers.get_all("Foo1").into_iter().collect::<Vec<_>>(),
            vec![
                &HeaderValue::from_static("baz2"),
                &HeaderValue::from_static("baz3")
            ]
        );
        headers.remove("Foo1");
        assert_eq!(headers.get("foo1"), Some(&HeaderValue::from_static("baz1")));
        assert_eq!(headers.get("Foo1"), None);
    }
}
