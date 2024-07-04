use clap::Parser;
use iced_x86::code_asm::CodeAssembler;
use raccoon::codegen::assembler::*;
use raccoon::codegen::elf::*;
use raccoon::codegen::linker::*;
use std::fs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("a.out"))]
    output: String,

    file: String,
}

fn main() {
    let args = Args::parse();

    let mut a = CodeAssembler::new(64).unwrap();
    let link_opts = LinkOpts {
        base_addr: 0x1000000,
        alignment: 0x1000,
    };

    let asm = hello_world(&mut a).unwrap();
    let mut object = create_elf(&asm, &link_opts);
    link_elf(&asm, &mut object, &link_opts);

    fs::write(args.file, object.buf).unwrap();
}
