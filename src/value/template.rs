use crate::value::*;

impl KutFunctionTemplate {
    pub fn new(instructions: Vec<KutInstruction>, capture_infos: Vec<KutCaptureInfo>, register_count: u8) -> KutFunctionTemplate {
        KutFunctionTemplate { instructions, capture_infos, register_count }
    }
    pub fn capture<'closure,'template>(&'template self, _env: Option<&mut KutFunction<'closure,'template>>) -> Result<KutClosure<'template>, KutError> {
        if let Some(env) = _env  {
            let mut captures: Vec<KutValue> = Vec::with_capacity(self.capture_infos.len());
            for capture_info in self.capture_infos.iter() {
                match capture_info {
                    KutCaptureInfo::Register(reg) => {
                        if let Some(val) = env.registers.get_mut(*reg as usize) {
                            let inner_val = (*val).clone();
                            *val = KutValue::Reference(Rc::new(RefCell::new(inner_val)));
                            captures.push((*val).clone());
                        } else {
                            return Err(KutError::OutOfRangeDestinationRegister { register: *reg, register_count: env.registers.len() });
                        }
                    },
                    KutCaptureInfo::Capture(cap) => {
                        if let Some(val) = env.closure.captures.get(*cap as usize) {
                            captures.push((*val).clone());
                        } else {
                            return Err(KutError::OutOfRangeSourceCapture { capture: *cap, capture_count: env.closure.captures.len() });
                        }
                    }
                }
            }
            Ok(KutClosure { template: self, captures: captures })
        } else if self.capture_infos.len() == 0 {
            Ok(KutClosure { template: self, captures: vec![] })
        } else {
            Err(KutError::CaptureEmptyEnvironment { needed_captures: self.capture_infos.len() })
        }
    }
}
