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

        let channel = sync_channel::<PipeLineStreamFormat>(0);
        let receiver = Arc::new(Mutex::new(channel.1));
     
        ElementPad{
            name : name,
            pad_type : pad_type,
            pad_data_type : pad_data_type,
            conn : (channel.0, receiver),
        }
    }

    // pub fn send_on_output(&self, index : u8) {
    //     //let mut map = self.inputs; 
    //     self.outputs[index].comm[0].send(id).unwrap();
    // }
}








pub trait Element : Send {

    //type ElementType = MyIterator<'a>;

    //fn new(&mut self) -> Self;
    
    // fn pipeline(&self) -> &Element;

    // fn initalise(&mut self);
    fn get_name(&self) -> &str;
    
    fn run(&mut self, position : Arc<AtomicUsize>, output : SyncSender<PipeLineStreamFormat>, input : Receiver<PipeLineStreamFormat>);
    
    //fn get_input_pads(&mut self) -> &[&ElementPad];
    fn get_input_pad(&self) -> &ElementPad;
    fn get_output_pad(&self) -> &ElementPad;

    // fn get_previous_element(&self) -> &Element {

    // }

    // fn attach_output_pad(sink_element : BaseElement, sink : &str) {
    //     //if source.pad_type != ElementPadType.INPUT {
    //     //    panic();
    //     //}
    // }pipeline
}