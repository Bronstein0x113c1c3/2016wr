use std::fs::File;
use std::io::copy;
use std::thread;
use ureq::get;

fn write_file(number: i32) {
    let resp = get("http://tailieuso.tlu.edu.vn/flowpaper/services/view.php")
        .query("doc", "57346364846697234711889407769857573585")
        .query("format", "jpg")
        .query("page", format!("{}",number).as_str())
        .query("subfolder", "57/34/63/")
        .call()
        .unwrap();

        let mut reader = resp.into_reader();
        let mut file = File::create(format!("workdir/page{}.jpg",number).as_str()).expect("creation failed");
    
        // Copy contents from the reader to the file
        copy(&mut reader, &mut file).expect("write failed");
    
        println!("Created a file page{}.jpg",number);
}

fn main(){
    let mut list_thread = vec![];
    for i in 1..336{
        let thread = thread::spawn(move ||{
            write_file(i);
        });
        list_thread.push(thread);
    }
    for thread in list_thread{
        thread.join().unwrap();
    }
}