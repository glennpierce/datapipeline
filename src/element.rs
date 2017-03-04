use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use pipeline::PipeLineStreamFormat;


#[derive(Debug)]
pub enum ElementError {
    FAILED_TO_READ_INPUT,
}

pub type ElementResult<T> = Result<T, Element>;

pub type ElementPadConnection = (SyncSender<PipeLineStreamFormat>, Arc<Mutex<Receiver<PipeLineStreamFormat>>>);

#[derive(Clone, Debug)]
pub enum ElementPadType {
    INPUT,
    OUTPUT,
}

#[derive(Clone, Debug)]
pub enum ElementPadDataType {
    STRING,
    NUMERIC,
    DATETIME,
    FILE
}

#[derive(Clone, Debug)]
pub enum ElementType {
    SOURCE,
    SINK,
    FILTER,
}

pub struct ElementPad {
    pub name : String,
    pub pad_type : ElementPadType,
    pub pad_data_type: ElementPadDataType,
    pub conn : ElementPadConnection
}

impl ElementPad {
    pub fn new(name : String, pad_type : ElementPadType, pad_data_type: ElementPadDataType) -> Self {

        let channel = sync_channel::<PipeLineStreamFormat>(1000);
        let sender = channel.0; //Arc::new( Mutex::new(channel.0));
        let receiver = Arc::new(Mutex::new(channel.1));
     
        ElementPad{
            name : name,
            pad_type : pad_type,
            pad_data_type : pad_data_type,
            conn : (sender, receiver),
        }
    }

    // pub fn send_on_output(&self, index : u8) {
    //     //let mut map = self.inputs; 
    //     self.outputs[index].comm[0].send(id).unwrap();
    // }
}


pub trait Element : Send {

    //type ElementType = MyIterator<'a>;

    fn get_name(&self) -> &str;
    
    //fn run(&self, output : SyncSender<PipeLineStreamFormat>, input : Arc<Mutex<Receiver<PipeLineStreamFormat>>>);
    fn run(&self);

    //fn get_input_pads(&mut self) -> &[&ElementPad];
    fn get_input_pad(&self) -> &ElementPad;
    fn get_output_pad(&self) -> &ElementPad;
}