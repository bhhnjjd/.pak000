use typed_arena::Arena;

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

    if header.magic != [*b'P', *b'K', *b'3', *b'M'] {
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
            PakEntry {
                name: [
                    data[start], data[start + 1], data[start + 2], data[start + 3],
                    data[start + 4], data[start + 5], data[start + 6], data[start + 7],
                    data[start + 8], data[start + 9], data[start + 10], data[start + 11],
                ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pak_parser() {
        let data = vec![0; 1024]; // Replace with actual sample.pak content
        match parse_pak(&data) {
            Ok((header, entries)) => {
                assert_eq!(header.magic, [*b'P', *b'K', *b'3', *b'M']);
                assert_eq!(entries.len(), header.file_count as usize);
            }
            Err(e) => {
                panic!("Parsing failed: {}", e);
            }
        }
    }
}