pub fn encode(s: String) -> Vec<u8> {
    let n: usize = (s.len() + 1) / 2;
    let mut buf = vec![0; n];
    let mut ix = s.len() % 2;

    for c in s.chars() {
        let b = match c {
            '0' => 0x0,
            '1' => 0x1,
            '2' => 0x2,
            '3' => 0x3,
            '4' => 0x4,
            '5' => 0x5,
            '6' => 0x6,
            '7' => 0x7,
            '8' => 0x8,
            '9' => 0x9,
            _ => panic!("Invalid character"),
        };
        buf[ix / 2] = ((buf[ix / 2] as u16 * 16) % 255) as u8;
        buf[ix / 2] = ((buf[ix / 2] as u16 + b as u16) % 255) as u8;
        ix += 1;
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        assert_eq!(encode("".to_string()), vec![]);
        assert_eq!(encode("1".to_string()), vec![0x01]);
        assert_eq!(encode("12".to_string()), vec![0x12]);
        assert_eq!(encode("123".to_string()), vec![0x01, 0x23]);
        assert_eq!(encode("1234".to_string()), vec![0x12, 0x34]);
        assert_eq!(encode("20191231".to_string()), vec![0x20, 0x19, 0x12, 0x31]);
    }
}
