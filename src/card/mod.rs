pub mod card_parser;
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CardBlock<T> {
    pub size: u16,
    pub data: T,
}

impl<T> CardBlock<T> {
    pub fn parse<F>(cursor: &mut Cursor<&[u8]>, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>) -> Result<T>,
    {
        let size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];
        cursor.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let data = parse_block(&mut inner_cursor).context(format!(
            "Failed to parse data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let consumed = inner_cursor.position();
        if consumed < size as u64 {
            let unused_bytes = size as u64 - consumed;
            log::warn!(
                "CardBlock of type {} did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                std::any::type_name::<T>(),
                size,
                consumed,
                unused_bytes
            );
        }

        Ok(CardBlock { size, data })
    }

    pub fn parse_dyn_size<F>(cursor: &mut Cursor<&[u8]>, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>, usize) -> Result<T>,
    {
        let size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];
        cursor.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let mut inner_cursor = Cursor::new(buf.as_slice());
        let data = parse_block(&mut inner_cursor, size as usize).context(format!(
            "Failed to parse data with dyn size in CardBlock of size {}",
            size
        ))?;

        let consumed = inner_cursor.position();
        if consumed < size as u64 {
            let unused_bytes = size as u64 - consumed;
            log::warn!(
                "CardBlock of type {} with dynamic size did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                std::any::type_name::<T>(),
                size,
                consumed,
                unused_bytes
            );
        }

        Ok(CardBlock { size, data })
    }
}
