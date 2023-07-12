use git2::{Commit, Repository};
use std::{env, fs};
use tempfile::tempdir;
use tokei::{CodeStats, Config, Languages};

fn main() {
    let tmp_dir = tempdir().expect("Temporary directory to be created");

    let repo = Repository::clone_recurse(
        env::current_dir().unwrap().to_str().unwrap(),
        tmp_dir.path(),
    )
    .unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit = repo.find_commit(oid.unwrap()).unwrap();

        println!("Checking commit {}", commit.id());

        repo.set_head_detached(commit.id()).unwrap();

        let mut langs = Languages::new();
        langs.get_statistics(&[&tmp_dir], &[".git", "target"], &Config::default());

        dbg!(langs);
    }

    println!("Results written to loc.csv");

    fs::remove_dir_all(tmp_dir).unwrap();
}
