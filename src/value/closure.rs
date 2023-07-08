use crate::value::*;

impl<'template> KutClosure<'template> {
    pub fn start<'closure>(&'closure self) -> KutFunction<'closure, 'template> {
        KutFunction { closure: self, registers: vec![KutValue::Nil; self.template.register_count as usize], call_stack: vec![] }
    }
}
