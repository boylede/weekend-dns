pub fn pop_u16(buf: &[u8], cursor: &mut usize) -> Option<u16> {
    let hi = *buf.get(*cursor)? as u16;
    *cursor = *cursor + 1;
    let lo = *buf.get(*cursor)? as u16;
    *cursor = *cursor + 1;
    Some((hi << 8) | lo)
}

pub fn pop_u8(buf: &[u8], cursor: &mut usize) -> Option<u8> {
    let lo = *buf.get(*cursor)?;
    *cursor = *cursor + 1;
    Some(lo)
}

pub fn pop_collection<T: FromBytes + Sized>(
    buf: &[u8],
    cursor: &mut usize,
    count: usize,
) -> Option<Vec<T>> {
    let mut c = *cursor;
    let vec: Vec<T> = (0..count)
        .map_while(|_index| -> Option<T> {
            let Some(item) = <T as FromBytes>::from_bytes(buf, &mut c) else {
                return None};
            Some(item)
        })
        .collect();
    *cursor = c;
    Some(vec)
}

pub trait FromBytes
where
    Self: Sized,
{
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self>;
}

impl FromBytes for u8 {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let byte = *buf.get(*cursor)?;
        *cursor += 1;
        Some(byte)
    }
}

impl FromBytes for char {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let byte = *buf.get(*cursor)?;
        let char = byte.try_into().ok()?;
        *cursor += 1;
        Some(char)
    }
}

impl FromBytes for i32 {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let a = *buf.get(*cursor)? as u32;
        *cursor += 1;
        let b = *buf.get(*cursor)?  as u32;
        *cursor += 1;
        let c = *buf.get(*cursor)?  as u32;
        *cursor += 1;
        let d = *buf.get(*cursor)?  as u32;
        *cursor += 1;

        let num = a << 24 | b << 16 | c << 8 | d;
        Some(num as i32)
    }
}