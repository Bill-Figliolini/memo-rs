use anyhow::Result;
use std::env;

use memo_rs::{open, sync};

fn main() -> Result<()> {
    let memos_file = "memos.txt";
    let mut memos = open(memos_file)?;
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        for memo in memos {
            println!("{memo}");
        }
    } else {
        let memo = args.join(" ");
        memos.push(memo);
        sync(&memos, memos_file);
    }
    Ok(())
}
