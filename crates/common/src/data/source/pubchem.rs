use std::io::Read;

#[derive(Debug)]
pub struct EntryPubChem {
    pub smiles: crate::app::chem::types::SMILES
}

impl From<&str> for EntryPubChem {
    fn from(l: &str) -> Self {
        Self { smiles: l.to_string() }
    }
}

type DataPubChem = Vec<EntryPubChem>;

pub struct SourcePubChem {
    path: std::path::PathBuf,
    data: DataPubChem
}

impl SourcePubChem {
    pub fn new() -> Self {
        Self { path: std::path::PathBuf::new(), data: DataPubChem::new() }
    }

    pub fn set_path(&mut self, path_str: &std::ffi::OsStr) {
        self.path = std::path::PathBuf::from(path_str);
    }

    fn convert_lines(&mut self, lines: impl std::iter::Iterator<Item = std::io::Result<String>>) {
        self.data.clear();
        self.data = lines.map(|l| {
            let line = l.unwrap();
            EntryPubChem::from(line.as_str())
        })
        .collect();
    }

    pub fn load(&mut self, count: usize) {
        let pb = indicatif::ProgressBar::new(count as u64);
        pb.set_style(indicatif::ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message("Loading PubChem smiles");

        let file = std::fs::File::open(&self.path).unwrap();
        let input = std::io::BufReader::new(file);
        let mut bytes = flate2::bufread::GzDecoder::new(input).bytes();
        let mut i: usize = 0;
        let mut buf = Vec::<u8>::new();
        while let Some(b_o) = bytes.next() {
            let b = b_o.unwrap();
            if b == b'\n' {
                let line = std::str::from_utf8(&buf).unwrap();
                let parts: Vec<&str> = line.split_ascii_whitespace().collect();
                if parts.len() == 2 {
                    self.data.push(EntryPubChem::from(parts[1]));
                    i += 1;
                    if i == count {
                        break;
                    }
                    if i % 1000 == 0 {
                        pb.set_position(i as u64);
                    }
                } else {
                    crate::logging::error(format!("PubChem read line error: {}", line).as_str());
                }
                buf.clear();
            } else {
                if b != b'\r' {
                    buf.push(b);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut spc = SourcePubChem::new();
        let path = std::ffi::OsString::from("../../../../../.chiral/CID-SMILES.gz");
        spc.set_path(&path);
        spc.load(1000 * 100);
        assert_eq!(100 * 100  * 10, spc.data.len());
    }
}

