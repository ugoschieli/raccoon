#[cfg(test)]
mod tests;

use crate::codegen::assembler::Asm;
use crate::codegen::linker::LinkOpts;
use object::write::elf::{FileHeader, ProgramHeader, Writer};
use object::Endianness;

#[derive(Debug)]
pub struct RaccoonFileHeader(FileHeader);
impl PartialEq for RaccoonFileHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0.os_abi == other.0.os_abi
            && self.0.abi_version == other.0.abi_version
            && self.0.e_type == other.0.e_type
            && self.0.e_machine == other.0.e_machine
            && self.0.e_entry == other.0.e_entry
            && self.0.e_flags == other.0.e_flags
    }
}
impl Eq for RaccoonFileHeader {}

#[derive(Debug)]
pub struct RaccoonProgramHeader(ProgramHeader);
impl PartialEq for RaccoonProgramHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0.p_type == other.0.p_type
            && self.0.p_flags == other.0.p_flags
            && self.0.p_offset == other.0.p_offset
            && self.0.p_vaddr == other.0.p_vaddr
            && self.0.p_paddr == other.0.p_paddr
            && self.0.p_filesz == other.0.p_filesz
            && self.0.p_memsz == other.0.p_memsz
            && self.0.p_align == other.0.p_align
    }
}
impl Eq for RaccoonProgramHeader {}

#[derive(Debug, Eq)]
pub struct Object {
    pub buf: Vec<u8>,
    pub file_header: RaccoonFileHeader,
    pub program_headers: Vec<RaccoonProgramHeader>,
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.file_header == other.file_header && self.program_headers == self.program_headers
    }
}

pub fn create_elf(asm: &Asm, link_opts: &LinkOpts) -> Object {
    let mut buf: Vec<u8> = Vec::new();
    let mut program_headers: Vec<RaccoonProgramHeader> = Vec::new();
    let mut w = Writer::new(Endianness::Little, true, &mut buf);

    w.reserve_file_header();
    let nb_ph = match asm.data_size {
        Some(_) => 2,
        None => 1,
    };
    w.reserve_program_headers(nb_ph);

    let text_offset = w.reserve(asm.text_size, 0);
    let data_offset = match asm.data_size {
        Some(size) => Some(w.reserve(size, 0)),
        None => None,
    };

    // ELF File Header
    let file_header = FileHeader {
        os_abi: 0,                                                                           // SYSV
        abi_version: 0,  // Ignored
        e_type: 2,       // Executable
        e_machine: 0x3E, // x86_64
        e_entry: link_opts.base_addr + link_opts.alignment + text_offset as u64 + asm.start, // Entrypoint (virtual address of _start)
        e_flags: 0, // Ignored
    };
    w.write_file_header(&file_header).expect("write header");

    // ELF Program Header (.text)
    let text_ph = ProgramHeader {
        p_type: 1,                                                               // Loadable segment
        p_offset: text_offset as u64, // Start position of code in bytes
        p_vaddr: link_opts.base_addr + link_opts.alignment + text_offset as u64, // Address in memory of _start
        p_paddr: 0,                                                              // Not relevant
        p_filesz: asm.text_size as u64,      // Section size in bytes
        p_memsz: asm.text_size as u64,       // Same
        p_flags: 5,                          // Executable (1) + Readable (4)
        p_align: link_opts.alignment as u64, // Alignment (4096 bytes)
    };
    w.write_program_header(&text_ph);
    program_headers.push(RaccoonProgramHeader(text_ph));

    match asm.data_size {
        Some(_) => {
            // ELF Program Header (.data)
            let data_ph = ProgramHeader {
                p_type: 1,                             // Loadable segment
                p_offset: data_offset.unwrap() as u64, // Start position of code in bytes
                p_vaddr: link_opts.base_addr
                    + 2 * link_opts.alignment
                    + data_offset.unwrap() as u64, // Address in memory of _start
                p_paddr: 0,                            // Not relevant
                p_filesz: asm.data_size.unwrap() as u64, // Section size in bytes
                p_memsz: asm.data_size.unwrap() as u64, // Same
                p_flags: 4,                            // Readable (4)
                p_align: 2 * link_opts.alignment,      // Alignment (4096 bytes)
            };
            w.write_program_header(&data_ph);
            program_headers.push(RaccoonProgramHeader(data_ph));
            w.write(&asm.opcodes[..asm.text_size]);
            w.write(&asm.opcodes[asm.text_size..]);
        }
        None => {
            w.write(&asm.opcodes[..asm.text_size]);
        }
    };

    Object {
        buf,
        file_header: RaccoonFileHeader(file_header),
        program_headers,
    }
}
