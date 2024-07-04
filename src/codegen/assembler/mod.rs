#[cfg(test)]
mod tests;

use iced_x86::{code_asm::*, BlockEncoderOptions};

#[derive(Debug)]
pub struct Asm {
    pub opcodes: Vec<u8>,
    pub start: u64,
    pub data: Option<u64>,
    pub text_size: usize,
    pub data_size: Option<usize>,
    pub symbol_list: Option<SymbolList>,
}

#[derive(Debug)]
pub struct SymbolList {
    pub symbols: Vec<Symbol>,
}

impl SymbolList {
    pub fn new() -> SymbolList {
        SymbolList {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str, section: &str, offset: u64) {
        self.symbols.push(Symbol::new(
            String::from(name),
            String::from(section),
            offset,
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    name: String,
    section: String,
    offset: u64,
    address: u64,
}

impl Symbol {
    fn new(name: String, section: String, offset: u64) -> Symbol {
        Symbol {
            name,
            section,
            offset,
            address: 0,
        }
    }
}

pub fn hello_world(a: &mut CodeAssembler) -> Result<Asm, IcedError> {
    let mut start = a.create_label();
    let mut data = a.create_label();
    let mut hello = a.create_label();

    a.set_label(&mut start)?;
    a.mov(eax, 1)?;
    a.mov(edi, 1)?;
    a.mov(esi, 0)?;
    a.mov(edx, 15)?;
    a.syscall()?;
    a.mov(eax, 60)?;
    a.mov(edi, 69)?;
    a.syscall()?;

    a.set_label(&mut data)?;
    a.zero_bytes()?;
    a.set_label(&mut hello)?;
    a.db("Hello, World !\n".as_bytes())?;

    let result = a.assemble_options(0, BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)?;
    let bytes = result.inner.code_buffer.clone();
    let start = result.label_ip(&start).unwrap();
    let data = result.label_ip(&data).unwrap();
    let mut symbols = SymbolList::new();
    symbols.add("hello", "data", result.label_ip(&hello).unwrap() - data);

    let asm = Asm {
        opcodes: bytes.clone(),
        symbol_list: Some(symbols),
        start,
        data: Some(data),
        text_size: data as usize,
        data_size: Some(bytes.len() - data as usize),
    };
    println!("{:?}", asm);

    Ok(asm)
}
