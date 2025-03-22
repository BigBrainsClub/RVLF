use std::{
    fs::{File, OpenOptions}, io::{BufReader, Read, Seek, SeekFrom}, mem::take, path::Path
};
use smallvec::SmallVec;

pub struct Reader {
    reader: BufReader<File>,
    buffer: SmallVec<[u8; 32768]>,
    temp_buffer: [u8; 32768]
}

impl Reader {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader,
            buffer: SmallVec::with_capacity(32768),
            temp_buffer: [0; 32768]
        })
    }

    fn loading_vector(&mut self) -> Option<(SmallVec<[u8; 32768]>, usize)> {
        self.buffer.clear();
        let n = self.reader.read(&mut self.temp_buffer).ok()?;
        if n == 0 {
            return None;
        }
        if self.temp_buffer[n - 1] == b'\n' {
            self.buffer.extend_from_slice(&self.temp_buffer[..n]);
            return Some((take(&mut self.buffer), n))
        }
        if let Some(last_lf_pos) = self.temp_buffer[..n].iter().rposition(|&b| b == b'\n') {
            let current_pos = self.reader.stream_position().ok()?;
            let new_pos = current_pos - (n - last_lf_pos - 1) as u64;
            self.reader.seek(SeekFrom::Start(new_pos)).ok()?;
            self.buffer.extend_from_slice(&self.temp_buffer[..=last_lf_pos]);
            Some((self.buffer.clone(), last_lf_pos + 1))
        } else {
            self.buffer.extend_from_slice(&self.temp_buffer[..n]);
            Some((take(&mut self.buffer), n))
        }
    }
}

impl Iterator for Reader {
    type Item = (SmallVec<[u8; 32768]>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.loading_vector()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::ToSmallVec;
    use std::{fs::remove_file, io::Write, path::PathBuf};

    #[test]
    fn reading_vector_lines() -> std::io::Result<()> {
        let temp_file = create_temp_file()?;
        let mut reader = Reader::new(&temp_file)?;
        let (data, len) = reader.next().unwrap();
        let expected: SmallVec<[u8; 8192]> = b"example text\n".to_smallvec();
        assert_eq!(data, expected);
        assert_eq!(len, 12);
        remove_file(&temp_file)?;
        Ok(())
    }

    #[test]
    fn test_partial_line() -> std::io::Result<()> {
        let temp_file = create_temp_file_with_content(b"line1\nline2")?;
        let mut reader = Reader::new(&temp_file)?;
        let (data, len) = reader.next().unwrap();
        assert_eq!(data, b"line1\n".to_smallvec());
        assert_eq!(len, 6);
        let (data, len) = reader.next().unwrap();
        assert_eq!(data, b"line2".to_smallvec());
        assert_eq!(len, 5);
        remove_file(&temp_file)?;
        Ok(())
    }

    fn create_temp_file() -> std::io::Result<PathBuf> {
        let path = PathBuf::from("example.txt");
        let mut file = File::create(&path)?;
        file.write_all(b"example text\n")?;
        Ok(path)
    }

    fn create_temp_file_with_content(content: &[u8]) -> std::io::Result<PathBuf> {
        let path = PathBuf::from("test_file.txt");
        let mut file = File::create(&path)?;
        file.write_all(content)?;
        Ok(path)
    }
}