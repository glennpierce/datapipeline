use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::Arc;


#[derive(Debug)]
pub enum ElementError {
    FAILED_TO_READ_INPUT,
}

pub type ElementResult<T> = Result<T, Element>;

enum ElementPadType {
    INPUT,
    OUTPUT,
}


enum ElementPadDataType {
    STRING,
    NUMERIC,
    DATETIME,
    FILE
}

enum ElementType {
    SOURCE,
    SINK,
    FILTER,
}

struct ElementPad {
    name : String,
    pad_type : ElementPadType,
    pad_data_type: ElementPadDataType,
    comm : Arc<(Sender<f64>, Receiver<f64>)>,
}

impl ElementPad {
    pub fn new(name : String, pad_type : ElementPadType, pad_data_type: ElementPadDataType) -> Self {
        ElementPad{
            name : name,
            pad_type : pad_type,
            pad_data_type : pad_data_type,
            comm : Arc::new(mpsc::channel()),
 //            let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        }
    }

    // pub fn send_on_output(&self, index : u8) {
    //     //let mut map = self.inputs; 
    //     self.outputs[index].comm[0].send(id).unwrap();
    // }
}

struct BaseElement {
    name : String,
    inputs : Vec<ElementPad>,
    output : ElementPad,
}

impl BaseElement {
    pub fn new(name: String) -> Self {
        BaseElement{
            name : name,
            inputs : Vec::new(),
            output : ElementPad::new ("output".to_string(), ElementPadType::OUTPUT, ElementPadDataType::STRING),
        }
    }

    pub fn add_input(&mut self, name : String, pad_type : ElementPadType, pad_data_type : ElementPadDataType) {
        //let mut map = self.inputs; 
        self.inputs.push(ElementPad::new (name, pad_type, pad_data_type));
    }

    // pub fn add_output(&mut self, name : String, pad_type : ElementPadType, pad_data_type : ElementPadDataType) {
    //     //let mut map = self.inputs; 
    //     self.output = ElementPad::new (name, pad_type, pad_data_type);
    // }
}

trait Element {
    //fn new(&mut self) -> Self;
    
    fn initalise(&mut self);
    fn run(&self);
    
    //fn attach_output_pad(source_pad : &str, sink_element : BaseElement, sink : &str) {
        //if source.pad_type != ElementPadType.INPUT {
        //    panic();
        //}
    //}
}

// trait PipeLine {
//     fn create_element(name : &str, type : ElementType);
// }


struct FileSourceElement {
    base : BaseElement,
    file: File,
}

impl FileSourceElement {
    fn new(name: String, file : File) -> Self {
        let mut element = FileSourceElement { base : BaseElement::new(name), file :file };

        element.initalise();
        
        element
    }
}

impl Element for FileSourceElement {
    


    fn initalise(&mut self) {
        //self.base.add_input("input_pad1_filesource".to_string(), ElementPadDataType::FILE);
       // self.base.add_output("data".to_string(), ElementPadType::OUTPUT, ElementPadDataType::FILE);
    }

    fn run(&self) {
        

//(Sender<String>, Receiver<String>)

//let (mut tx, mut rx) = mpsc::channel();

   //     let (tx, rx): (Sender<String>, Receiver<String>) = self.base.output.comm;

//let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

         //let channels = &self.base.output.comm;
        // let tx = channels.0;
//         let rx = channels.1;

    //    let thread_tx = tx.clone();
    //    let thread_rx = rx.clone();

        let mut local_comm = self.base.output.comm.clone();

        // Spawn off an expensive computation
        thread::spawn(move|| {

            // Read from file and stream data to the output pad.
            let mut reader = BufReader::new(&self.file);

            for line in reader.lines() {
                let l = line.unwrap();
                //self.base.outputs[0].comm
                //println!("{}", l); 
                local_comm.0.send(3.0).unwrap();
             }

        });

             
    


    }
}



struct ConsoleEchoElement {
    base : BaseElement,
}

impl ConsoleEchoElement {
    fn new(name: String) -> Self {
        let mut element = ConsoleEchoElement { base : BaseElement::new(name) };
        element.initalise();
        element
    }
}

impl Element for ConsoleEchoElement {
    
    
    fn initalise(&mut self) {
        // TODO needs multiple types
        self.base.add_input("data".to_string(), ElementPadType::INPUT, ElementPadDataType::FILE);
    }

    fn run(&self) {
        //let (_, rx): (_, Receiver<String>) = mpsc::channel();
        //println!("{}", rx.recv().unwrap());    

        // let channels = &self.base.inputs[0].comm;
        // let rx = &(channels.1);
        // println!("{}", rx.recv().unwrap());    
    }
}



// struct ElementPipeline {
//     elements : Vec<Element>,
// }


// class Race {
// Frame createFrame() { return new Frame(); }
// Wheel createWheel() { return new Wheel(); }
// Bicycle createBicycle(Frame frame, Wheel front, Wheel rear) {
// return new Bicycle(frame, front, rear);
// }
// // return a complete bicycle without needing any arguments
// Bicycle completeBicycle() {
// Frame frame = createFrame();
// Wheel frontWheel = createWheel();
// Wheel rearWheel = createWheel();
// return createBicycle(frame, frontWheel, rearWheel);
// }
// Race createRace() {
// Bicycle bike1 = completeBicycle();
// Bicycle bike2 = completeBicycle();
// ...
// }
// }






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use ::Element;
        use ::FileSourceElement;
        use ::ConsoleEchoElement;
        use std::fs::File;

        match File::open("data/test.txt") {
            Ok(file) => {
              let file_source_element = FileSourceElement::new("My Source".to_string(), file);

              let echo_element = ConsoleEchoElement::new("My Echo".to_string());

             // file_source_element.attach_output_pad(source_pad : &str, sink_element : BaseElement, sink : &str) {
              //echo_element.attach_pads(file_source_element, );

              file_source_element.run();
              echo_element.run();
            },
            Err(e) => {
                // fallback in case of failure.
                // you could log the error, panic, or do anything else.
                println!("{}", e);
            }
        };

        
    }
}
