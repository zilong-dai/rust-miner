use core::num;
use log::info;
use serde_json::Value;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug)]
pub struct Pool {
    pub url: String,
    pub username: String,
    pub workername: String,
    pub passwd: String,

    pub stream: TcpStream,
    pub sockbuf: String,
    pub jobid: String,
    pub submitid: u32,
    pub newwork: bool,
    pub calc: bool,
    pub work: Work<326>,
}

#[derive(Debug)]
pub struct Work<const HEADER_SIZE: usize> {
    pub header: [u8; HEADER_SIZE],
    pub from_group: u8,
    pub to_group: u8,
    pub job_id: String,
}

impl<const HEADER_SIZE: usize> Work<HEADER_SIZE> {
    fn new() -> Self {
        todo!()
    }
    fn empty() -> Self {
        Self {
            header: [0u8; HEADER_SIZE],
            from_group: 0,
            to_group: 0,
            job_id: "0000".to_owned(),
        }
    }
}
// fn resolve_host(hostname_port: &str) -> Result<SocketAddr> {
// let socketaddr = hostname_port.to_socket_addrs()?.next().ok_or_else(|| {
//     io::Error::new(
//         io::ErrorKind::AddrNotAvailable,
//         format!("Could not find destination {hostname_port}"),
//     )
// })?;
// Ok(socketaddr)
// }

const NONCE_SIZE: usize = 24;

impl Pool {
    pub fn new(url: &str, username: &str, workername: &str) -> Self {
        info!("connect pool {} start", url);
        // let sockaddr = std::net::SocketAddr::from_str(url).unwrap();
        // info!("{}", sockaddr);
        let sockaddr = url
            .to_socket_addrs()
            .expect("doamin to ip error")
            .next()
            .expect("fist ip:port");
        info!("url to ip:port: {}", sockaddr);
        let stream =
            TcpStream::connect_timeout(&sockaddr, Duration::from_secs(5)).expect("connect failed");

        // let stream = TcpStream::connect(url).expect("connect error");
        // stream.set_nodelay(true).expect("set_nodelay call failed");
        // let stream = TcpStream::connect_timeout(&sockaddr, Duration::new(10, 0)).expect("connect failed");
        // {
        //     Ok(stream) => stream,
        //     Err(e) => {
        //         panic!("Failed to connect pool{}: {}", url, e);
        //     }
        // };

        info!("connect pool {} end", url);
        return Pool {
            url: url.to_owned(),
            username: username.to_owned(),
            workername: workername.to_owned(),
            passwd: "******".to_owned(),
            stream: stream,
            sockbuf: String::new(),
            jobid: "1".to_owned(),
            submitid: 0,
            newwork: false,
            calc: false,
            work: Work::empty(),
        };
    }

    fn send_message(&mut self, message: &String) {
        match self.stream.write(message.as_bytes()) {
            Ok(len) => info!("send {} bytes: {}", len, message),
            Err(e) => {
                panic!("Failed to send: {}", e);
            }
        };
    }

    fn recv_message(&mut self) -> String {
        let mut buf = [0u8; 1024];
        match self.stream.read(&mut buf[..]) {
            Ok(_) => String::from_utf8_lossy(&buf).to_string(),
            Err(e) => {
                panic!("Failed to recv: {}", e);
            }
        }
    }

    fn stratum_authorize(&mut self) {
        // let recv_buf=  [0u8; 1024];
        info!("stratum_authorize start");
        // {"id":3,"method":"mining.authorize","params":["1HCxMTaVRL5ct6rUvo992VMj3Fa8hL8bqt2NcTRkkLsd"]}
        let auth_message = format!(
            "{{\"id\":3,\"method\":\"mining.authorize\",\"params\":[\"{}\"]}}",
            self.username
        );

        self.send_message(&auth_message);
        // let recv_message = self.recv_message();
        // info!("{}", recv_message);
        info!("stratum_authorize end");
    }

    fn startum_subscribe(&mut self) {
        // let recv_buf=  [0u8; 1024];
        let subs_message = format!(
            "{{\"id\":3,\"method\":\"mining.authorize\",\"params\":[\"{}\"]}}",
            self.username
        );

        self.send_message(&subs_message);
        let recv_message = self.recv_message();
        info!("{}", recv_message);
    }

    fn stratum_parse_target(&mut self, message: &str) {
        // todo!()
    }

    fn startum_notify(&mut self, message: &str) {
        // info!("notify: {}", message);

        let binding =
            serde_json::from_str::<serde_json::Value>(message).expect("parse notify params error");
        let params = binding.as_object().expect("parse map error");

        info!("headerBlob: {}", params.get("headerBlob").unwrap());

        if let Some(from_group) = params.get("fromGroup") {
            self.work.from_group = from_group.as_u64().expect("fromGroup is not int") as u8;
        } else {
            self.calc = false;
            info!("fromGroup parse error");
        }

        if let Some(to_group) = params.get("toGroup") {
            self.work.to_group = to_group.as_u64().expect("fromGroup is not int") as u8;
        } else {
            self.calc = false;
            info!("toGroup parse error");
        }

        if let Some(job_id) = params.get("jobId") {
            self.work.job_id = job_id.as_str().expect("jobId is not string").to_string();
        } else {
            self.calc = false;
            info!("jobId parse error");
        }

        if let Some(job_id) = params.get("headerBlob") {
            let header_blob = job_id
                .as_str()
                .expect("headerBlob is not string")
                .as_bytes();
            // info!("bytes {:?}", header_blob);
            if 2 * (self.work.header.len() - NONCE_SIZE) == header_blob.len() {
                // let header = &mut self.work.header;
                // {
                //     header
                //         .iter_mut()
                //         .zip(header_blob.chunks(2))
                //         .map(|(h, arr)| {
                //             *h = self.to_hex_int(arr[0]) << 4 + self.to_hex_int(arr[1])
                //         });
                // }
                for i in NONCE_SIZE..self.work.header.len() {
                    // info!(
                    //     "header {} {} {}",
                    //     i,
                    //     Pool::to_hex_int(header_blob[2 * (i - NONCE_SIZE)]),
                    //     Pool::to_hex_int(header_blob[2 * (i - NONCE_SIZE) + 1])
                    // );
                    self.work.header[i] = Pool::to_hex_int(header_blob[2 * (i - NONCE_SIZE)]) * 16
                        + Pool::to_hex_int(header_blob[2 * (i - NONCE_SIZE) + 1]);
                }
                let mut header = String::new();
                for num in self.work.header.iter() {
                    header = format!("{}{:02x}", header, num);
                }
                info!("{:?}", header);
                // .zip(self.work.header.iter())
            } else {
                self.calc = false;
                info!("headerBlob length {} error", header_blob.len());
            }
        } else {
            self.calc = false;
            info!("headerBlob parse error");
        }
    }

    pub fn stratum_submit(&mut self, job_id: &String, nonce: u64) {
        let nonce_hex = format!("{:x}", nonce);
        let from = 0u32;
        let to = 1u32;
        let message = format!("{{\"id\":{}, \"method\": \"mining.submit\", \"params\": {{\"jobId\": \"{}\",\"fromGroup\":{},\"toGroup\":{},\"nonce\":\"{}\",\"worker\":\"{}\"}}}}", 5, self.jobid, from, to, nonce_hex, self.username);
        let solution = message + "\n";
        self.send_message(&solution);
    }

    fn startum_submitted(&mut self, message: &str) {
        info!("{}", message);
    }

    pub fn to_hex_int(ch: u8) -> u8 {
        let res = match ch {
            48..=57 => ch - 48,
            97..=102 => ch - 87,
            65..=70 => ch - 55,
            _ => unreachable!(),
        };
        res % 16
    }

    fn create_jobs(&mut self, json: Value) {
        todo!()
    }

    fn handle_other(&self, message: &str) {
        info!("{}", message);
    }

    pub fn handle_datastream(&mut self) {
        info!("{:?}", self);
        // self.startum_subscribe();
        self.stratum_authorize();

        loop {
            let mut reader = BufReader::new(&self.stream);
            let mut data: Vec<u8> = Vec::new();

            match reader.read_until(b'\n', &mut data) {
                // Ok(_) => info!("read {}", std::str::from_utf8(&data).expect("Could not convert to String")),
                // Err(_) => info!("read no data"),
                Ok(_) => {
                    let message = std::str::from_utf8(&data).expect("Could not convert to String");
                    // info!("{}", message);
                    let json_message: Value =
                        match serde_json::from_str::<serde_json::Value>(message) {
                            Ok(json) => json,
                            Err(_) => continue,
                        };

                    let json_map = json_message
                        .as_object()
                        .expect("message can not serde to map");

                    if let Some(method) = json_map.get("method") {
                        // info!("{}", method);
                        let params = json_message["params"]
                            .as_array()
                            .expect("message do not have params field")[0]
                            .to_string();
                        // info!("{:?}", params);

                        match method.as_str().expect("method cat not to string") {
                            "mining.notify" => self.startum_notify(&params),
                            "mining.set_target" | "mining.set_difficulty" => {
                                self.stratum_parse_target(&params)
                            }
                            "mining.submitted" => self.startum_submitted(&params),
                            _ => info!("unknown method"),
                        }
                    } else {
                        self.handle_other(message);
                        continue;
                    }
                }
                Err(e) => {
                    info!("Failed to Read Data {}", e);
                    continue;
                }
            }
        }
    }
}

// impl Debug for Pool {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Pool: {} connected to {} with {} active job(s) in queue ",
//             &self.username,
//             &self.url,
//             &self.active_job_queue.len(),
//         )
//     }
// }

mod tests {
    use super::Pool;

    #[test]
    fn test_pool_connect() {
        // env_logger::init();

        let mut pool = Pool::new(
            "hk.alephium.herominers.com:1199",
            "14SGTJouv8bJaPbpFsL6zRzBJ8povEbY1pKyCo658HHPZ",
            "longer",
        );
        pool.handle_datastream();
    }
}

// 00070000000000006a6860f663e2545367f4df143ddea724ec0f157f04b2bb6a83d5000000000001461d757095bbf336babe65e80934c497cd8393d9bf6a25ef7eca00000000000374b34b900db068ae482a25243bf720344d9ae6fbe648358a409f000000000000e47dc7e8ef7491e48a7f84781aec564b05b5d340c781661561e00000000000008259a530d45059344675177e2a02ef358e00674e2d9ce4910d610000000000009f007302d71ee52c23f46888f77f38487060f4c1c962d4abae92000000000002e38d11eb3708619a94d9927eb6967e568296e772e8c499c1bb138acfa383403a7acccb6158c14f7905dc2c0609a416f396472ebec78202b5566c78a556efb6f34aa1ced0191ab5cd102c0d8a18ccb61d3001a2b54bd40d98aec60000018d36db62261b03fbd6
// 00000000000000000000000000000000000000000000000000070000000000006a6860f663e2545367f4df143ddea724ec0f157f04b2bb6a83d5000000000001461d757095bbf336babe65e80934c497cd8393d9bf6a25ef7eca00000000000374b34b900db068ae482a25243bf720344d9ae6fbe648358a409f000000000000e47dc7e8ef7491e48a7f84781aec564b05b5d340c781661561e00000000000008259a530d45059344675177e2a02ef358e00674e2d9ce4910d610000000000009f007302d71ee52c23f46888f77f38487060f4c1c962d4abae92000000000002e38d11eb3708619a94d9927eb6967e568296e772e8c499c1bb138acfa383403a7acccb6158c14f7905dc2c0609a416f396472ebec78202b5566c78a556efb6f34aa1ced0191ab5cd102c0d8a18ccb61d3001a2b54bd40d98aec60000018d36db62261b03fbd6