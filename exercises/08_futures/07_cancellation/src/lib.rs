// TODO: fix the `assert_eq` at the end of the tests.
//  Do you understand why that's the resulting output?
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener, n_messages: usize, timeout: Duration) -> Vec<u8> {
    let mut buffer = Vec::new();
    for _ in 0..n_messages {
        let (mut stream, _) = listener.accept().await.unwrap();
        //let _ here doesn't mean an immediate drop, the timeout future fully runs.
        let _ = tokio::time::timeout(timeout, async {
            stream.read_to_end(&mut buffer).await.unwrap();
        })
        .await;
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let messages = vec!["hello", "from", "this", "task"];
        let timeout = Duration::from_millis(20);
        let handle = tokio::spawn(run(listener, messages.len(), timeout.clone()));

        for message in messages {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (_, mut writer) = socket.split();

            let (beginning, end) = message.split_at(message.len() / 2);

            // Send first half
            writer.write_all(beginning.as_bytes()).await.unwrap();
            tokio::time::sleep(timeout * 2).await;
            writer.write_all(end.as_bytes()).await.unwrap();

            // Close the write side of the socket
            let _ = writer.shutdown().await;
        }

        let buffered = handle.await.unwrap();
        let buffered = std::str::from_utf8(&buffered).unwrap();
        // What's actually happening:

        // For each message (e.g., "hello"):

        // Client sends "he" (first half)
        // Client sleeps 40ms
        // Meanwhile, server's 20ms timeout fires → cancels read_to_end mid-read
        // Only "he" is in the buffer when cancellation happens
        // Second half "llo" never arrives (timeout already fired)
        // The "two characters" pattern isn't about await points—it's that each message splits at len/2, and only the first half arrives before the timeout:

        // "hello" → "he" (len 5, split at 2)
        // "from" → "fr" (len 4, split at 2)
        // "this" → "th" (len 4, split at 2)
        // "task" → "ta" (len 4, split at 2)
        // Result: "hefrthta"
        assert_eq!(buffered, "hefrthta");
    }
}
