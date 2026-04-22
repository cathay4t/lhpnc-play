// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;

use tokio_uring::net::TcpStream;

const REQUST_COUNT: usize = 10 * 1000 * 1000;

fn main() {
    let socket_addr: SocketAddr = "127.0.0.1:2345".parse().unwrap();

    tokio_uring::start(async {
        let stream = TcpStream::connect(socket_addr).await.unwrap();

        for i in 0..REQUST_COUNT {
            let name = format!("client_{i}");
            let mut buf = name.len().to_be_bytes().to_vec();
            buf.extend_from_slice(name.as_bytes());

            let (result, _) = stream.write(buf).submit().await;
            result.unwrap();

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

            // BUG: partial read? Since io_uring is sending this buffer to
            // kernel and get it back afterwards, how could we do
            // `read_all`
            let (result, _buf) = stream.read(buf).await;

            result.unwrap();
        }
    });
}
