pub mod value;
pub mod vm;
use value::*;
use vm::*;
use std::rc::Rc;

fn main() -> Result<(), String> {
    let instructions = vec![
        KutInstruction::GetLiteralR { reg: 0, literal: 0 },
        KutInstruction::GetLiteralR { reg: 1, literal: 1 },
        KutInstruction::GetLiteralR { reg: 2, literal: 1 },
    ];
    let literals = vec![
        KutValue::Number(5.0),
        KutValue::String(Rc::new(String::from("zort"))),
    ];
    let templates = vec![KutFunctionTemplate::new(instructions, vec![], 4)];
    let vm = KutVm::new(literals, templates);
    let capture = vm.templates[0].capture(None)?;
    let mut func = capture.start();
    func.run(&vm)?;
    dbg!(&func);
    if let KutValue::String(s) = &vm.literals[1] {
        println!("{}", Rc::strong_count(s));
    }
    return Ok(());
}
