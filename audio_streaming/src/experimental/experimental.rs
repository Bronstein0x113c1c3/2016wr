use std::
    sync::{Arc, Mutex}
;

use tokio;

use rodio::{Decoder, OutputStream, Sink};
#[tokio::main]
async fn main() {
    // std::io::BufReader::new(inner)
    let file =
        std::io::BufReader::new(std::fs::File::open("list_songs/08 - Like To Be You.flac").unwrap());
    /*
    Audio need 3 things:
        1. buffers/payload.
        2. sample rate
        3. channels
    */
    println!("getting file done, start extracting to info");

    let mut decoder = Decoder::new(file).unwrap();
    let buffer: Vec<i16> = decoder.by_ref().collect();
    let res = rodio::buffer::SamplesBuffer::new(2, 48000, buffer);
    // let buffer: Vec<i16> = decoder.by_ref().collect();
    // let sample_rate = decoder.sample_rate();
    // let channels = decoder.channels();
    // tokio::task::yield_now()
    // rodio::buffer::SamplesBuffer::new(channels, sample_rate, data)
    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    // let mut res = UniformSourceIterator::new(decoder, 2u16, 48000u32);
    let s1 = sink.clone();
    let t1 = tokio::task::spawn(async move {
        // stream_handle.play_raw(decoder.convert_samples());

        /*
        to increase responsiveness, use yield_now() to increase the chance of other tasks to be scheduled..., especially with cpu bound or long-waiting i/o...
         */
        // tokio::task::yield_now().await;
        s1.lock().unwrap().append(res);
        println!("the queue is appended before yielding....");
        tokio::task::yield_now().await;
        println!("yield_now() is called....");
        // s1.lock().unwrap().stop();
        // rodio::source::
        // std::thread::sleep(std::time::Duration::from_secs(5));
    });
    let t2 = tokio::task::spawn(async move {
        sink.lock().unwrap().sleep_until_end();
        println!("waiting is called!!!!");
        tokio::task::yield_now().await;
        println!("now, yield...");
        


        println!("the player has been done!!!!")
    });
    t1.await.unwrap();
    // println!("asdfdsfsdsdaf????");
    t2.await.unwrap();
    // decoder.
}
