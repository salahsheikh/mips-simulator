pub fn format_as_word(value: u32) -> String {
    format!("{}{:0>8x}", "0x", value)
}