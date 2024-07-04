use super::*;
use crate::codegen::assembler::hello_world;
use iced_x86::code_asm::CodeAssembler;

#[test]
fn hello_world_elf() {
    let mut a = CodeAssembler::new(64).unwrap();
    let asm = hello_world(&mut a).unwrap();
    let link_opts = LinkOpts {
        base_addr: 0x1000000,
        alignment: 0x1000,
    };

    let object = create_elf(&asm, &link_opts);

    assert_eq!(
        object,
        Object {
            buf: vec![],
            file_header: RaccoonFileHeader(FileHeader {
                os_abi: 0,
                abi_version: 0,
                e_type: 2,
                e_machine: 0x3E,
                e_entry: 0x10010b0,
                e_flags: 0,
            }),
            program_headers: vec![
                RaccoonProgramHeader(ProgramHeader {
                    p_type: 1,
                    p_offset: 64 + 2 * 56,
                    p_vaddr: 0x1000000 + 0x1000 + (64 + 2 * 56),
                    p_paddr: 0,
                    p_filesz: 34,
                    p_memsz: 34,
                    p_flags: 5,
                    p_align: 0x1000 as u64,
                }),
                RaccoonProgramHeader(ProgramHeader {
                    p_type: 1,
                    p_offset: 64 + 2 * 56 + 34,
                    p_vaddr: 0x1000000 + 0x2000 + 64 + 2 * 56 + 34,
                    p_paddr: 0,
                    p_filesz: 15,
                    p_memsz: 15,
                    p_flags: 4,
                    p_align: 0x2000 as u64,
                })
            ]
        }
    );
}
