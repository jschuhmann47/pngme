use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|b| !is_character(*b)) {
            return Err(String::from("invalid chunk type"));
        }
        Ok(ChunkType { bytes: value })
    }
}
fn is_character(byte: u8) -> bool {
    byte.is_ascii()
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
}
