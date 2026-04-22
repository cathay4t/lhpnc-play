// SPDX-License-Identifier: Apache-2.0

use tokio_uring::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio_uring::start(async {
        hello_server().await?;
        Ok(())
    })
}

async fn hello_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:2345".parse()?)?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                // Spawn a concurrent task per connection to maintain high
                // throughput
                tokio_uring::spawn(handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Fatal error accepting connection: {}", e);
                break;
            }
        }
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    loop {
        let header_buf = vec![0u8; 8];
        let (result, header_buf) = stream.read(header_buf).await;
        result.unwrap();

        let payload_length = u64::from_be_bytes([
            header_buf[0],
            header_buf[1],
            header_buf[2],
            header_buf[3],
            header_buf[4],
            header_buf[5],
            header_buf[6],
            header_buf[7],
        ]) as usize;

        if payload_length > 1024 {
            eprintln!("Payload length exceeded the maximum 1024");
            return;
        }

        let buf = vec![0u8; payload_length];

        // BUG: partial read? Since io_uring is sending this buffer to kernel
        // and get it back afterwards, how could we do `read_all`
        let (result, buf) = stream.read(buf).await;

        let reply_len = result.unwrap() + "Hello ".len();
        let mut reply_buf = reply_len.to_be_bytes().to_vec();
        reply_buf.extend_from_slice("Hello ".as_bytes());
        reply_buf.extend_from_slice(&buf);

        let (result, _) = stream.write(reply_buf).submit().await;
        result.unwrap();
    }
}
