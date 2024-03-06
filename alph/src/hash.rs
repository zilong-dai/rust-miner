use blake3;

pub fn alphhash(work: &[u8]) -> [u8; 32] {
    assert_eq!(work.len(), 326);
    let out1 = blake3::hash(work);
    let out = blake3::hash(out1.as_bytes());
    *out.as_bytes()
}

mod tests {
    use crate::alphhash;

    pub fn to_hex_int(ch: u8) -> u8 {
        let res = match ch {
            48..=57 => ch - 48,
            97..=102 => ch - 87,
            65..=70 => ch - 55,
            _ => unreachable!(),
        };
        res % 16
    }

    #[test]
    fn test_alphhash() {
        let work_str = "f5fde3982dc99e5a0000000000000000240000000000000000070000000000002cc909398dd748a99f5d0b70eb0d899003f6a63316bd32236e500000000000065dc852111a904f9fc802e3dba65446c63ef295337a158da6e11a000000000003c0a858836eef27ff90522ded62e93ad334eb605a1a848f118fff000000000004fe4283473a6454ac5dd42996e97518ba526cca1489e85422e7f400000000000381365059c6da68aa438ab0ca323479eddb61a708bdcfcb2da1e5000000000001ef514e652d70a448e39a3498d28db7666a1859196b6d1d7d828600000000000611c9a56d77e25d0fcbec558d788ee7642fd294ecabcce095ac07335bf22f5b435bce657b83859d1d58f29a80dda6a11e3065eb71c9b92413a2c93e378c677047145ca8fdbc2e8c7290562c6f61be3bd502f4c70b650019847b830000018ca48a8d941b078175";
        let work: Vec<u8> = work_str
            .as_bytes()
            .chunks(2)
            .map(|chs| to_hex_int(chs[0]) * 16 + to_hex_int(chs[1]))
            .collect::<Vec<u8>>();
        let out = alphhash(&work);

        let hash_hex: String = out.iter().map(|num| format!("{:02x}", num)).collect();

        assert_eq!(hash_hex, "00000001f8ebc0093b967cd8561b17cafb0c5056d27b2f91c9cc00ec0787b204");
        // for &num in out.iter(){
        //     print!("{:02x}", num);
        //     let str = format!("{:02x}", str);
        // }
        // println!();
    }
}
