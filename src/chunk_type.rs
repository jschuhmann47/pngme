use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        self.bytes().iter().all(|b| b.is_ascii_alphabetic()) && self.bytes().len() == 4
    }

    fn is_critical(&self) -> bool {
        self.is_zero_bit_from_byte_at(5, 0)
    }

    fn is_public(&self) -> bool {
        self.is_zero_bit_from_byte_at(5, 1)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.is_zero_bit_from_byte_at(5, 2)
    }

    fn is_safe_to_copy(&self) -> bool {
        !self.is_zero_bit_from_byte_at(5, 3)
    }

    fn is_zero_bit_from_byte_at(&self, position: u8, byte_number: usize) -> bool {
        if position > 8 || byte_number > 4 {
            return false;
        }
        // moves the bit 1 `position` positions to the left so that it is in bit `byte_number` and we do a binary AND, keeping only that bit in 1 or 0
        (self.bytes()[byte_number] & (1 << position)) == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let new_chunk_type = ChunkType { bytes: value };
        if !new_chunk_type.is_valid() {
            return Err(String::from("invalid chunk type"));
        }
        Ok(new_chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(String::from("invalid chunk type"));
        }
        let mut bytes: [u8; 4] = [0; 4];
        for (i, c) in s.bytes().enumerate() {
            bytes[i] = c
        }
        let new_chunk_type = ChunkType { bytes };
        if !new_chunk_type.is_valid() {
            return Err(String::from("invalid chunk type"));
        }
        Ok(ChunkType { bytes })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(self.bytes().as_slice()).unwrap_or_else(|_| "non utf-8")
        )
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        assert_eq!("RuSt", chunk_type_1.to_string());
        assert_eq!("RuSt", chunk_type_2.to_string());
    }
}
