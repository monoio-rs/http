#[cfg(feature = "fasthttp")]
mod fasthttp_tests {
    use http::{HeaderMap, HeaderName, HeaderValue, Request};

    #[test]
    fn test_insert() {
        let mut header_map = HeaderMap::new();

        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        assert_eq!(header_map.get("Content-Type").unwrap(), "application/json");

        header_map.insert(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("Content-Type").unwrap(), "text/html");
        assert_eq!(header_map.get("Content-Type").unwrap(), "text/html");

        header_map.insert(
            HeaderName::from_bytes("content-length".as_bytes()).unwrap(),
            HeaderValue::from_static("-1"),
        );
        assert_eq!(header_map.len(), 2);

        let mut iter = header_map.iter();

        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("Content-Type", "text/html")
        );

        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("content-length", "-1")
        );
    }

    // tes append for non-std headers
    #[test]
    fn test_append() {
        let mut header_map = HeaderMap::new();

        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar1"),
        );
        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar2"),
        );
        assert_eq!(header_map.len(), 2);
        assert_eq!(header_map.get("foo").unwrap(), "bar1");
        assert_eq!(header_map.get("Foo").unwrap(), "bar2");

        header_map.append(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar3"),
        );
        assert_eq!(header_map.len(), 3);
        let mut header_value_iter = header_map.get_all("foo").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar1");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar3");
        assert_eq!(header_map.get("foo").unwrap(), "bar1");
        assert_eq!(header_map.get("Foo").unwrap(), "bar2");

        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar4"),
        );
        assert_eq!(header_map.len(), 4);
        assert_eq!(header_map.get("foo").unwrap(), "bar1");
        let mut header_value_iter = header_map.get_all("Foo").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar2");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar4");

        let mut iter = header_map.iter();
        let (k, v) = iter.next().unwrap();
        assert_eq!((k.as_raw_str(), v.to_str().unwrap()), ("foo", "bar1"));
        let (k, v) = iter.next().unwrap();
        assert_eq!((k.as_raw_str(), v.to_str().unwrap()), ("foo", "bar3"));
        let (k, v) = iter.next().unwrap();
        assert_eq!((k.as_raw_str(), v.to_str().unwrap()), ("Foo", "bar2"));
        let (k, v) = iter.next().unwrap();
        assert_eq!((k.as_raw_str(), v.to_str().unwrap()), ("Foo", "bar4"))
    }

    #[test]
    fn test_append_special_std_header() {
        // test append for special std header, should equal to insert
        let mut header_map = HeaderMap::new();

        header_map.insert(
            HeaderName::from_bytes("content-length".as_bytes()).unwrap(),
            HeaderValue::from_static("1"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Length".as_bytes()).unwrap(),
            HeaderValue::from_static("2"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-length").unwrap(), "2");
        assert_eq!(header_map.get("Content-Length").unwrap(), "2");

        header_map.append(
            HeaderName::from_bytes("content-length".as_bytes()).unwrap(),
            HeaderValue::from_static("3"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-length").unwrap(), "3");

        header_map.append(
            HeaderName::from_bytes("Content-Length".as_bytes()).unwrap(),
            HeaderValue::from_static("4"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-length").unwrap(), "4");
    }

    #[test]
    fn test_remove() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar1"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("foo").unwrap(), "bar1");

        header_map.remove(HeaderName::from_bytes("foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        assert_eq!(header_map.get("foo"), None);
        let mut header_map = HeaderMap::new();

        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar1"),
        );
        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar2"),
        );
        assert_eq!(header_map.len(), 2);

        header_map.remove(HeaderName::from_bytes("foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("foo").unwrap(), "bar2");
        assert_eq!(header_map.get("Foo").unwrap(), "bar2");

        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar1"),
        );
        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("bar2"),
        );
        assert_eq!(header_map.len(), 2);

        header_map.remove(HeaderName::from_bytes("Foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("foo").unwrap(), "bar1");
        assert_eq!(header_map.get("Foo").unwrap(), "bar1");
    }

    #[test]
    fn test_remove_all() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("foo1"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("foo").unwrap(), "foo1");
        header_map.remove_all(HeaderName::from_bytes("foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        assert_eq!(header_map.get("foo"), None);
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("foo1"),
        );
        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("foo2"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove_all(HeaderName::from_bytes("foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("foo".as_bytes()).unwrap(),
            HeaderValue::from_static("foo1"),
        );
        header_map.append(
            HeaderName::from_bytes("Foo".as_bytes()).unwrap(),
            HeaderValue::from_static("foo2"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove_all(HeaderName::from_bytes("Foo".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
    }

    #[test]
    fn test_get_with_request() {
        let request = Request::builder()
            .uri("/")
            .header("Foo", "bar1")
            .header("Foo", "bar2")
            .header("foo", "bar3")
            .body(())
            .unwrap();

        assert_eq!(request.headers().get("foo").unwrap(), "bar3");
        assert_eq!(request.headers().get("Foo").unwrap(), "bar1");
        let mut header_value_iter = request.headers().get_all("Foo").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar1");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "bar2");
    }
}
