use crate::chunk_type::ChunkType;
use crate::Result;
use anyhow::{anyhow, bail, Error};
use std::fmt::Display;

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub bytes: Vec<u8>,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    // 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233,
    fn try_from(value: &[u8]) -> Result<Self> {
        let mut chunk_bytes = [0; 4];
        chunk_bytes.clone_from_slice(&value[4..8]);
        let chunk_type = ChunkType::try_from(chunk_bytes)?;

        let chunk = Self {
            chunk_type,
            data: value[8..value.len() - 4].to_vec(),
            bytes: value.to_vec(),
        };

        let mut crc = [0; 4];
        crc.clone_from_slice(&value[value.len() - 4..]);

        let crc_bytes: [u8; 4] = chunk.crc().to_be_bytes();

        if crc != crc_bytes {
            bail!("crc and calculated crc don't match")
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self {
            chunk_type,
            data: data.clone(),
            bytes: data,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        let input: Vec<u8> = self
            .chunk_type()
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&input)
    }

    pub fn data_as_string(&self) -> Result<String> {
        std::str::from_utf8(self.data())
            .map(|s| s.to_string())
            .map_err(|e| anyhow!(e))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length() // length.
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter()) // chunk type.
            .chain(self.data.iter()) // data,
            .chain(self.crc().to_be_bytes().iter()) // crc
            .cloned()
            .collect()
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
