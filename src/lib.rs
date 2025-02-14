use std::{fs::{File, OpenOptions}, io::{BufReader, Read}, path::Path};

use smallvec::SmallVec;

pub struct Reader {
    reader: BufReader<File>,
    buffer: SmallVec<[u8; 1024]>
}

impl Reader {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        Ok(
            Self {
                reader: reader,
                buffer: SmallVec::new()
            }
        )
    }

    fn loading_vector(&mut self) -> Option<(SmallVec<[u8; 1024]>, usize)> {
        self.buffer.clear();
        let mut temp_buffer = SmallVec::<[u8; 1024]>::new();
        temp_buffer.resize(1024, 0);

        match self.reader.read(&mut temp_buffer) {
            Ok(0) => None,
            Ok(n) => {
                self.buffer.extend_from_slice(&temp_buffer[..n]);
                Some((self.buffer.clone(), n))
            },
            Err(_) => None
        }
    }
}

impl Iterator for Reader {
    type Item = (SmallVec<[u8; 1024]>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.loading_vector()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::PathBuf};

    use smallvec::ToSmallVec;

    use super::*;

    #[test]
    fn reading_vector_lines() -> std::io::Result<()> {
        let temp_file = create_temp_file()?;
        let reader = Reader::new(&temp_file)?;
        for (lines, len) in reader {
            let equal_vector: SmallVec<[u8; 1024]> = [101, 120, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116].to_smallvec();
            assert_eq!(lines, equal_vector);
            assert_eq!(len, 12)
        }
        remove_file(&temp_file)?;
        Ok(())
    }
    fn create_temp_file() -> std::io::Result<PathBuf> {
        let path = PathBuf::from("example.txt");
        if !path.is_file() {
            File::create(&path)?;
        }
        Ok(path)
    }
}
