pub fn pop_u16(buf: &[u8], cursor: &mut usize) -> Option<u16> {
    let hi = *buf.get(*cursor)? as u16;
    *cursor = *cursor + 1;
    let lo = *buf.get(*cursor)? as u16;
    *cursor = *cursor + 1;
    Some((hi << 8) | lo)
}
