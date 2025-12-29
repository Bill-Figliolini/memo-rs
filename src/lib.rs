use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
#[derive(Debug, PartialEq, Eq)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<String>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut memos = Memos {
            path: path.as_ref().to_path_buf(),
            inner: Vec::new(),
        };
        if std::fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            for memo in file.lines() {
                memos.add_memo(memo?);
            }
        }
        Ok(memos)
    }
    pub fn add_memo(&mut self, new_memo: String) {
        self.inner.push(new_memo);
    }
    pub fn sync(&mut self) -> Result<(), std::io::Error> {
        let mut memo_file = File::options().create(true).append(true).open(&self.path)?;
        // initial implementation, but I don't like it. Going to need to figure out some kind of diff-based
        // scheme for keeping memos in order.
        if !self.inner.is_empty() {
            writeln!(memo_file, "{}", self.inner.join("\n"))
        } else {
            Ok(())
        }
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
        use std::{fs, io::Write};

        use super::*;
        #[test]
        fn returns_empty_vec_on_nonexistent_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("dne.txt");
            let memos = Memos::open(test_file).unwrap();
            assert!(memos.is_empty(), "Memos should be empty");
        }
        #[test]
        fn returns_empty_vec_on_empty_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("empty.txt");
            fs::File::create(&test_file).unwrap();
            let memos = Memos::open(test_file).unwrap();
            assert!(memos.is_empty(), "Memos should be empty");
        }
        #[test]
        fn returns_vec_on_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("some.txt");
            let mut test_writer = fs::File::create(&test_file).unwrap();
            let lines = vec!["Test".to_string(), "Develop".to_string()];
            for line in lines.iter() {
                _ = writeln!(test_writer, "{line}");
            }
            let memos = Memos::open(test_file).unwrap();
            assert_eq!(memos.inner, lines);
        }
    }
    mod sync {

        use super::*;
        #[test]
        fn creates_file_if_needed() {
            let test_dir = tempdir().unwrap();
            let path = test_dir.path().join("new_file.txt");
            let mut memos = Memos::open(path.clone()).unwrap();

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
            let path = test_dir.path().join("new_file.txt");
            let mut memos = Memos {
                path: path.clone(),
                inner: vec!["Hello World!".to_string(), "Foo".to_string()],
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
