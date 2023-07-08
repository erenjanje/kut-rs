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
    pub captures: Vec<RefCell<KutValue<'template>>>,
}

#[derive(Debug)]
pub struct KutFunction<'closure,'template> where 'closure: 'template {
    pub closure: &'closure KutClosure<'template>,
    pub registers: Vec<KutValue<'template>>,
    pub call_stack: Vec<KutValue<'template>>,
}

#[derive(Debug)]
pub enum KutErrorType {
    CaptureEmptyEnvironment{needed_captures: usize},
    CaptureOutOfRangeRegister{register: u8, register_count: usize},
    CaptureOutOfRangeCapture{capture: u16, capture_count: usize},
    TemplateOutOfRange{template: u16, template_count: usize},
    TemplateOutOfRangeRegister{register: u8, register_count: usize},
    LiteralOutOfRange{literal: u16, literal_count: usize},
    LiteralOutOfRangeRegister{register: u8, register_count: usize},
}

pub type KutReturnType<'a> = Result<Option<KutValue<'a>>, KutErrorType>;

impl<'template> Clone for KutValue<'template> {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
            Self::Undefined => Self::Undefined,
            Self::Number(num) => Self::Number(*num),
            Self::String(string) => Self::String(Rc::clone(string)),
            Self::List(list) => Self::List(Rc::clone(list)),
            Self::Func(func) => Self::Func(Rc::clone(func)),
            Self::External(ext) => Self::External(Rc::clone(ext)),
        }
    }
}

impl From<KutErrorType> for String {
    fn from(value: KutErrorType) -> Self {
        match value {
            KutErrorType::CaptureEmptyEnvironment { needed_captures } => {
                format!("Error CaptureEmptyEnvironment: {needed_captures} captures are needed")
            },
            KutErrorType::CaptureOutOfRangeCapture { capture, capture_count } => {
                format!("Error CaptureOutOfRangeCapture: try to capture {capture} when there are {capture_count} captured elements")
            },
            KutErrorType::CaptureOutOfRangeRegister { register, register_count } => {
                format!("Error CaptureOutOfRangeRegister: try to capture {register} when there are {register_count} registers")
            },
            KutErrorType::TemplateOutOfRange { template, template_count } => {
                format!("Error TemplateOutOfRange: try to capture {template} when there are {template_count} templates")
            },
            KutErrorType::TemplateOutOfRangeRegister { register, register_count } => {
                format!("Error TemplateOutOfRangeRegister: try to put created template into register {register} when there are {register_count} registers")
            },
            KutErrorType::LiteralOutOfRange { literal, literal_count } => {
                format!("Error LiteralOutOfRange: try to get literal {literal} when there are {literal_count} literals")
            },
            KutErrorType::LiteralOutOfRangeRegister { register, register_count } => {
                format!("Error LiteralOutOfRangeRegister: try to set to register {register} when there are {register_count} registers")
            },
        }
    }
}
