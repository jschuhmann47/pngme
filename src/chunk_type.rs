

struct ChunkType {
    bytes: [i32; 4]
}

impl ChunkType {
    fn bytes(&self) -> [i32; 4] {
        self.bytes
    }

    fn try_from(bytes: [i32; 4]) -> Result<ChunkType, &'static str> {
        if bytes.iter().any(|b| !is_character(*b)) {
            return Err("invalid chunk type");
        }
        Ok(ChunkType { bytes: bytes })

    }

    
}

fn is_character(byte: i32) -> bool {
    (byte >= 97 && byte <= 122) || (byte >= 65 && byte <= 90)
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

}
