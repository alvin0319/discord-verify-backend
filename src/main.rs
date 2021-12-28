use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    create_udp_server()
}

fn create_udp_server() -> std::io::Result<()> {
    let mut accountCodes: [str; i32] = [];
    {
        let socket = UdpSocket::bind("127.0.0.1:1012");

        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("Received {} bytes from {}", amt, src);

        let mut buf = &buf[..amt];
        let mut code = String::new();
        while buf.len() > 0 {
            let c = buf.read_u8()?;
            if c == 0 {
                break;
            }
            code.push(c as char);
        }
        println!("Code: {}", code);
        accountCodes.push(code);
    }
    Ok(())
}
