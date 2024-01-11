use telnet::{Telnet, Event};
use std::io;
use std::thread;
use std::sync::mpsc;


fn main() {
    let mut telnet = Telnet::connect(("192.177.1.170", 9393), 256).expect("faild");


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // 将输入写入到 Telnet 服务器
        // telnet.write(input.as_bytes()).expect("msg");
        tx.send(input).unwrap();
        }
    });

    loop {
        let event = telnet.read_nonblocking().expect("Read error");

        if let Event::Data(buffer) = event {
            // Debug: print the data buffer
            let s = String::from_utf8_lossy(&buffer);
            println!("{:?}", s);
            // process the data buffer
        }

        let received = rx.try_recv();

        if received.is_ok() {
            // println!("{:?}", received.unwrap());
           telnet.write(received.unwrap().as_bytes()).expect("write error");
        }
    }
}