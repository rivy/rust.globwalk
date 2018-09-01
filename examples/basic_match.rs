extern crate globset;
extern crate globwalk;

use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use globwalk::GlobWalker;

use std::env::current_dir;

fn main() {
    let pat = "*x";
    let dir = current_dir().unwrap();
    println!("[ {} ] {}", dir.display(), pat);

    // let globset = GlobSetBuilder::new().add(Glob::new(pat).unwrap()).build().unwrap();
    let globset = GlobSetBuilder::new().add(GlobBuilder::new(pat).case_insensitive(true).literal_separator(true).build().ok().unwrap()).build().unwrap();

    for file in GlobWalker::from_globset(globset)
        .into_iter()
        .filter_map(Result::ok)
    {
        println!("{}", file.path().display());
    }
}
