use sha1::{Sha1, Digest};
use base64::encode;
use std::net::{TcpStream};
use std::io::{Read, Write};

pub fn websocket_handshake(stream: &mut TcpStream, key: &str) {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hash = hasher.finalize();
    let accept_key = encode(hash);

    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\r\n",
        accept_key
    );

    stream.write_all(response.as_bytes()).unwrap();
    println!("Websocket connected");
}

pub fn read_frame(stream: &mut TcpStream) -> Option<String> {
    let mut header = [0; 2];
    if stream.read_exact(&mut header).is_err() { return None; }

    let fin = header[0] & 0x80 != 0;
    let opcode = header[0] & 0x0F;

    let masked = header[1] & 0x80 != 0;
    let mut payload_len = (header[1] & 0x7F) as usize;

    if payload_len == 126 {
        let mut extended = [0; 2];
        stream.read_exact(&mut extended).unwrap();
        payload_len = u16::from_be_bytes(extended) as usize;
    } else if payload_len == 127 {
        let mut extended = [0; 8];
        stream.read_exact(&mut extended).unwrap();
        payload_len = u64::from_be_bytes(extended) as usize;
    }

    let mut mask = [0; 4];
    if masked { stream.read_exact(&mut mask).unwrap(); }

    let mut payload = vec![0; payload_len];
    stream.read_exact(&mut payload).unwrap();

    if masked {
        for i in 0..payload_len {
            payload[i] ^= mask[i % 4];
        }
    }

    if opcode == 1 {
        Some(String::from_utf8(payload).unwrap())
    } else {
        None
    }
}

pub fn send_frame(stream: &mut TcpStream, message: &str) {
    let mut frame = vec![0x81];
    let len = message.len();

    if len < 126 {
        frame.push(len as u8);
    } else if len < 65536 {
        frame.push(126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }

    frame.extend_from_slice(message.as_bytes());
    stream.write_all(&frame).unwrap();
}
