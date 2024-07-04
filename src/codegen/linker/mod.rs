use super::{assembler::Asm, elf::Object};

pub struct LinkOpts {
    pub base_addr: u64,
    pub alignment: u64,
}

pub fn link_elf(asm: &Asm, object: &mut Object, opts: &LinkOpts) {}
