use std::collections::HashMap;
use std::io::Result;
use std::net::UdpSocket;

fn main() -> Result<()> {
    create_udp_server()
}

fn create_udp_server() -> Result<()> {
    let mut account_codes: HashMap<String, i32> = HashMap::new();
    {
        let socket = UdpSocket::bind("127.0.0.1:1012")
            .expect("Could not bind to socket");

        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("Received {} bytes from {}", amt, src);

        let buf = &buf[..amt];
        let mut code = String::new();
        let mut index = 0;
        while buf.len() > 0 {
            let c = buf[index];
            if c == 0 {
                break;
            }
            code.push(c as char);
            index += 1;
        }
        println!("Code: {}", code);
        account_codes.insert(code.to_string(), 0);
    }
    Ok(())
}
