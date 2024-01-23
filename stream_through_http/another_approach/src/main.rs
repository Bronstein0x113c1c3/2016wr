use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file
    let file = BufReader::new(File::open("rightforyou.mp3").unwrap());

    // Decode that sound file into a source
    let mut source = Decoder::new(file).unwrap();

    // Get the sample rate and the number of channels of the source
    let sample_rate = source.sample_rate();
    let channels = source.channels();
    let mut sink = Sink::try_new(&stream_handle).unwrap();

    //enough intro!!!

    use std::sync::{Arc, Mutex};
    use std::thread;

    let buffer: Vec<i16> = source.by_ref().collect();
    let buffer = Arc::new(Mutex::new(buffer));

    let sink = Arc::new(Mutex::new(sink));

    let sink_clone = Arc::clone(&sink);
    let buffer_clone = Arc::clone(&buffer);

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

    // Create a new source from the buffer
    // let source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, buffer);

    // // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
}
