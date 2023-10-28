#[inline]
pub const fn adjust_endianess16(word: u16) -> u16 {
    word.to_be()
}

#[inline]
pub const fn adjust_endianess32(d_word: u32) -> u32 {
    d_word.to_be()
}

#[inline]
pub const fn adjust_endianess64(q_word: u64) -> u64 {
    q_word.to_be()
}
