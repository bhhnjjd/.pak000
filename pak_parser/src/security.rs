
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

// File size restrictions removed per user request
const MAX_FILE_COUNT: u32 = 10000;

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

    // Validate magic number
    if header.magic != [*b'P', *b'K', *b'3', *b'M'] {
        return Err("Invalid .pak file magic".to_string());
    }

    // Validate basic constraints
    if header.file_count > MAX_FILE_COUNT {
        return Err("Excessive file count in .pak".to_string());
    }

    if header.directory_offset as usize > data.len() {
        return Err("Directory offset exceeds file size".to_string());
    }

    let dir_start = header.directory_offset as usize;
    let dir_size = (header.file_count as usize) * 64;

    if dir_start + dir_size > data.len() {
        return Err("Invalid directory size".to_string());
    }


    // Validate each entry
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
    fn test_file_count_validation() {
        let mut large_data = vec![0; 64];
        // Set file count to MAX_FILE_COUNT + 1
        large_data[4] = (MAX_FILE_COUNT + 1) as u8;
        large_data[5] = ((MAX_FILE_COUNT + 1) >> 8) as u8;
        large_data[6] = ((MAX_FILE_COUNT + 1) >> 16) as u8;
        large_data[7] = ((MAX_FILE_COUNT + 1) >> 24) as u8;

        assert!(parse_pak(&large_data).is_err());
    }
}
