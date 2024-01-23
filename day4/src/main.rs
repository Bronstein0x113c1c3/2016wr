use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::{fs::copy, sync::mpsc, thread};
use ureq::get;
fn crawl_receiver() -> (mpsc::Receiver<(Vec<u8>, u32)>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut list_child_thread = vec![];
        for number in 1..337 {
            let s = sender.clone();
            let thread = thread::spawn(move || {
                println!("started to read page {}", number);
                let resp = get("http://tailieuso.tlu.edu.vn/flowpaper/services/view.php")
                    .query("doc", "57346364846697234711889407769857573585")
                    .query("format", "jpg")
                    .query("page", format!("{}", number).as_str())
                    .query("subfolder", "57/34/63/")
                    .call()
                    .unwrap();
                println!("{}", resp.get_url());
                println!("received");
                let mut bytes = Vec::new();
                let mut res = resp.into_reader();
                res.read_to_end(&mut bytes).unwrap();
                s.send((bytes, number)).unwrap();
            });
            list_child_thread.push(thread)
        }
        for smaller_thread in list_child_thread{
            smaller_thread.join().unwrap();
        }
    });
    (receiver, handle)
}

fn main() {
    let (receiver, work) = crawl_receiver();
    work.join().unwrap();
    for (bytes, page) in receiver {
        println!("received bytes of page {}", page);
        let mut file =
            File::create(format!("workdir/page{}.jpg", page).as_str()).expect("creation failed");
        file.write_all(&bytes);
        println!("write done of page {}", page);
    }
}
// fn main() {
//     let receiver, main_work =
//     println!("Hello, world!");
// }
