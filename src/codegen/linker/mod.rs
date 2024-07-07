use crate::codegen::{assembler::Asm, elf::Elf};

pub struct LinkOpts {
    pub base_addr: u64,
    pub alignment: u64,
}

pub fn link_elf(asm: &mut Asm, object: &mut Elf, opts: &LinkOpts) {
    for symbol in &asm.symbols {
        let address = opts.base_addr
            + match symbol.section.as_str() {
                "start" => 1 * opts.alignment + object.text_offset as u64 + symbol.offset,
                "data" => {
                    2 * opts.alignment + object.data_offset.unwrap() as u64 + symbol.offset
                        - asm.text_size as u64
                }
                _ => 0,
            };
        let address_bytes = &address.to_le_bytes()[..4];
        for offset in &symbol.position_offsets {
            let start_byte = object.text_offset + offset;
            let _ = &object.buf[start_byte + 1..start_byte + 5].copy_from_slice(address_bytes);
        }
    }
}
