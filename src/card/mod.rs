pub mod card_parser;
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardBlock<T> {
    pub size: u16,
    pub data: T,
}

impl<T> CardBlock<T> {
    pub fn parse<F>(reader: &mut dyn Read, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut dyn Read) -> Result<T>,
    {
        let size = reader
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let mut buf_slice = buf.as_slice();
        let initial_len = buf_slice.len();
        let data = parse_block(&mut buf_slice).context(format!(
            "Failed to parse data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let consumed = initial_len - buf_slice.len();
        if consumed < size as usize {
            let unused_bytes = size as usize - consumed;
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

    pub fn parse_dyn_size<F>(reader: &mut dyn Read, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut dyn Read, usize) -> Result<T>,
    {
        let size = reader
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];

        reader.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let mut buf_slice = buf.as_slice();
        let initial_len = buf_slice.len();
        let data = parse_block(&mut buf_slice, size as usize).context(format!(
            "Failed to parse data with dyn size in CardBlock of size {}",
            size
        ))?;

        let consumed = initial_len - buf_slice.len();
        if consumed < size as usize {
            let unused_bytes = size as usize - consumed;
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
