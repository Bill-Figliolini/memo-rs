use anyhow::Result;
use std::env;

use memo_rs::{Memo, Memos, Status};

fn main() -> Result<()> {
    let memos_file = "memos.txt";
    let mut memos = Memos::open(memos_file)?;
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        for memo in memos.inner {
            println!("{}", memo);
        }
    } else {
        let memo = args.join(" ");
        let memo = Memo::new(memo, Status::Pending);
        memos.add_memo(memo);
        memos.sync()?;
    }
    Ok(())
}
