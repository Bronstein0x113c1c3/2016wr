use std::{io::Read, net};

// use serde_json::json;
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct S{
//     payload: Vec<i16>,
// }

// fn main(){
//     let mut conn = net::TcpStream::connect("127.0.0.1:8080").unwrap();
//     let mut res = String::new();
//     conn.read_to_string(&mut res).unwrap();
//     // println!("{}",serde_json::to_value(res).unwrap());
//     let res = serde_json::from_str(&res).unwrap();
//     // println!("{:?}",res["payload"]);

// }
use serde::Deserialize;
use serde_json::Result;
use rodio::{source::Source, Decoder, OutputStream, Sink};
#[derive(Deserialize, Debug)]
struct Audio {
    payload: Vec<i16>,
    sample_rate: u32,
    channels: u16,
}

fn process_audio(audio: Audio){
    use std::sync::{Arc, Mutex};
    use std::thread;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    
    let sample_rate = audio.sample_rate;
    let channels = audio.channels; 

    println!("gathering up info done, starting setting to hear...");


    let buffer = Arc::new(Mutex::new(audio.payload));
    let sink = Arc::new(Mutex::new(sink));
    let sink_clone = Arc::clone(&sink);
    let buffer_clone = Arc::clone(&buffer);

    println!("everything done! happy listening");

    thread::spawn(move || {
        let binding = buffer_clone.lock().unwrap();
        let mut iter = binding.chunks(2000);
        loop {
            let chunk = match iter.next() {
                Some(chunk) => chunk.to_vec(),
                None => break,
            };

            let new_source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, chunk);

            let mut sink = sink_clone.lock().unwrap();
            sink.append(new_source);
        }
    }).join();

    sink.lock().unwrap().sleep_until_end();

}







fn main() -> Result<()> {
    let mut conn = net::TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("{}","connected, starting downloading the sound.....");
    let mut res = String::new();
    conn.read_to_string(&mut res).unwrap();
    // let data = r#"{"numbers": [1, 2, 3, 4, 5]}"#;
    println!("{}","sound downloaded, extracting....");

    let data = res.as_str();

    let deserialized: Audio = serde_json::from_str(data)?;

    // println!("{:?}", deserialized.channels);

    conn.shutdown(net::Shutdown::Both).unwrap();
    println!("{}","extracting done, setting things up");

    process_audio(deserialized);
    Ok(())
}
