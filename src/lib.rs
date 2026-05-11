use log_overflow::{log, log_init, Severity};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const MAGIC: u8 = 0x67;

struct metadata {
    name: String,
    length: String,
    artist: String,
}

//#[derive(Serialize, Deserialize, Debug)]
struct packet {
    header: u8,
    metadata: metadata,
    chunk: u32,
    size: u32,
    data: Vec<u8>, //temporary placeholder
}

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                log(Severity::INFO, "Connection closed by client");
                return
            }
            Ok(n) => {
                if n == 0 || buf[0] != MAGIC {
                    log(Severity::CRITICAL, "Invalid magic byte");
                    return
                }
                log(Severity::DEBUG, "magic byte correct");
                let first_metadata = metadata {
                    name: String::from("Foo is bar"),
                    length: String::from("1:23"),
                    artist: String::from("foobar"),
                };
                let first_packet = packet {
                    header: 0x67,
                    metadata: first_metadata,
                    chunk: 1,
                    size: 1234,
                    data: vec![0, 1, 2, 3, 4],
                };
                let encoded: Vec<u8> = bincode::serialize(&first_packet).unwrap();
                socket.write_all(&encoded).await.unwrap();
                log(Severity::DEBUG, "sent first packet");
                return
            }
            Err(e   ) => {
                log(Severity::WARNING, &format!("Failed to read from socket: {}", e));
                return
            }

        }
    }
}

async fn request(ip: &str) -> tokio::io::Result<()>{
    log_init("network-music-protocol", "~/.cache/network-music-protocol", true);
    log(Severity::DEBUG, "Logging started");
    let mut stream = TcpStream::connect(ip).await?;
    log(Severity::DEBUG, "connecting to ip_placeholder");
    stream.write_all(&[0x67]).await?;
    log(Severity::DEBUG, "magic byte sent");
    Ok(())
}

async fn listen() {
    log_init("network-music-protocol", "~/.cache/network-music-protocol", true);
    log(Severity::DEBUG, "Logging started");
    let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
    log(Severity::DEBUG, "Tcplistener opened and listening on port 1234");
    loop {
        let (socket , _) = listener.accept().await.unwrap();
        log(Severity::INFO, "Got connection");
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        listen().await;
    }
    #[tokio::test]
    async fn request_test() {
        request("127.0.0.1:1234").await;
    }
}
