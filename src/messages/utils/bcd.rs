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

#[test]
fn test_encode() {
    assert_eq!(encode("".to_string()), vec![]);
    assert_eq!(encode("1".to_string()), vec![0x01]);
    assert_eq!(encode("12".to_string()), vec![0x12]);
    assert_eq!(encode("123".to_string()), vec![0x01, 0x23]);
    assert_eq!(encode("1234".to_string()), vec![0x12, 0x34]);
    assert_eq!(encode("20191231".to_string()), vec![0x20, 0x19, 0x12, 0x31]);
}

// pub fn decode(bytes: Vec<u8>) -> String {
//     let mut s = String::new();
//     for b in bytes {
//         let c = match b & 0xf0 {
//             0x00 => '0',
//             0x10 => '1',
//             0x20 => '2',
//             0x30 => '3',
//             0x40 => '4',
//             0x50 => '5',
//             0x60 => '6',
//             0x70 => '7',
//             0x80 => '8',
//             0x90 => '9',
//             _ => panic!("Invalid byte"),
//         };
//         s.push(c);
//         let c = match b & 0x0f {
//             0x00 => '0',
//             0x01 => '1',
//             0x02 => '2',
//             0x03 => '3',
//             0x04 => '4',
//             0x05 => '5',
//             0x06 => '6',
//             0x07 => '7',
//             0x08 => '8',
//             0x09 => '9',
//             _ => panic!("Invalid byte"),
//         };
//         s.push(c);
//     }
//     s
// }

// #[test]
// fn test_decode() {
//     assert_eq!(decode(vec![]), "");
//     assert_eq!(decode(vec![0x01]), "01");
//     assert_eq!(decode(vec![0x12]), "12");
//     assert_eq!(decode(vec![0x01, 0x23]), "0123");
//     assert_eq!(decode(vec![0x12, 0x34]), "1234");
//     assert_eq!(decode(vec![0x20, 0x19, 0x12, 0x31]), "20191231");
// }
