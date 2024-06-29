use clap::Parser;
use object::write::elf::{FileHeader, Writer};
use object::Endianness;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("a.out"))]
    output: String,

    file: String,
}

fn assemble_object(input: &str) {
    let mut buffer = Vec::new();
    let mut writer = Writer::new(Endianness::Little, true, &mut buffer);
    let entrypoint = 0;
    let header = FileHeader {
        os_abi: 0,
        abi_version: 0,
        e_type: 2,
        e_machine: 0x3E,
        e_entry: entrypoint,
        e_flags: 0,
    };
    writer.reserve_file_header();
    writer.write_file_header(&header).expect("write header");
    fs::write(input, buffer).expect("cannot write buffer");
}

#[allow(dead_code)]
fn assemble(input: &str, output: &str) -> io::Result<()> {
    let path = "/tmp/raccoon";
    if !Path::new(path)
        .try_exists()
        .expect("failed to check existence of path")
    {
        fs::create_dir(path).expect("failed to create directory in tmpfs");
    }
    let object = format!("{path}/{input}.o");
    let nasm_output = Command::new("nasm")
        .arg(input)
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(&object)
        .output()?;
    if !nasm_output.status.success() {
        println!(
            "{}",
            String::from_utf8(nasm_output.stderr).expect("expected valid utf-8")
        )
    }
    let ld_output = Command::new("ld")
        .arg(&object)
        .arg("-o")
        .arg(output)
        .output()?;
    if !ld_output.status.success() {
        println!(
            "{}",
            String::from_utf8(ld_output.stderr).expect("expected valid utf-8")
        )
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    //let _ = assemble(&args.file, &args.output);
    //let _ = assemble_iced();
    assemble_object(&args.file)
}
