use std::io::Cursor;

pub fn extract_u8_bits_into_tup(byte: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        byte >> 7 & 1,
        byte >> 6 & 1,
        byte >> 5 & 1,
        byte >> 4 & 1,
        byte >> 3 & 1,
        byte >> 2 & 1,
        byte >> 1 & 1,
        byte & 1,
    )
}
pub fn extract_u16_bits_into_tup(
    byte: u16,
) -> (
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
) {
    (
        (byte >> 15 & 1) as u8,
        (byte >> 14 & 1) as u8,
        (byte >> 13 & 1) as u8,
        (byte >> 12 & 1) as u8,
        (byte >> 11 & 1) as u8,
        (byte >> 10 & 1) as u8,
        (byte >> 9 & 1) as u8,
        (byte >> 8 & 1) as u8,
        (byte >> 7 & 1) as u8,
        (byte >> 6 & 1) as u8,
        (byte >> 5 & 1) as u8,
        (byte >> 4 & 1) as u8,
        (byte >> 3 & 1) as u8,
        (byte >> 2 & 1) as u8,
        (byte >> 1 & 1) as u8,
        (byte & 1) as u8,
    )
}

pub trait TakeExact {
    fn take_exact(&mut self, size: usize) -> Cursor<&[u8]>;
}

impl TakeExact for Cursor<&[u8]> {
    fn take_exact(&mut self, size: usize) -> Cursor<&[u8]> {
        let position = self.position() as usize;
        let remaining = self.get_ref().len() - position;
        if size > remaining {
            panic!(
                "Attempted to take {} bytes, but only {} bytes remain",
                size, remaining
            );
        }
        let end = position + size;
        let slice = &self.get_ref()[position..end];
        self.set_position(end as u64);
        Cursor::new(slice)
    }
}
