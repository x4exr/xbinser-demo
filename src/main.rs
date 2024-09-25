use std::io::{Cursor, Seek};
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use ocl::ProQue;
use tungstenite::{accept, Message};
use xbinser::encoding::Encoded;
use xbinser_macros::StructEncoded;

/// One-directional structure.
#[derive(Debug, StructEncoded)]
struct FrameBuffer {
    frame: Vec<u8>
}
fn main() {
    let src = include_str!("./shader.cpp");

    let pro_que = ProQue::builder()
        .src(src)
        .dims(4 * 1920 * 1080)
        .build().expect("Kernel error");

    let buffer = pro_que.create_buffer::<u8>().expect("Buffer");
    let mut vec = vec![0u8; buffer.len()];

    let kernel = pro_que.kernel_builder("generate")
        .arg(&buffer)
        .arg(10.0f32)
        .build().expect("Build error");

    let server = TcpListener::bind("0.0.0.0:1084").unwrap();
    for stream in server.incoming() {
        let mut socket = accept(stream.unwrap()).expect("Failed to accept websocket");
        println!("Websocket connection accepted");

        loop {
            unsafe { kernel.enq().expect("Failed to queue kernel"); }
            buffer.read(&mut vec).enq().expect("Read");
            let mut buf = Cursor::new(vec![0u8]);
            FrameBuffer { frame: vec.clone() }.encode(&mut buf).expect("Failed to assemble buffer");

            if let Err(error) = socket.send(Message::Binary(buf.into_inner())) {
                println!("Error: Failed to write frame output");
                println!("{error}");

                break;
            }

            // sleep(Duration::from_millis(16));
        }
    }
}

// use std::net::TcpListener;
// use std::thread::spawn;
// use tungstenite::{accept, Message};
//
// /// A WebSocket echo server
// fn main () {
//     let server = TcpListener::bind("127.0.0.1:1084").unwrap();
//     for stream in server.incoming() {
//         spawn (move || {
//             let mut websocket = accept(stream.unwrap()).unwrap();
//             loop {
//                 let msg = websocket.read().unwrap();
//
//                 // We do not want to send back ping/pong messages.
//                 if msg.is_binary() || msg.is_text() {
//                     println!("{msg}");
//                     websocket.send(Message::Binary(vec![10, 20])).unwrap();
//                 }
//             }
//         });
//     }
// }