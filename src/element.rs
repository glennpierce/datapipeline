#[derive(Debug)]
pub enum ElementError {
    FAILED_TO_READ_INPUT,
}

pub type ElementResult<T> = Result<T, Element>;

pub enum ElementPadType {
    INPUT,
    OUTPUT,
}

pub enum ElementPadDataType {
    STRING,
    NUMERIC,
    DATETIME,
    FILE
}

pub enum ElementType {
    SOURCE,
    SINK,
    FILTER,
}

pub struct ElementPad {
    name : String,
    pad_type : ElementPadType,
    pad_data_type: ElementPadDataType,
}

impl ElementPad {
    pub fn new(name : String, pad_type : ElementPadType, pad_data_type: ElementPadDataType) -> Self {
        ElementPad{
            name : name,
            pad_type : pad_type,
            pad_data_type : pad_data_type,
        }
    }

    // pub fn send_on_output(&self, index : u8) {
    //     //let mut map = self.inputs; 
    //     self.outputs[index].comm[0].send(id).unwrap();
    // }
}








pub trait Element {
    //fn new(&mut self) -> Self;
    
    fn pipeline(&self) -> &Element;

    fn initalise(&mut self);
    fn get_name(&self) -> &str;
    
    fn run(&self);
    
    fn get_previous_element(&self) -> &Element {

    }
    // fn attach_output_pad(sink_element : BaseElement, sink : &str) {
    //     //if source.pad_type != ElementPadType.INPUT {
    //     //    panic();
    //     //}
    // }pipeline
}