// basic_match
// ref: [Add examples to your Rust libraries](http://xion.io/post/code/rust-examples.html)[`@`](https://archive.is/E8Lpb)
extern crate globwalk;

use std::env::current_dir;

fn main() {
    let pat = "*README*";
    let dir = current_dir().unwrap();
    println!("[ {} ] {}", dir.display(), pat);
    for file in globwalk::GlobWalker::new(dir, pat)
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
    {
        println!("{}", file.path().display());
    }
}
