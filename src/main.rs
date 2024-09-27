use std::io::{Cursor, Seek};
use std::net::TcpListener;
use std::thread::sleep;
use std::time::{Duration, Instant};
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
    let kernel_builder_start = Instant::now();
    let src = include_str!("./shader.cpp");

    let pro_que = ProQue::builder()
        .src(src)
        .dims(4 * 1000 * 200)
        .build().expect("Kernel error");

    let buffer = pro_que.create_buffer::<u8>().expect("Buffer");
    let mut vec = vec![0u8; buffer.len()];
    
    println!("Created kernel in {:?}", kernel_builder_start.elapsed());

    let server = TcpListener::bind("0.0.0.0:1084").unwrap();
    for stream in server.incoming() {
        let mut socket = accept(stream.unwrap()).expect("Failed to accept websocket");
        println!("Websocket connection accepted");
        let mut buf = Cursor::new(vec![0u8]);
        let mut scalar = 0u8;

        loop {
            buf.set_position(0);
            
            let frame_start = Instant::now();

            let kernel = pro_que.kernel_builder("generate")
                .arg(&buffer)
                .arg(scalar)
                .build().expect("Build error");
            
            unsafe { kernel.enq().expect("Failed to queue kernel"); }
            buffer.read(&mut vec).enq().expect("Read");
            println!("Rendered and copied frame from GPU in {:?}", frame_start.elapsed());
            
            let frame =  vec.clone();
            let packet_start = Instant::now();
            FrameBuffer { frame }.encode(&mut buf).expect("Failed to assemble buffer");
            println!("Generated packet i {:?}", packet_start.elapsed());

            let send_start = Instant::now();
            if let Err(error) = socket.send(Message::Binary(buf.clone().into_inner())) {
                println!("Error: Failed to write frame output");
                println!("{error}");
                break;
            }
            
            scalar += 10;
            println!("Sent frame in {:?}", send_start.elapsed());

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