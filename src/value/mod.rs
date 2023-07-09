pub mod function;
pub mod instruction;
pub mod template;
pub mod closure;
use std::rc::Rc;
// use std::ffi::CStr;
use std::ffi::c_void;
use std::cell::RefCell;

#[derive(Debug)]
pub struct KutObject {
    // dispatch: unsafe extern "C" fn(*mut KutValue, *const CStr, *mut KutValue, *mut c_void),
    pub data: *mut c_void,
}

// #[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
pub enum KutValue<'template> {
    Nil,
    Undefined,
    Number(f64),
    String(Rc<String>),
    List(Rc<Vec<KutValue<'template>>>),
    Func(Rc<KutClosure<'template>>),
    Reference(Rc<RefCell<KutValue<'template>>>),
    External(Rc<KutObject>),
}

#[derive(Debug)]
pub enum KutInstruction {
    NoOperation,

    MovRegister{destination: u8, source: u8},
    CallMethodR{ret_position: u8, arg_count: u8, subject: u8},
    CallMethodS{arg_count: u8, subject: u8},
    RetfMethodR{value: u8},
    RetfMethodS,
    PushValue1R{val1: u8},
    PushValue2R{val1: u8, val2: u8},
    PushValue3R{val1: u8, val2: u8, val3: u8},
    SwapValuesR{reg1: u8, reg2: u8},

    GetLiteralR{reg: u8, literal: u16},
    GetCaptureR{reg: u8, capture: u16},
    SetCaptureR{reg: u8, capture: u16},
    CaptureFunc{reg: u8, template: u16},
    PushLiteral{literal: u16},
    PushCapture{capture: u16},
    PushFuncStk{template: u16},
    PopCaptureS{capture: u16},
}

#[derive(Debug)]
pub enum KutCaptureInfo {
    Capture(u16),
    Register(u8),
}

#[derive(Debug)]
pub struct KutFunctionTemplate {
    pub instructions: Vec<KutInstruction>,
    pub capture_infos: Vec<KutCaptureInfo>,
    pub register_count: u8,
}

#[derive(Debug)]
pub struct KutClosure<'template> {
    pub template: &'template KutFunctionTemplate,
    pub captures: Vec<KutValue<'template>>,
}

#[derive(Debug)]
pub struct KutFunction<'closure,'template> where 'closure: 'template {
    pub closure: &'closure KutClosure<'template>,
    pub registers: Vec<KutValue<'template>>,
    pub call_stack: Vec<KutValue<'template>>,
}

#[derive(Debug)]
pub enum KutError {
    StackUnderflow,
    CaptureEmptyEnvironment{needed_captures: usize},
    NonReferenceCapture{capture: u16, capture_type: String},
    OutOfRangeTemplate{template: u16, template_count: usize},
    OutOfRangeLiteral{literal: u16, literal_count: usize},
    OutOfRangeDestinationCapture{capture: u16, capture_count: usize},
    OutOfRangeSourceCapture{capture: u16, capture_count: usize},
    OutOfRangeDestinationRegister{register: u8, register_count: usize},
    OutOfRangeSourceRegister{register: u8, register_count: usize},
    OutOfRangeSwapRegister{register: u8, register_count: usize},
}

pub type KutReturnType<'a> = Result<Option<KutValue<'a>>, KutError>;

impl<'template> Clone for KutValue<'template> {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
            Self::Undefined => Self::Undefined,
            Self::Number(num) => Self::Number(*num),
            Self::String(string) => Self::String(Rc::clone(string)),
            Self::List(list) => Self::List(Rc::clone(list)),
            Self::Func(func) => Self::Func(Rc::clone(func)),
            Self::Reference(r) => Self::Reference(Rc::clone(r)),
            Self::External(ext) => Self::External(Rc::clone(ext)),
        }
    }
}

impl<'template> KutValue<'template> {
    fn get_type_string(&self) -> String {
        match self {
            KutValue::Nil => "Nil",
            KutValue::Undefined => "Undefined",
            KutValue::Number(_) => "Number",
            KutValue::String(_) => "String",
            KutValue::List(_) => "List",
            KutValue::Func(_) => "Func",
            KutValue::Reference(_) => "Reference",
            KutValue::External(_) => "External",
        }.to_owned()
    }
}

impl From<KutError> for String {
    fn from(value: KutError) -> Self {
        match value {
            KutError::StackUnderflow => {
                format!("KutError::StackUnderflow: try to pop from empty call stack")
            },
            KutError::CaptureEmptyEnvironment { needed_captures } => {
                format!("KutError::CaptureEmptyEnvironment: {needed_captures} captures are needed")
            },
            KutError::NonReferenceCapture { capture, capture_type } => {
                format!("KutError::NonReferenceCapture: try to get capture {capture} when its type is {capture_type} instead of Reference")
            }
            KutError::OutOfRangeTemplate { template, template_count } => {
                format!("KutError::OutOfRangeTemplate: try to capture {template} when there are {template_count} templates")
            },
            KutError::OutOfRangeLiteral { literal, literal_count } => {
                format!("KutError::OutOfRangeLiteral: try to get literal {literal} when there are {literal_count} literals")
            },
            KutError::OutOfRangeDestinationCapture { capture, capture_count } => {
                format!("KutError::OutOfRangeDestinationCapture: try to set to capture {capture} when there are {capture_count} captures")
            },
            KutError::OutOfRangeSourceCapture { capture, capture_count } => {
                format!("KutError::OutOfRangeSourceCapture: try to get capture {capture} when there are {capture_count} captured captures")
            },
            KutError::OutOfRangeDestinationRegister { register, register_count } => {
                format!("KutError::OutOfRangeDestinationRegister: try to set to register {register} when there are {register_count} registers")
            },
            KutError::OutOfRangeSourceRegister { register, register_count } => {
                format!("KutError::OutOfRangeSourceRegister: try to get register {register} when there are {register_count} registers")
            },
            KutError::OutOfRangeSwapRegister { register, register_count } => {
                format!("KutError::OutOfRangeSwapRegister: try to get and set register {register} when there are {register_count} registers")
            }
        }
    }
}
