use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
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
    let mut memo_file = File::options().create(true).append(true).open(file)?;
    // initial implementation, but I don't like it. Going to need to figure out some kind of diff-based
    // scheme for keeping memos in order.
    writeln!(memo_file, "{}", memos.join("\n"))
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
        use std::io::read_to_string;

        use super::*;
        #[test]
        fn creates_file_if_needed() {
            let test_dir = tempdir().unwrap();
            let path = test_dir.path().join("new_file.txt");
            let input_memos = vec!["Hello World".to_string()];

            sync(&input_memos, &path).unwrap();

            let output_memos: Vec<String> = open(path).unwrap();

            assert_eq!(
                input_memos, output_memos,
                "input should be written to the output"
            );
        }
        #[test]
        fn appends_to_file() {
            let test_dir = tempdir().unwrap();
            let path = test_dir.path().join("new_file.txt");
            let mut input_memos = vec!["Hello World".to_string()];

            sync(&input_memos, &path).unwrap();
            sync(&input_memos, &path).unwrap();

            input_memos.push("Hello World".to_string());

            let output_memos: Vec<String> = open(path).unwrap();

            assert_eq!(
                input_memos, output_memos,
                "input should be written to the output"
            );
        }
    }
}
