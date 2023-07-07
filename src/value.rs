
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::ffi::{CStr, c_void};

pub struct KutObject {
    dispatch: unsafe extern "C" fn(*mut KutValue, CStr, *mut KutValue, *mut c_void),
    data: *mut c_void,
}

// #[derive(Eq, Hash, PartialEq)]
pub enum KutValue<'a> {
    Nil,
    Undefined,
    Number(f64),
    String(Rc<String>),
    List(Rc<Vec<KutValue<'a>>>),
    Dict(Rc<HashMap<KutValue<'a>,KutValue<'a>>>),
    Func(Rc<KutFunc<'a>>),
    External(Rc<KutObject>),
}

impl<'a> Drop for KutValue<'a> {
    fn drop(&mut self) {
        match self {
            KutValue::Nil | KutValue::Undefined | KutValue::Number(_) => (),
            KutValue::String(val) => drop(val),
            KutValue::List(val) => drop(val),
            KutValue::Dict(val) => drop(val),
            KutValue::Func(val) => drop(val),
            KutValue::External(val) => drop(val),
        }
    }
}

impl<'a> Clone for KutValue<'a> {
    fn clone(&self) -> Self {
        match self {
            KutValue::Nil => KutValue::Nil,
            KutValue::Undefined => KutValue::Undefined,
            KutValue::Number(val) => KutValue::Number(f64::clone(val)),
            KutValue::String(val) => KutValue::String(Rc::clone(val)),
            KutValue::List(val) => KutValue::List(Rc::clone(val)),
            KutValue::Dict(val) => KutValue::Dict(Rc::clone(val)),
            KutValue::Func(val) => KutValue::Func(Rc::clone(val)),
            KutValue::External(val) => KutValue::External(Rc::clone(val)),
        }
    }
}

impl<'a> Display for KutValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Undefined => write!(f, "undefined"),
            Self::Number(num) => write!(f, "{}", num),
            Self::String(str) => write!(f, "\"{}\"", str),
            Self::List(lst) => {
                write!(f, "[")?;
                write!(f, "{}", lst[0])?;
                for elem in lst.iter().skip(1) {
                    write!(f, " {}", elem)?;
                }
                write!(f, "]")
            },
            Self::Dict(dict) => {
                write!(f, "{{")?;
                for (key, val) in dict.iter() {
                    write!(f, "{}:{} ", key, val)?;
                }
                write!(f, "}}")
            },
            Self::Func(_) => write!(f, "func"),
            Self::External(_) => write!(f, "external"),
        }
    }
}

pub enum KutEmptyInstruction {
    NoOperation
}

impl Display for KutEmptyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoOperation => write!(f, "NoOperation"),
        }
    }
}

pub enum KutRegisterInstruction {
    MovRegister,
    
    CallMethodR,
    RetfMethodR,
    
    PushValue1R,
    PushValue2R,
    PushValue3R,

    SwapValuesR,
}

impl Display for KutRegisterInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MovRegister => write!(f, "MovRegister"),
            Self::CallMethodR => write!(f, "CallMethodR"),
            Self::RetfMethodR => write!(f, "RetfMethodR"),
            Self::PushValue1R => write!(f, "PushValue1R"),
            Self::PushValue2R => write!(f, "PushValue2R"),
            Self::PushValue3R => write!(f, "PushValue3R"),
            Self::SwapValuesR => write!(f, "SwapValuesR"),
        }
    }
}

pub enum KutImmediateInstruction {
    GetLiteralR,
    GetClosureR,
    SetClosureR,
    GetFunction,
}

impl Display for KutImmediateInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetLiteralR => write!(f, "GetLiteralR"),
            Self::GetClosureR => write!(f, "GetClosureR"),
            Self::SetClosureR => write!(f, "SetClosureR"),
            Self::GetFunction => write!(f, "GetFunction"),
        }
    }
}

pub enum KutInstruction {
    EmptyInstruction(KutEmptyInstruction),
    RegisterInstruction(KutRegisterInstruction, u8, u8, u8),
    ImmediateInstruction(KutImmediateInstruction, u8, u16),
}

impl Display for KutInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInstruction(instruction) => write!(f, "{}", instruction),
            Self::RegisterInstruction(instruction, reg1, reg2, reg3) => write!(f, "{}\t{},{},{}", instruction, reg1, reg2, reg3),
            Self::ImmediateInstruction(instruction, reg, imm) => write!(f, "{}\t{},{}", instruction, reg, imm),
        }
    }
}

impl KutInstruction {
    pub fn new_empty(instruction: KutEmptyInstruction) -> KutInstruction {
        KutInstruction::EmptyInstruction(instruction)
    }
    pub fn new_reg(instruction: KutRegisterInstruction, reg1: u8, reg2: u8, reg3: u8) -> KutInstruction {
        KutInstruction::RegisterInstruction(instruction, reg1, reg2, reg3)
    }
    pub fn new_imm(instruction: KutImmediateInstruction, reg: u8, imm: u16) -> KutInstruction {
        KutInstruction::ImmediateInstruction(instruction, reg, imm)
    }
    pub fn new(instructions: &[u32]) -> Result<Vec<KutInstruction>,String> {
        instructions.iter().map(|instruction| {
            let byte_arr = instruction.to_be_bytes();
            match byte_arr[0] {
                0 =>  Ok(KutInstruction::EmptyInstruction(KutEmptyInstruction::NoOperation)),
                1 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::MovRegister, byte_arr[1], byte_arr[2], byte_arr[3])),
                2 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::CallMethodR, byte_arr[1], byte_arr[2], byte_arr[3])),
                3 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::RetfMethodR, byte_arr[1], byte_arr[2], byte_arr[3])),
                4 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::PushValue1R, byte_arr[1], byte_arr[2], byte_arr[3])),
                5 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::PushValue2R, byte_arr[1], byte_arr[2], byte_arr[3])),
                6 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::PushValue3R, byte_arr[1], byte_arr[2], byte_arr[3])),
                7 =>  Ok(KutInstruction::RegisterInstruction(KutRegisterInstruction::SwapValuesR, byte_arr[1], byte_arr[2], byte_arr[3])),
                8 =>  Ok(KutInstruction::ImmediateInstruction(KutImmediateInstruction::GetLiteralR, byte_arr[1], ((byte_arr[2] as u16) << 8) + byte_arr[3] as u16)),
                9 =>  Ok(KutInstruction::ImmediateInstruction(KutImmediateInstruction::GetClosureR, byte_arr[1], ((byte_arr[2] as u16) << 8) + byte_arr[3] as u16)),
                10 => Ok(KutInstruction::ImmediateInstruction(KutImmediateInstruction::SetClosureR, byte_arr[1], ((byte_arr[2] as u16) << 8) + byte_arr[3] as u16)),
                11 => Ok(KutInstruction::ImmediateInstruction(KutImmediateInstruction::GetFunction, byte_arr[1], ((byte_arr[2] as u16) << 8) + byte_arr[3] as u16)),
                _ => Err(format!("undefined instruction with opcode {}", byte_arr[0]))
            }
        }).collect()
    }
}

pub struct KutFuncData<'a> {
    registers: Vec<KutValue<'a>>,
    closures: Vec<Box<KutValue<'a>>>,
    call_stack: Vec<KutValue<'a>>,
    literals: &'a Vec<KutValue<'a>>,
}

pub struct KutFuncCode {
    instructions: Vec<KutInstruction>,
}

pub struct KutFunc<'a> {
    data: KutFuncData<'a>,
    code: KutFuncCode,
}

type InstructionHandlerReturn = Result<(), String>;


impl<'a> KutFuncData<'a> {
    fn no_operation(&mut self) -> InstructionHandlerReturn {
        Ok(())
    }
    
    fn mov_register(&mut self, reg1: u8, reg2: u8, _: u8) -> InstructionHandlerReturn {
        let reg1 = reg1 as usize;
        let reg2 = reg2 as usize;
        if reg1 >= self.registers.len() {
            return Err(format!("first register index {} should be smaller than register count {}", reg1, self.registers.len()));
        } else if reg2 >= self.registers.len() {
            return Err(format!("second register index {} should be smaller than register count {}", reg2, self.registers.len()));
        }
        if reg1 == reg2 {
            return Ok(());
        }

        // drop(&mut self.registers[reg1]);
        self.registers[reg1] = self.registers[reg2].clone();
        Ok(())
    }
    
    fn call_method(&mut self, _: u8, _: u8, _: u8) -> InstructionHandlerReturn {
        
        Ok(())
    }
    
    fn ret_method(&mut self, _: u8, _: u8, _: u8) -> InstructionHandlerReturn {
        //TODO: Implement me
        Ok(())
    }
    
    fn push_value1(&mut self, val: u8, _: u8, _: u8) -> InstructionHandlerReturn {
        let val = val as usize;
        if val >= self.registers.len() {
            return Err(format!("register index {} should be smaller than register count {}", val, self.registers.len()));
        }
        self.call_stack.push(self.registers[val as usize].clone());
        Ok(())
    }
    
    fn push_value2(&mut self, val1: u8, val2: u8, _: u8) -> InstructionHandlerReturn {
        if val1 as usize >= self.registers.len() {
            return Err(format!("first register index {} should be smaller than register count {}", val1, self.registers.len()));
        } else if val2 as usize >= self.registers.len() {
            return Err(format!("second register index {} should be smaller than register count {}", val2, self.registers.len()));
        }
        self.call_stack.push(self.registers[val1 as usize].clone());
        self.call_stack.push(self.registers[val2 as usize].clone());
        Ok(())
    }
    
    fn push_value3(&mut self, val1: u8, val2: u8, val3: u8) -> InstructionHandlerReturn {
        if val1 as usize >= self.registers.len() {
            return Err(format!("first register index {} should be smaller than register count {}", val1, self.registers.len()));
        } else if val2 as usize >= self.registers.len() {
            return Err(format!("second register index {} should be smaller than register count {}", val2, self.registers.len()));
        } else if val3 as usize >= self.registers.len() {
            return Err(format!("second register index {} should be smaller than register count {}", val3, self.registers.len()));
        }
        self.call_stack.push(self.registers[val1 as usize].clone());
        self.call_stack.push(self.registers[val2 as usize].clone());
        self.call_stack.push(self.registers[val3 as usize].clone());
        Ok(())
    }
    
    fn swap_values(&mut self, val1: u8, val2: u8, _: u8) -> InstructionHandlerReturn {
        if val1 as usize >= self.registers.len() {
            return Err(format!("first register index {} should be smaller than register count {}", val1, self.registers.len()));
        } else if val2 as usize >= self.registers.len() {
            return Err(format!("second register index {} should be smaller than register count {}", val2, self.registers.len()));
        }
        
        self.registers.swap(val1 as usize, val2 as usize);
        Ok(())
    }
    
    fn get_literal(&mut self, val: u8, lit: u16) -> InstructionHandlerReturn {
        let val = val as usize;
        let lit = lit as usize;
        if val >= self.registers.len() {
            return Err(format!("register index {} should be smaller than register count {}", val, self.registers.len()));
        } else if lit >= self.literals.len() {
            return Err(format!("literal index {} should be smaller than register count {}", lit, self.literals.len()));
        }
    
        drop(&mut self.registers[val]);
        self.registers[val] = self.literals[lit].clone();
        Ok(())
    }
    
    fn get_closure(&mut self, val: u8, closure: u16) -> InstructionHandlerReturn {
        let val = val as usize;
        let closure = closure as usize;
        if val >= self.registers.len() {
            return Err(format!("register index {} should be smaller than register count {}", val, self.registers.len()));
        } else if closure >= self.literals.len() {
            return Err(format!("literal index {} should be smaller than register count {}", closure, self.literals.len()));
        }
    
        drop(&mut self.registers[val]);
        self.registers[val] = (*self.closures[closure]).clone();
        Ok(())
    }
    
    fn set_closure(&mut self, val: u8, closure: u16) -> InstructionHandlerReturn {
        let val = val as usize;
        let closure = closure as usize;
        if val >= self.registers.len() {
            return Err(format!("register index {} should be smaller than register count {}", val, self.registers.len()));
        } else if closure >= self.literals.len() {
            return Err(format!("literal index {} should be smaller than register count {}", closure, self.literals.len()));
        }
    
        drop(&mut (*self.closures[closure]));
        *self.closures[closure] = self.registers[val].clone();
        Ok(())
    }

    fn get_function(&mut self, val: u8, function: u16, funcs: &mut Vec<KutFunc<'a>>) -> InstructionHandlerReturn {

        Ok(())
    }

    fn handle_empty_instruction(&mut self, instruction: &KutEmptyInstruction, _: &mut Vec<KutFunc<'a>>) -> InstructionHandlerReturn {
        match instruction {
            KutEmptyInstruction::NoOperation => self.no_operation()
        }
    }
    
    fn handle_register_instruction(&mut self, instruction: &KutRegisterInstruction, funcs: &mut Vec<KutFunc<'a>>, reg1: u8, reg2: u8, reg3: u8) -> InstructionHandlerReturn {
        match instruction {
            KutRegisterInstruction::MovRegister => self.mov_register(reg1, reg2, reg3),
            KutRegisterInstruction::CallMethodR => self.call_method(reg1, reg2, reg3),
            KutRegisterInstruction::RetfMethodR => self.ret_method(reg1, reg2, reg3),
            KutRegisterInstruction::PushValue1R => self.push_value1(reg1, reg2, reg3),
            KutRegisterInstruction::PushValue2R => self.push_value2(reg1, reg2, reg3),
            KutRegisterInstruction::PushValue3R => self.push_value3(reg1, reg2, reg3),
            KutRegisterInstruction::SwapValuesR => self.swap_values(reg1, reg2, reg3),
        }
    }
    
    fn handle_immediate_instruction(&mut self, instruction: &KutImmediateInstruction, funcs: &mut Vec<KutFunc<'a>>, reg: u8, imm: u16) -> InstructionHandlerReturn {
        match instruction {
            KutImmediateInstruction::GetLiteralR => self.get_literal(reg, imm),
            KutImmediateInstruction::GetClosureR => self.get_closure(reg, imm),
            KutImmediateInstruction::SetClosureR => self.set_closure(reg, imm),
            KutImmediateInstruction::GetFunction => self.get_function(reg, imm, funcs),
        }
    }
}


impl<'a> KutFunc<'a> {
    pub fn new(instruction_list: Vec<KutInstruction>, register_count: usize, closure_count: usize, literals: &'a Vec<KutValue>) -> KutFunc<'a> {
        KutFunc {
            data: KutFuncData {
                registers: vec![KutValue::Nil; register_count],
                closures: Vec::with_capacity(closure_count),
                call_stack: Vec::new(),
                literals
            },
            code: KutFuncCode {
                instructions: instruction_list
            }
        }
    }

    pub fn run(&mut self, funcs: &mut Vec<KutFunc<'a>>) -> Result<(), String> {
        for instruction in self.code.instructions.iter() {
            for reg in self.data.registers.iter() {
                print!("{} ", reg);
            }
            match instruction {
                KutInstruction::EmptyInstruction(instruction) => self.data.handle_empty_instruction(&instruction, funcs)?,
                KutInstruction::RegisterInstruction(instruction, reg1, reg2, reg3) => self.data.handle_register_instruction(&instruction, funcs, reg1.clone(), reg2.clone(), reg3.clone())?,
                KutInstruction::ImmediateInstruction(instruction, reg, imm) => self.data.handle_immediate_instruction(&instruction, funcs, reg.clone(), imm.clone())?,
            }
            print!(" | ");
            for reg in self.data.registers.iter() {
                print!("{} ", reg);
            }
            println!("");
        }
        Ok(())
    }
}

impl<'a> Display for KutFunc<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = write!(f, "Func of {} registers and {} closures\n", self.data.registers.len(), self.data.closures.len());
        if result.is_err() {
            return result;
        }
        for instruction in self.code.instructions.iter() {
            result = write!(f, "   {}\n", instruction);
            if result.is_err() {
                return result;
            }
        }
        result
    }
}
