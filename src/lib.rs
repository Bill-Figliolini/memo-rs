use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>, std::io::Error> {
    if std::fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

pub fn sync(memos: &Vec<String>, file: impl AsRef<Path>) -> Result<(), std::io::Error> {
    Ok(())
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
            let memos = open(test_file).unwrap();
            assert!(memos.is_empty(), "Memos should be empty");
        }
        #[test]
        fn returns_empty_vec_on_empty_file() {
            let test_dir = tempdir().unwrap();
            let test_file = test_dir.path().join("empty.txt");
            fs::File::create(&test_file).unwrap();
            let memos = open(test_file).unwrap();
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
            let memos = open(test_file).unwrap();
            assert_eq!(memos, lines);
        }
    }
    mod sync {
        use super::*;
    }
}
