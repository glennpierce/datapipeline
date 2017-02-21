use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::Arc;
use std::sync::deque::BufferPool;


struct Element {
    comm : (Sender<String>, Receiver<String>),
}

struct Element {
    comm : (Sender<String>, Receiver<String>),
}

impl Element {
    pub fn new(name : String) -> Self {
        Element{
            comm : mpsc::channel(),
        }
    }
}

struct FileSourceElement {
    base : Element,
    file: File,
}

impl FileSourceElement {
    fn new(name: String, file : File) -> Self {
        let mut element = FileSourceElement { base : Element::new(name), file : file };
        element
    }

// https://aturon.github.io/stability-dashboard/std/sync/deque/index.html

    fn run(&self) {


        let mut local_comm = self.base.comm.0.clone();
        //let mut local_comm_rx = self.base.comm.1.clone();
        
        let f = self.file.try_clone().unwrap();

        thread::spawn(move || {
            let mut reader = BufReader::new(f);
            for line in reader.lines() {
                let l = line.unwrap();
                local_comm.send(l).unwrap();
            }
        });

/*
        let mut local_comm = self.base.comm.clone();

        // Spawn off an expensive computation
        thread::spawn(move|| {

            // Read from file and stream data to the output pad.
            let mut reader = BufReader::new(&self.file);

            for line in reader.lines() {
                let l = line.unwrap();
                local_comm.0.send(l).unwrap();
             }

        });
        */


    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use ::Element;
        use ::FileSourceElement;
        use std::fs::File;

        match File::open("data/test.txt") {
            Ok(file) => {
              let file_source_element = FileSourceElement::new("My Source".to_string(), file);
              file_source_element.run();
            },
            Err(e) => {
                // fallback in case of failure.
                // you could log the error, panic, or do anything else.
                println!("{}", e);
            }
        };

        
    }
}
