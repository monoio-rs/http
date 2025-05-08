use crate::ext::fasthttp::consts::STANDARD_HEADERS;
use crate::HeaderName;

const TO_LOWER: u8 = b'a' - b'A';

lazy_static::lazy_static! {
    static ref TO_LOWER_TABLE: [u8; 256] = {
        let mut a = [0u8; 256];
        for i in 0..256 {
            let mut c = i as u8;
            if (b'A'..=b'Z').contains(&c){
                c += TO_LOWER;
            }
            a[i] = c;
        }
        a
    };

    static ref TO_UPPER_TABLE: [u8; 256] = {
        let mut a = [0u8; 256];
        for i in 0..256 {
            let mut c = i as u8;
            if (b'a'..=b'z').contains(&c){
                c -= TO_LOWER;
            }
            a[i] = c;
        }
        a
    };
}

/// 只normalize标准header，其他自定义header保持原样透传
pub(crate) fn normalize_header_key(
    header_name: &HeaderName,
    disable_normalizing: bool,
) -> HeaderName {
    if disable_normalizing {
        return header_name.clone();
    }

    if !STANDARD_HEADERS.is_std_header(&(header_name.clone())) {
        return header_name.clone();
    }

    normalize_header_key2(&header_name).into()
}

pub(crate) fn normalize_header_key2<K>(header_name: &K) -> impl Into<HeaderName>
where
    K: Into<HeaderName> + Clone,
{
    let header_name: HeaderName = header_name.clone().into();
    let mut header_name_str = header_name.as_raw_str().to_string();
    // let normalized_header_name = header_name.as_str();
    let n = header_name_str.len();
    if n == 0 {
        return header_name;
    }

    unsafe {
        let header_name_bytes = header_name_str.as_bytes_mut();
        header_name_bytes[0] = TO_UPPER_TABLE[header_name_bytes[0] as usize];
        let mut i = 1;
        while i < n {
            let p = &mut header_name_bytes[i];
            if *p == b'-' {
                i += 1;
                if i < n {
                    header_name_bytes[i] = TO_UPPER_TABLE[header_name_bytes[i] as usize];
                    i += 1;
                }
                continue;
            }
            *p = TO_LOWER_TABLE[*p as usize];
            i += 1;
        }

        HeaderName::from_bytes(header_name_bytes).unwrap()
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::ext::fasthttp::header_name::{normalize_header_key, normalize_header_key2};
    use crate::HeaderName;
    use std::str::FromStr;

    #[test]
    fn test_normalize_header_key() {
        let mut header_name = HeaderName::from_str("content-type").unwrap();
        assert_eq!(
            normalize_header_key(&mut header_name, false).as_raw_str(),
            "Content-Type"
        );

        let mut header_name = HeaderName::from_str("Content-Type").unwrap();
        assert_eq!(
            normalize_header_key(&mut header_name, false).as_raw_str(),
            "Content-Type"
        );

        // 非标准header，不做处理
        let mut header_name = HeaderName::from_str("x-tt-agw").unwrap();
        assert_eq!(
            normalize_header_key(&mut header_name, false).as_raw_str(),
            "x-tt-agw"
        );
    }

    #[test]
    fn test_normalize_header_key2() {
        let mut header_name = HeaderName::from_str("content-type").unwrap();
        assert_eq!(
            normalize_header_key2(&mut header_name).into().as_raw_str(),
            "Content-Type"
        );

        let mut header_name = HeaderName::from_str("Content-Type").unwrap();
        assert_eq!(
            normalize_header_key2(&mut header_name).into().as_raw_str(),
            "Content-Type"
        );

        // 非标准header，不做处理
        let mut header_name = HeaderName::from_str("x-tt-agw").unwrap();
        assert_eq!(
            normalize_header_key2(&mut header_name).into().as_raw_str(),
            "X-Tt-Agw"
        );
    }
}
