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

    #[test]
    fn test_append() {
        let mut header_map = HeaderMap::new();

        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 2);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        assert_eq!(header_map.get("Content-Type").unwrap(), "text/html");

        header_map.append(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/plain"),
        );
        assert_eq!(header_map.len(), 3);
        let mut header_value_iter = header_map.get_all("content-type").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "application/json");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "text/plain");
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        assert_eq!(header_map.get("Content-Type").unwrap(), "text/html");

        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("image/jpeg"),
        );
        assert_eq!(header_map.len(), 4);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        let mut header_value_iter = header_map.get_all("Content-Type").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "text/html");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "image/jpeg");

        let mut iter = header_map.iter();
        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("content-type", "application/json")
        );
        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("content-type", "text/plain")
        );
        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("Content-Type", "text/html")
        );
        let (k, v) = iter.next().unwrap();
        assert_eq!(
            (k.as_raw_str(), v.to_str().unwrap()),
            ("Content-Type", "image/jpeg")
        )
    }

    #[test]
    fn test_remove() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        header_map.remove(HeaderName::from_bytes("content-type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        assert_eq!(header_map.get("content-type"), None);
        let mut header_map  = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove(HeaderName::from_bytes("content-type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-type").unwrap(), "text/html");
        assert_eq!(header_map.get("Content-Type").unwrap(), "text/html");
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove(HeaderName::from_bytes("Content-Type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        assert_eq!(header_map.get("Content-Type").unwrap(), "application/json");
    }

    #[test]
    fn test_remove_all() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        assert_eq!(header_map.len(), 1);
        assert_eq!(header_map.get("content-type").unwrap(), "application/json");
        header_map.remove_all(HeaderName::from_bytes("content-type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        assert_eq!(header_map.get("content-type"), None);
        let mut header_map  = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove_all(HeaderName::from_bytes("content-type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
        let mut header_map = HeaderMap::new();
        header_map.insert(
            HeaderName::from_bytes("content-type".as_bytes()).unwrap(),
            HeaderValue::from_static("application/json"),
        );
        header_map.append(
            HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
            HeaderValue::from_static("text/html"),
        );
        assert_eq!(header_map.len(), 2);
        header_map.remove_all(HeaderName::from_bytes("Content-Type".as_bytes()).unwrap());
        assert_eq!(header_map.len(), 0);
    }

    #[test]
    fn test_get_with_request() {
        let request = Request::builder()
            .uri("/")
            .header("Content-Type", "application/json")
            .header("Content-Type", "text/html")
            .header("content-type", "text/plain")
            .body(())
            .unwrap();

        assert_eq!(request.headers().get("content-type").unwrap(), "text/plain");
        assert_eq!(request.headers().get("Content-Type").unwrap(), "application/json");
        let mut header_value_iter = request.headers().get_all("Content-Type").iter();
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "application/json");
        let header_value = header_value_iter.next().unwrap();
        assert_eq!(header_value.to_str().unwrap(), "text/html");
    }
}
