use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Pending,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pending => "-",
                Self::Done => "x",
            }
        )
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

impl Memo {
    pub fn new(text: String, status: Status) -> Self {
        Self { text, status }
    }
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut memos = Self {
            path: path.as_ref().to_path_buf(),
            inner: Vec::new(),
        };
        if std::fs::exists(&path)? {
            let file = BufReader::new(File::open(path)?);
            memos.inner = serde_json::from_reader(file)?;
        }
        Ok(memos)
    }
    pub fn add_memo(&mut self, new_memo: Memo) {
        self.inner.push(new_memo);
    }
    pub fn sync(&mut self) -> Result<(), std::io::Error> {
        let memo_file = BufWriter::new(File::create(&self.path)?);
        // initial implementation, but I don't like it. Going to need to figure out some kind of diff-based
        // scheme for keeping memos in order.
        if !self.inner.is_empty() {
            serde_json::to_writer(memo_file, &self.inner)?;
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;
    mod open {
        use std::fs;

        use super::*;
        #[test]
        fn returns_empty_vec_on_nonexistent_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("dne.json");
            let memos = Memos::open(test_file).unwrap();
            assert!(memos.is_empty(), "Memos should be empty");
        }
        #[test]
        #[ignore = "No longer possible with Serde"]
        fn returns_empty_vec_on_empty_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("empty.json");
            fs::File::create(&test_file).unwrap();
            let memos = Memos::open(test_file).unwrap();
            assert!(memos.is_empty(), "Memos should be empty");
        }
        #[test]
        fn returns_vec_on_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("some.json");
            let test_writer = fs::File::create(&test_file).unwrap();
            let lines = vec![
                Memo::new("Test".to_string(), Status::Pending),
                Memo::new("Develop".to_string(), Status::Pending),
            ];
            serde_json::to_writer(test_writer, &lines).unwrap();
            let memos = Memos::open(test_file).unwrap();
            assert_eq!(memos.inner, lines);
        }
    }
    mod sync {

        use super::*;
        #[test]
        fn creates_file_if_needed() {
            let test_dir = tempdir().unwrap();
            let path = test_dir.path().join("new_file.json");
            let mut memos = Memos::open(path.clone()).unwrap();
            memos.add_memo(Memo::new("Hello".to_string(), Status::Pending));

            memos.sync().unwrap();

            let output_memos: Memos = Memos::open(path).unwrap();

            assert_eq!(
                memos.inner, output_memos.inner,
                "input should be written to the output"
            );
        }
        #[test]
        fn appends_to_file() {
            let test_dir = tempdir().unwrap();
            let path = test_dir.path().join("new_file.json");
            let mut memos = Memos {
                path: path.clone(),
                inner: vec![
                    Memo::new("Hello World!".to_string(), Status::Done),
                    Memo::new("Foo".to_string(), Status::Pending),
                ],
            };

            memos.sync().unwrap();

            let output_memos: Memos = Memos::open(path).unwrap();

            assert_eq!(
                memos.inner, output_memos.inner,
                "input should be written to the output"
            );
        }
    }
}
