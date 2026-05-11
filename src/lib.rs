use log_overflow::{log, log_init, Severity};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};

const MAGIC: u8 = 0x67;

//#[derive(Serialize, Deserialize, Debug)]
struct packet {
    header: u8,
    chunk: u32,
    size: u32,
    data: String, //temporary placeholder
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
