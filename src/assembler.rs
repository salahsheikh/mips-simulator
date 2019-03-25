use crate::parser;

pub fn assemble_instruction(word: String) -> u32 {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    match components.get(0).unwrap().to_lowercase().as_str() {
        "addi" => {
            let rd: u8 = parser::parse_register(components.get(1).unwrap());
            let rs: u8 = parser::parse_register(components.get(2).unwrap());
            let imm_str = components.get(2).unwrap();
            let mut imm: i16 = 0;
            if imm_str.starts_with("0x") {
            } else {
                imm = i16::from_str_radix(imm_str, 10).unwrap();
            }
            return (0x08 << 32) + (rs << 27) as u32;
        },
        _ => {
            return 0xffff_ffff; // -1
        }
    }
}