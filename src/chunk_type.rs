use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_critical(&self) -> bool {
        // moves the bit 1 four positions to the left so that it is in bit five, and we do a binary AND, keeping only that bit in 1 or 0
        let first_char = self.bytes()[0];
        (first_char & (1 << 5)) == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|b| !b.is_ascii()) {
            return Err(String::from("invalid chunk type"));
        }
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(String::from("invalid str length"));
        }
        let mut bytes: [u8; 4] = [0; 4];
        for (i, c) in s.bytes().enumerate() {
            bytes[i] = c
        }
        Ok(ChunkType { bytes })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(self.bytes().as_slice()).expect("non utf8")
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
    pub fn test_chunk_type_display() {
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!("RuSt", actual.to_string());
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
}
