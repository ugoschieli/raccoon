use super::*;
use crate::codegen::utils::ToBytes;

#[test]
fn hello_world_asm() {
    let mut a = CodeAssembler::new(64).unwrap();
    let asm = hello_world(&mut a).unwrap();
    let mut opcodes = "B8 01 00 00 00 \
                  BF 01 00 00 00 \
                  BE 00 00 00 00 \
                  BA 0F 00 00 00 \
                  0F 05 \
                  B8 3C 00 00 00 \
                  BF 45 00 00 00 \
                  0F 05 \
                  "
    .to_bytes();
    opcodes.extend("Hello, World !\n".as_bytes());
    //assert_eq!(
    //    asm,
    //    Asm {
    //        opcodes,
    //        start: 0,
    //        data: Some(34),
    //        text_size: 34,
    //        data_size: Some(15)
    //    }
    //);
}
