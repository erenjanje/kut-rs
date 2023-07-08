use crate::value::*;
use crate::vm::*;

impl KutInstruction {
    pub fn no_operation() -> KutInstruction {
        KutInstruction::NoOperation
    }
    pub fn mov_register(destination: u8, source: u8) -> KutInstruction {
        KutInstruction::MovRegister { destination, source }
    }
    pub fn call_method_r(ret_position: u8, arg_count: u8, subject: u8) -> KutInstruction {
        KutInstruction::CallMethodR { ret_position, arg_count, subject }
    }
    pub fn call_method_s(arg_count: u8, subject: u8) -> KutInstruction {
        KutInstruction::CallMethodS { arg_count, subject }
    }
    pub fn ret_method_r(value: u8) -> KutInstruction {
        KutInstruction::RetfMethodR { value }
    }
    pub fn ret_method_s() -> KutInstruction {
        KutInstruction::RetfMethodS
    }
    pub fn push_value_1(val1: u8) -> KutInstruction {
        KutInstruction::PushValue1R { val1 }
    }
    pub fn push_value_2(val1: u8, val2: u8) -> KutInstruction {
        KutInstruction::PushValue2R { val1, val2 }
    }
    pub fn push_value_3(val1: u8, val2: u8, val3: u8) -> KutInstruction {
        KutInstruction::PushValue3R { val1, val2, val3 }
    }
    pub fn swap_values(reg1: u8, reg2: u8) -> KutInstruction {
        KutInstruction::SwapValuesR { reg1, reg2 }
    }
    pub fn get_literal(reg: u8, literal: u16) -> KutInstruction {
        KutInstruction::GetLiteralR { reg, literal }
    }
    pub fn get_capture(reg: u8, capture: u16) -> KutInstruction {
        KutInstruction::GetCaptureR { reg, capture }
    }
    pub fn set_capture(reg: u8, capture: u16) -> KutInstruction {
        KutInstruction::SetCaptureR { reg, capture }
    }
    pub fn capture_func(reg: u8, template: u16) -> KutInstruction {
        KutInstruction::CaptureFunc { reg, template }
    }
    pub fn push_literal(literal: u16) -> KutInstruction {
        KutInstruction::PushLiteral { literal }
    }
    pub fn push_capture(capture: u16) -> KutInstruction {
        KutInstruction::PushCapture { capture }
    }
    pub fn push_func_stack(template: u16) -> KutInstruction {
        KutInstruction::PushFuncStk { template }
    }
    pub fn pop_capture(capture: u16) -> KutInstruction {
        KutInstruction::PopCaptureS { capture }
    }
}

impl KutInstruction {
    pub fn run<'closure, 'template>(&self, context: &mut KutFunction<'closure, 'template>, vm: &'closure KutVm<'template>) -> KutReturnType<'template> {
        match self {
            KutInstruction::NoOperation => KutInstruction::handle_no_operation(),
            KutInstruction::CallMethodR { ret_position, arg_count, subject } => KutInstruction::handle_call_method_r(context, *ret_position, *arg_count, *subject),
            KutInstruction::CallMethodS { arg_count, subject } => KutInstruction::handle_call_method_s(context, *arg_count, *subject),
            KutInstruction::CaptureFunc { reg, template } => KutInstruction::handle_capture_function(context, vm, *reg, *template),
            KutInstruction::GetCaptureR { reg, capture } => unimplemented!(),
            KutInstruction::GetLiteralR { reg, literal } => KutInstruction::handle_get_literal_r(context, vm, *reg, *literal),
            KutInstruction::MovRegister { destination, source } => unimplemented!(),
            KutInstruction::PopCaptureS { capture } => unimplemented!(),
            KutInstruction::PushCapture { capture } => unimplemented!(),
            KutInstruction::PushFuncStk { template } => unimplemented!(),
            KutInstruction::PushLiteral { literal } => unimplemented!(),
            KutInstruction::PushValue1R { val1 } => unimplemented!(),
            KutInstruction::PushValue2R { val1, val2 } => unimplemented!(),
            KutInstruction::PushValue3R { val1, val2, val3 } => unimplemented!(),
            KutInstruction::RetfMethodR { value } => unimplemented!(),
            KutInstruction::RetfMethodS => unimplemented!(),
            KutInstruction::SetCaptureR { reg, capture } => unimplemented!(),
            KutInstruction::SwapValuesR { reg1, reg2 } => unimplemented!(),
        }
    }

}

impl<'closure, 'template> KutInstruction {
    fn handle_no_operation() -> KutReturnType<'template> {
        Ok(None)
    }
    
    fn handle_call_method_r(context: &mut KutFunction<'closure, 'template>, ret_position: u8, arg_count: u8, subject: u8) -> KutReturnType<'template> {
        unimplemented!()
    }

    fn handle_call_method_s(context: &mut KutFunction<'closure, 'template>, arg_count: u8, subject: u8) -> KutReturnType<'template> {
        unimplemented!()
    }

    fn handle_capture_function(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, reg: u8, template: u16) -> KutReturnType<'template> {
        if let Some(tmplt) = vm.templates.get(template as usize) {
            let closure = KutValue::Func(Rc::new(tmplt.capture(Some(context))?));
            if let Some(register) = context.registers.get_mut(reg as usize) {
                *register = closure;
                Ok(None)
            } else {
                Err(KutErrorType::TemplateOutOfRangeRegister { register: reg, register_count: context.registers.len() })
            }
        } else {
            Err(KutErrorType::TemplateOutOfRange { template, template_count: vm.templates.len() })
        }
    }

    fn handle_get_literal_r(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, reg: u8, literal: u16) -> KutReturnType<'template> {
        if let Some(lit) = vm.literals.get(literal as usize) {
            let literal_cloned = lit.clone();
            if let Some(register) = context.registers.get_mut(reg as usize) {
                *register = literal_cloned;
                Ok(None)
            } else {
                Err(KutErrorType::LiteralOutOfRangeRegister { register: reg, register_count: context.registers.len() })
            }
        } else {
            Err(KutErrorType::LiteralOutOfRange { literal, literal_count: vm.literals.len() })
        }
    }
}
