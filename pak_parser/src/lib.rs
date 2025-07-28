
use wasm_bindgen::prelude::*;
#[derive(Debug)]
struct PakHeader {
    magic: [u8; 4],
    file_count: u32,
    directory_offset: u32,
}

#[derive(Debug)]
struct PakEntry {
    name: [u8; 56],
    offset: u32,
    size: u32,
}

pub fn parse_pak(data: &[u8]) -> Result<(PakHeader, Vec<PakEntry>), String> {
    if data.len() < 64 {
        return Err("File too small for .pak format".to_string());
    }

    let mut header = PakHeader {
        magic: [data[0], data[1], data[2], data[3]],
        file_count: u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
        directory_offset: u32::from_le_bytes([
            data[8], data[9], data[10], data[11],
        ]),
    };

    if header.magic != [b'P', b'K', b'3', b'M'] {
        return Err("Invalid .pak file magic".to_string());
    }

    let dir_start = header.directory_offset as usize;
    let dir_size = (header.file_count as usize) * 64;

    if data.len() < dir_start + dir_size {
        return Err("Invalid directory size".to_string());
    }

    let entries = (0..header.file_count)
        .map(|i| {
            let start = dir_start + (i as usize) * 64;
            let mut name = [0u8; 56];
            name[..12].copy_from_slice(&data[start..start + 12]);
            PakEntry {
                name,
                offset: u32::from_le_bytes([
                    data[start + 12], data[start + 13], data[start + 14], data[start + 15],
                ]),
                size: u32::from_le_bytes([
                    data[start + 16], data[start + 17], data[start + 18], data[start + 19],
                ]),
            }
        })
        .collect();

    Ok((header, entries))
}

#[wasm_bindgen]
pub fn parse_pak_js(data: &[u8]) -> Result<u32, JsValue> {
    match parse_pak(data) {
        Ok((header, _)) => Ok(header.file_count),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pak_parser() {
        let mut data = vec![0u8; 128];
        data[0..4].copy_from_slice(b"PK3M");
        data[4..8].copy_from_slice(&1u32.to_le_bytes());
        data[8..12].copy_from_slice(&64u32.to_le_bytes());
        match parse_pak(&data) {
            Ok((header, entries)) => {
                assert_eq!(header.magic, [b'P', b'K', b'3', b'M']);
                assert_eq!(entries.len(), header.file_count as usize);
            }
            Err(e) => {
                panic!("Parsing failed: {}", e);
            }
        }
    }
}

