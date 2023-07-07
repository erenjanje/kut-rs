use crate::value::*;
pub struct KutVM<'a> {
    main: KutFunc<'a>,
    funcs: Vec<KutFunc<'a>>,
}

impl<'a> KutVM<'a> {
    pub fn new(main: KutFunc<'a>, funcs: Vec<KutFunc<'a>>) -> KutVM<'a> {
        KutVM{
            main,
            funcs
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.main.run(&mut self.funcs)
    }
}