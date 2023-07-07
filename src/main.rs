pub mod value;
pub mod vm;
use value::*;
use vm::*;
use std::rc::Rc;
use std::collections::HashMap;

fn main() {
    let inner_instructions = vec![];
    let instruction_list = vec![
        KutInstruction::new_imm(KutImmediateInstruction::GetLiteralR, 2, 0),
        KutInstruction::new_imm(KutImmediateInstruction::GetLiteralR, 3, 2),
    ];
    let mut dict: HashMap<KutValue, KutValue> = HashMap::new();
    dict.insert(KutValue::String(Rc::new("aa".to_owned())));
    let literals = Box::new(vec![
        KutValue::Number(21.0),
        KutValue::String(Rc::new("zort".to_owned())),
        KutValue::List(Rc::new(vec![KutValue::Number(5.0), KutValue::Number(10.0)])),
        KutValue::Dict(Rc::new(HashMap::new()))
    ]);
    let mut vm = KutVM::new(
        KutFunc::new(instruction_list, 4, 0, &literals), vec![
        KutFunc::new(inner_instructions, 4, 0, &literals),
    ]);
    vm.run().unwrap();
}
