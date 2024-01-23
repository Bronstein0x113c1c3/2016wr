use rodio::Source;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};
#[tokio::main]
async fn main() {
    let resp = reqwest::get("http://localhost:3000").await.unwrap();
    let bytes = resp.bytes().await.unwrap();
    let cursor = Cursor::new(bytes);

    let device = rodio::default_output_device().unwrap();
    let source = rodio::Decoder::new(cursor).unwrap();
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
