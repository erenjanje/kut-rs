use crate::value::*;
use crate::vm::*;
// use crate::value::instruction::*;

impl<'closure, 'template> KutFunction<'closure, 'template> {
    pub fn run(&mut self, vm: &'closure KutVm<'template>) -> KutReturnType<'template> {
        for instruction in self.closure.template.instructions.iter() {
            instruction.run(self, vm)?;
        }
        Ok(None)
    }
}
