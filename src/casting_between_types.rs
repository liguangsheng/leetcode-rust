fn u8_slice_to_i32(v: &[u8]) -> i32 {
    std::str::FromStr::from_str(std::str::from_utf8(v).unwrap()).unwrap()
}

fn u8_slice_to_string() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_slice_to_i32() {
        let s = String::from("123456");
        let bytes = s.as_bytes();
        let u8_slice = &bytes[1..5];
        assert_eq!(u8_slice_to_i32(u8_slice), 2345i32);
    }
}
