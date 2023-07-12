use anyhow::Error;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, io};
use tempfile::tempdir;
use tokei::{Config, Languages};

#[derive(Debug, Serialize, Deserialize)]
struct Stat {
    commit: String,
    total: usize,
}

fn main() -> Result<(), Error> {
    let tmp_dir = tempdir().expect("Temporary directory to be created");

    let repo = Repository::clone_recurse(
        env::current_dir()
            .expect("Getting current directory not to fail")
            .to_str()
            .expect("Converting current directory to string not to fail"),
        tmp_dir.path(),
    )
    .expect("Checking out repo not to fail");

    let mut revwalk = repo.revwalk().expect("Creating revwalk not to fail");
    revwalk.push_head()?;

    let mut stats: HashMap<String, Languages> = HashMap::new();

    for oid in revwalk.flatten() {
        let commit_hash = oid.clone().to_string();
        let commit = repo.find_commit(oid).unwrap();

        let _ = repo.branch(&commit_hash, &commit, false);

        let obj = repo
            .revparse_single(&("refs/heads/".to_owned() + &commit_hash))
            .unwrap();

        repo.checkout_tree(&obj, None)
            .expect("checking out commit not to fail");

        repo.set_head(&("refs/heads/".to_owned() + &commit_hash))
            .expect("setting head not to fail");

        repo.set_head_detached(commit.id()).unwrap();

        let mut langs = Languages::new();
        langs.get_statistics(&[&tmp_dir], &[".git", "target"], &Config::default());

        stats.insert(commit_hash, langs);
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for (commit, stat) in stats {
        wtr.serialize(Stat {
            commit,
            total: stat.total().code,
        })?;
    }

    wtr.flush()?;

    Ok(())
}
