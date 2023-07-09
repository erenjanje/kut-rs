use crate::value::*;
use crate::vm::*;

/// Instruction executor
impl KutInstruction {
    pub fn run<'closure, 'template>(&self, context: &mut KutFunction<'closure, 'template>, vm: &'closure KutVm<'template>) -> KutReturnType<'template> {
        match self {
            KutInstruction::NoOperation => KutInstruction::handle_no_operation(),
            KutInstruction::CallMethodR { ret_position, arg_count, subject } => KutInstruction::handle_call_method_r(context, *ret_position, *arg_count, *subject),
            KutInstruction::CallMethodS { arg_count, subject } => KutInstruction::handle_call_method_s(context, *arg_count, *subject),
            KutInstruction::CaptureFunc { reg, template } => KutInstruction::handle_capture_function(context, vm, *reg, *template),
            KutInstruction::GetCaptureR { reg, capture } => KutInstruction::handle_get_capture_r(context, *reg, *capture),
            KutInstruction::GetLiteralR { reg, literal } => KutInstruction::handle_get_literal(context, vm, *reg, *literal),
            KutInstruction::MovRegister { destination, source } => KutInstruction::handle_mov_register(context, *destination, *source),
            KutInstruction::PopCaptureS { capture } => KutInstruction::handle_pop_capture(context, *capture),
            KutInstruction::PushCapture { capture } => KutInstruction::handle_push_capture(context, *capture),
            KutInstruction::PushFuncStk { template } => KutInstruction::handle_push_template(context, vm, *template),
            KutInstruction::PushLiteral { literal } => KutInstruction::handle_push_literal(context, vm, *literal),
            KutInstruction::PushValue1R { val1 } => KutInstruction::handle_push_value_1(context, *val1),
            KutInstruction::PushValue2R { val1, val2 } => KutInstruction::handle_push_value_2(context, *val1, *val2),
            KutInstruction::PushValue3R { val1, val2, val3 } => KutInstruction::handle_push_value_3(context, *val1, *val2, *val3),
            KutInstruction::RetfMethodR { value } => KutInstruction::handle_ret_r(context, *value),
            KutInstruction::RetfMethodS => KutInstruction::handle_ret_s(context),
            KutInstruction::SetCaptureR { reg, capture } => KutInstruction::handle_set_capture_r(context, *reg, *capture),
            KutInstruction::SwapValuesR { reg1, reg2 } => KutInstruction::handle_swap_values(context, *reg1, *reg2),
        }
    }

}

/// Auxillary functions
impl<'closure, 'template> KutInstruction {
    fn get_register_value<'reg>(context: &'reg mut KutFunction<'closure, 'template>, reg: u8) -> Result<KutValue<'template>, KutError> {
        if let Some(source) = context.registers.get(reg as usize) {
            if let KutValue::Reference(r) = source {
                Ok((**r).borrow().clone())
            } else {
                Ok(source.clone())
            }
        } else {
            Err(KutError::OutOfRangeSourceRegister { register: reg, register_count: context.registers.len() })
        }
    }

    fn set_register_value<'reg>(context: &'reg mut KutFunction<'closure, 'template>, reg: u8, value: KutValue<'template>) -> Result<(),KutError> {
        if let Some(destination) = context.registers.get_mut(reg as usize) {
            if let KutValue::Reference(r) = destination {
                *(**r).borrow_mut() = value;
            } else {
                *destination = value;
            }
            Ok(())
        } else {
            Err(KutError::OutOfRangeDestinationRegister { register: reg, register_count: context.registers.len() })
        }
    }

    fn check_register<'reg>(context: &'reg mut KutFunction<'closure, 'template>, reg: u8, err: KutError) -> Result<(),KutError> {
        if context.registers.get(reg as usize).is_none() {
            Err(err)
        } else {
            Ok(())
        }
    }
}

/// Instruction handlers
impl<'closure, 'template> KutInstruction {
    fn handle_no_operation() -> KutReturnType<'template> {
        Ok(None)
    }
    
    fn handle_call_method_r(_context: &mut KutFunction<'closure, 'template>, _ret_position: u8, _arg_count: u8, _subject: u8) -> KutReturnType<'template> {
        unimplemented!()
    }

    fn handle_call_method_s(_context: &mut KutFunction<'closure, 'template>, _arg_count: u8, _subject: u8) -> KutReturnType<'template> {
        unimplemented!()
    }

    fn handle_capture_function(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, reg: u8, template: u16) -> KutReturnType<'template> {
        if let Some(tmplt) = vm.templates.get(template as usize) {
            let closure = KutValue::Func(Rc::new(tmplt.capture(Some(context))?));
            KutInstruction::set_register_value(context, reg, closure)?;
            Ok(None)
        } else {
            Err(KutError::OutOfRangeTemplate { template, template_count: vm.templates.len() })
        }
    }

    fn handle_get_capture_r(context: &mut KutFunction<'closure, 'template>, reg: u8, capture: u16) -> KutReturnType<'template> {
        if let Some(cap) = context.closure.captures.get(capture as usize) {
            if let KutValue::Reference(captured) = cap {
                let value = (*captured).borrow().clone();
                KutInstruction::set_register_value(context, reg, value)?;
                Ok(None)
            } else {
                Err(KutError::NonReferenceCapture { capture, capture_type: cap.get_type_string() })
            }
        } else {
            Err(KutError::OutOfRangeSourceCapture { capture, capture_count: context.closure.captures.len() })
        }
    }

    fn handle_get_literal(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, reg: u8, literal: u16) -> KutReturnType<'template> {
        if let Some(lit) = vm.literals.get(literal as usize) {
            let literal_cloned = lit.clone();
            KutInstruction::set_register_value(context, reg, literal_cloned)?;
            Ok(None)
        } else {
            Err(KutError::OutOfRangeLiteral { literal, literal_count: vm.literals.len() })
        }
    }

    fn handle_mov_register(context: &mut KutFunction<'closure, 'template>, destination: u8, source: u8) -> KutReturnType<'template> {
        if destination == source {
            Ok(None)
        } else {
            let value = KutInstruction::get_register_value(context, source)?;
            KutInstruction::set_register_value(context, destination, value)?;
            Ok(None)
        }
    }

    fn handle_pop_capture(context: &mut KutFunction<'closure, 'template>, capture: u16) -> KutReturnType<'template> {
        if let Some(source) = context.call_stack.pop() {
            let val = source.clone();
            if let Some(destination) = context.closure.captures.get(capture as usize) {
                if let KutValue::Reference(dest) = destination {
                    *(*dest).borrow_mut() = val;
                    Ok(None)
                } else {
                    Err(KutError::NonReferenceCapture { capture, capture_type: destination.get_type_string() })
                }
            } else {
                Err(KutError::OutOfRangeDestinationCapture { capture, capture_count: context.closure.captures.len() })
            }
        } else {
            Err(KutError::StackUnderflow)
        }
    }

    fn handle_push_capture(context: &mut KutFunction<'closure, 'template>, capture: u16) -> KutReturnType<'template> {
        if let Some(cap) = context.closure.captures.get(capture as usize) {
            if let KutValue::Reference(r) = cap {
                let value = (*r).borrow().clone();
                context.call_stack.push(value);
                Ok(None)
            } else {
                Err(KutError::NonReferenceCapture { capture, capture_type: cap.get_type_string() })
            }
        } else {
            Err(KutError::OutOfRangeSourceCapture { capture, capture_count: context.closure.captures.len() })
        }
    }

    fn handle_push_template(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, template: u16) -> KutReturnType<'template> {
        if let Some(tmplt) = vm.templates.get(template as usize) {
            let closure = KutValue::Func(Rc::new(tmplt.capture(Some(context))?));
            context.call_stack.push(closure);
            Ok(None)
        } else {
            Err(KutError::OutOfRangeTemplate { template, template_count: vm.templates.len() })
        }
    }

    fn handle_push_literal(context: &mut KutFunction<'closure, 'template>, vm: &'template KutVm<'template>, literal: u16) -> KutReturnType<'template> {
        if let Some(lit) = vm.literals.get(literal as usize) {
            let literal_cloned = lit.clone();
            context.call_stack.push(literal_cloned);
            Ok(None)
        } else {
            Err(KutError::OutOfRangeLiteral { literal, literal_count: vm.literals.len() })
        }
    }

    fn handle_push_value_1(context: &mut KutFunction<'closure, 'template>, val1: u8) -> KutReturnType<'template> {
        let value = KutInstruction::get_register_value(context, val1)?;
        context.call_stack.push(value);
        Ok(None)
    }

    fn handle_push_value_2(context: &mut KutFunction<'closure, 'template>, val1: u8, val2: u8) -> KutReturnType<'template> {
        let value = KutInstruction::get_register_value(context, val1)?;
        context.call_stack.push(value);
        let value = KutInstruction::get_register_value(context, val2)?;
        context.call_stack.push(value);
        Ok(None)
    }

    fn handle_push_value_3(context: &mut KutFunction<'closure, 'template>, val1: u8, val2: u8, val3: u8) -> KutReturnType<'template> {
        let value = KutInstruction::get_register_value(context, val1)?;
        context.call_stack.push(value);
        let value = KutInstruction::get_register_value(context, val2)?;
        context.call_stack.push(value);
        let value = KutInstruction::get_register_value(context, val3)?;
        context.call_stack.push(value);
        Ok(None)
    }

    fn handle_ret_r(context: &mut KutFunction<'closure, 'template>, value: u8) -> KutReturnType<'template> {
        let val = KutInstruction::get_register_value(context, value)?;
        Ok(Some(val))
    }

    fn handle_ret_s(context: &mut KutFunction<'closure, 'template>) -> KutReturnType<'template> {
        if let Some(val) = context.call_stack.pop() {
            Ok(Some(val))
        } else {
            Err(KutError::StackUnderflow)
        }
    }

    fn handle_set_capture_r(context: &mut KutFunction<'closure, 'template>, reg: u8, capture: u16) -> KutReturnType<'template> {
        let value = KutInstruction::get_register_value(context, reg)?;
        if let Some(cap) = context.closure.captures.get(capture as usize) {
            if let KutValue::Reference(captured) = cap {
                *(*captured).borrow_mut() = value;
                Ok(None)
            } else {
                Err(KutError::NonReferenceCapture { capture, capture_type: cap.get_type_string() })
            }
        } else {
            Err(KutError::OutOfRangeDestinationCapture { capture, capture_count: context.closure.captures.len() })
        }
    }

    fn handle_swap_values(context: &mut KutFunction<'closure, 'template>, reg1: u8, reg2: u8) -> KutReturnType<'template> {
        KutInstruction::check_register(context, reg1, KutError::OutOfRangeSwapRegister { register: reg1, register_count: context.registers.len() })?;
        KutInstruction::check_register(context, reg2, KutError::OutOfRangeSwapRegister { register: reg2, register_count: context.registers.len() })?;
        context.registers.swap(reg1 as usize, reg2 as usize);
        Ok(None)
    }
}
