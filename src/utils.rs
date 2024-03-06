
pub fn to_hex_int(ch: u8) -> u8 {
    let res = match ch {
        48..=57 => ch - 48,
        97..=102 => ch - 87,
        65..=70 => ch - 55,
        _ => unreachable!(),
    };
    res % 16
}

pub fn print_u8array(arr: &[u8]) {
    for &num in arr.iter() {
        print!("{:02x}", num);
    }
    // println!();
}

pub fn println_u8array(arr: &[u8]) {
    // for &num in arr.iter() {
    //     print!("{:02x}", num);
    // }
    print_u8array(arr);
    println!();
}
pub fn print_pattern(work: &[u8], hash: &[u8]) {
    print_u8array(work);
    print!(", ");
    print_u8array(&hash);
    println!("");
}

pub fn check_diff(hash: &[u8]) -> u8 {
    let mut diff = 0u8;
    let mut endnum = 0u8;
    for (_i, &num) in hash.iter().enumerate() {
        if num == 0u8 {
            diff += 8;
        } else {
            endnum = num;
            break;
        }
    }
    for ind in 0..7 {
        if (endnum >> (7 - ind)) == 0 {
            diff += 1;
        } else {
            break;
        }
    }
    diff
}