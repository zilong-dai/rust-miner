use std::io::Write;
use std::sync::{Arc, Mutex};


use rand::Rng;

use std::thread::JoinHandle;

mod utils;
mod pool;
mod pattern;
use iron::ironhash;
use utils::{check_diff, print_pattern, print_u8array, println_u8array,to_hex_int};
use pattern::Pattern;

use log_tracing::auto_log;

const HEADER_SIZE: usize = 180;
const NONE_SIZE: usize = 24;
const HASH_SIZE: usize = 32;
struct Work {
    header: [u8; HEADER_SIZE],
    nonce: [u8; NONE_SIZE],
}


#[auto_log]
fn add() {
    for i in 0..10 {
        println!("{}", i);
    }
}



#[test]
fn test_hex_2_int() {
    assert_eq!(1, to_hex_int('1' as u8));
    assert_eq!(to_hex_int('1' as u8) << 4 + to_hex_int('6' as u8), 0x16);
}

#[test]
fn test_check_diff() {
    assert_eq!(check_diff(&[0x13]), 3);
}

#[test]
fn test_move() {
    let str1 = "helloworld".to_string();
    let str2 = &str1;
    println!("{}", str1);
    println!("{}", str2);
}

fn main() {
    let pool = <pool::Pool as clap::Parser>::parse();
    println!("{:?}", pool);
    let work_str = "00a016fae14f0010b7f200000000000000000070a9772a0dcf2abafc3ec43697b8639dc9802543db11ed7ad627509b5e44de3f4f101cc052e053a8bc87ac22051059787d177aa9b7891cd05ffb857ebd06b960fea6e61c57ed36ab4084e33b2cdbd8484db6b55b2143b6a260000000000000007d5fa49afe59db618cc9c7f1902f10e6d6d31f591a69e5485536526a7a880100000000000000000000000000000000000000000000000000005015770d00000000";
    let work: Vec<u8> = work_str
        .as_bytes()
        .chunks(2)
        .map(|chs| to_hex_int(chs[0]) * 16 + to_hex_int(chs[1]))
        .collect::<Vec<u8>>();

    // let pattern_arr = Arc::new(Mutex::new(Vec::<Pattern>::new()));
    let fd = std::fs::File::create("./iron-pattern-pld-mult.txt").unwrap();
    let pattern_fd = Arc::new(Mutex::new(
        fd,
    ));

    // let count = Arc::new(Mutex::new(0));
    // let mut work = Arc::new(work);
    for addr in [0x00u8, 0x5au8, 0xffu8] {
        let mut threads: Vec<JoinHandle<()>> = Vec::new();
        for index in 0..14 {
            // let mut work_thr = Arc::clone(&work);
            // let pattern_arr = Arc::clone(&pattern_arr);
            let pattern_fd = Arc::clone(&pattern_fd);
            let mut work = work.clone();

            work[174] = addr;
            // let count = Arc::clone(&count);
            let handel = std::thread::spawn(move || {
                println!("index: {}", index);
                let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                work[172] = index;
                let mut count = 0;
                loop {
                    // let work_arr = work_thr.deref();
                    // work[2] = rng.gen::<u8>() % 5 + index * 2;
                    // work[3] = rng.gen::<u8>() % 23;
                    work[173] = rng.gen::<u8>() % 126;
                    let hash = ironhash(&work);
                    // print!("hash: ");
                    // print_u8array(&hash);
                    // println!();
                    if check_diff(&hash) >= 24 {
                        {
                            // let mut cnt = count.lock().unwrap();
                            // let mut patt_arr = pattern_arr.lock().unwrap();
                            let mut pattern_fd = pattern_fd.lock().unwrap();
                            let pattern_work: [u8; 180] = work[..180].try_into().unwrap();
                            print!("Thread index {}: ", index);
                            // print_pattern(&work, &hash);
                            let pattern = Pattern::new(pattern_work.clone(),hash.clone());
                                
                            {
                                for &num in pattern.work().iter() {
                                    write!(pattern_fd, "{:02x}", num).unwrap();
                                }
                                write!(pattern_fd, ", ").unwrap();
                                for &num in pattern.hash().iter() {
                                    write!(pattern_fd, "{:02x}", num).unwrap();
                                }
                                writeln!(pattern_fd).unwrap();
                            }
                            // patt_arr.push(pattern);
                            // writeln!(pattern_fd, "{:?}", pattern).unwrap();
                            count += 1;
                            if count >=  126{
                                break;
                            }
                        }
                        work[4] = rng.gen::<u8>();
                        work[5] = rng.gen::<u8>();
                    } else {
                        let mut nonce: u64 = u64::from_le_bytes(work[4..12].try_into().unwrap());
                        nonce += 1;
                        work[4..12].clone_from_slice(&nonce.to_le_bytes());
                    }
                }
            });
            threads.push(handel);
        }

        threads.into_iter().for_each(|h| h.join().unwrap());
        {
            // let patt_arr = pattern_arr.lock().unwrap();
            // println!("{:?}", patt_arr[0]);
            // println!("{:?}", patt_arr[patt_arr.len() - 1]);
        }
    }
}