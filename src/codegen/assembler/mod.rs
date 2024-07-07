#[cfg(test)]
mod tests;

use iced_x86::{code_asm::*, BlockEncoderOptions};

#[derive(Debug)]
pub struct Asm {
    pub start: u64,
    pub data: Option<u64>,
    pub text_size: usize,
    pub data_size: Option<usize>,
    pub symbols: Vec<Symbol>,
    asm_result: Box<CodeAssemblerResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub section: String,
    pub offset: u64,
    pub address: u64,
    pub positions: Vec<CodeLabel>,
    pub position_offsets: Vec<usize>,
}

impl Asm {
    fn new(a: &mut CodeAssembler, start: &CodeLabel, data: &CodeLabel) -> Self {
        let result = a
            .assemble_options(0, BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
            .unwrap();
        let bytes = result.inner.code_buffer.clone();
        let start = result.label_ip(&start).unwrap();
        let data = result.label_ip(&data).unwrap();

        Asm {
            start,
            data: Some(data),
            text_size: data as usize,
            data_size: Some(bytes.len() - data as usize),
            symbols: Vec::new(),
            asm_result: Box::new(result),
        }
    }

    pub fn get_instructions(&self) -> &Vec<u8> {
        self.asm_result.inner.code_buffer.as_ref()
    }

    pub fn add_symbol(&mut self, label: &CodeLabel, symbol: Symbol) {
        let indexs: Vec<usize> = symbol
            .positions
            .iter()
            .map(|x| self.asm_result.label_ip(&x).unwrap() as usize)
            .collect();
        self.symbols.push(Symbol {
            name: symbol.name,
            section: symbol.section,
            address: 0,
            offset: self.asm_result.label_ip(label).unwrap(),
            positions: symbol.positions,
            position_offsets: indexs,
        });
    }
}

impl Symbol {
    fn new(name: &str, section: &str) -> Symbol {
        Symbol {
            name: String::from(name),
            section: String::from(section),
            offset: 0,
            address: 0,
            positions: Vec::new(),
            position_offsets: Vec::new(),
        }
    }

    fn add_pos(&mut self, a: &mut CodeAssembler) {
        let mut label = a.create_label();
        a.set_label(&mut label).unwrap();
        self.positions.push(label);
        a.zero_bytes().unwrap();
    }
}

pub fn hello_world(a: &mut CodeAssembler) -> Result<Asm, IcedError> {
    let mut start = a.create_label();
    let mut data = a.create_label();
    let mut hello = a.create_label();
    let mut hello_sym = Symbol::new("hello", "data");

    a.set_label(&mut start)?;
    a.mov(eax, 1)?;
    a.mov(edi, 1)?;
    hello_sym.add_pos(a);
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

    let mut asm = Asm::new(a, &start, &data);
    asm.add_symbol(&hello, hello_sym);
    //println!("{:?}", asm);
    //println!("{:?}", asm.instructions[10]);

    Ok(asm)
}
