#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world!");
}
// #[no_mangle]
// pub extern "C" fn _start() {
//     println!("start fn!");
// }

use std::net::TcpListener;

#[no_mangle]
pub extern "C" fn _start() {
    println!("start server fn!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // let stream = stream.unwrap();
        match stream {
            Ok(_) => println!("ok"),
            Err(_) => println!("err"),
        };

        println!("Connection established!");
    }
}
