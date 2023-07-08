use crate::value::*;

pub struct KutVm<'template> {
    pub literals: Vec<KutValue<'template>>,
    pub templates: Vec<KutFunctionTemplate>,
}

impl<'template> KutVm<'template> {
    pub fn new(literals: Vec<KutValue<'template>>, templates: Vec<KutFunctionTemplate>) -> KutVm<'template> {
        KutVm { literals, templates }
    }
}
