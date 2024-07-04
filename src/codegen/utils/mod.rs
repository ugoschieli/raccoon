#[cfg(test)]
mod tests;

use hex;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        self.split(" ")
            .map(|s| match s.strip_prefix("0x") {
                Some(stripped_s) => stripped_s,
                None => s,
            })
            .map(|s| hex::decode(s).unwrap())
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
    }
}

// Example that use assembler and linker from OS
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
