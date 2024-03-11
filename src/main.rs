use anyhow::Error;
use chrono::{Date, DateTime, Local, TimeZone};
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
    langs: Vec<usize>,
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

    /*TODO:
    1. get the element with the most languages (track all languages from the beginning)
    2. write the languages as headers
     3. transform the languages in the btreemap to a hashmap, fill in the missing ones as 0, sort by name, and then convert to vec of tuples
      4. serialize into the Stat struct, keeping the sorting */

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
        let time = DateTime::from_timestamp(commit.time().seconds(), 0)
            .expect("Creating time from commit not to fail");
        stats.insert(commit_hash, langs);
    }

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    for (commit, stat) in stats {
        wtr.serialize(Stat {
            commit,
            total: stat.total().code,
            langs: stat
                .iter()
                .map(|(typ, lang)| lang.reports.last().unwrap().stats.code)
                .collect(),
        })?;
    }

    wtr.flush()?;

    Ok(())
}
