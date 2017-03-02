use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use element::{Element, ElementPad, ElementPadType, ElementPadDataType};
use std::{thread, time};
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct FileSourceElement {
    //base : BaseElement<'a>,
    reader: BufReader<File>,
}

impl FileSourceElement {
    fn new(name: String, filepath : &str) -> Self {
        // let mut element = FileSourceElement { base : BaseElement::new(name), file :file };

        // element.initalise();
        
        //"data/test.txt"
        // element
        let file : File = File::open(filepath).unwrap();
        FileSourceElement { reader :  BufReader::new(file) }
    }
}

impl Element for FileSourceElement {

    fn run(&mut self, position :  Arc<AtomicUsize>) {
        let mut i : usize = 0;
       // let max = position.load(Ordering::Relaxed);
      
        //println!("max {}", max);


// for line in file.lines() {
//         let l = line.unwrap();
//         println!("{}", l); 
//     }    

        //for line in &self.reader.lines() {
        for (index, line) in self.reader.by_ref().lines().enumerate() {
            let l = line.unwrap();
            //"2017-01-19 14:38:02.460741+00",17,"unknown"
            let fields : Vec<&str> = l.split(',').collect();
            println!("{}", l); 

            position.fetch_add(1, Ordering::SeqCst);
        }    


        // loop {

        //     let max = position.fetch_add(0, Ordering::SeqCst);

        //     println!("max {}", max);

        //     while i < max {
        //         //println!("Hello");

        //         //let v = position.fetch_add(1, Ordering::SeqCst);
        //         println!("{:?}", i);
        //         i += 1;

        //         thread::sleep(time::Duration::from_millis(100));
        //     }

        //     thread::sleep(time::Duration::from_millis(1000));
        // }
      
    }
}

// impl<'a> Element for FileSourceElement<'a> {
    
//     fn next(&self) -> &Element {
//         return self.base.next();
//     }

//     fn pipeline(&self) -> &Element {
//         return &self;
//     }

//     fn get_name(&self) -> &str {
//         return "FileSourceElement";
//     }

//     fn initalise(&mut self) {
//         //self.base.add_input("input_pad1_filesource".to_string(), ElementPadDataType::FILE);
//        // self.base.add_output("data".to_string(), ElementPadType::OUTPUT, ElementPadDataType::FILE);
//     }

//     fn run(&self) {
   
//     }
// }

