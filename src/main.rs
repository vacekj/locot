use anyhow::Error;
use chrono::{Date, DateTime, Local, TimeZone, Utc};
use git2::{Repository, Time};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::os::macos::raw::stat;
use std::{env, io};
use tempfile::tempdir;
use tokei::{Config, LanguageType, Languages};

#[derive(Debug, Serialize, Deserialize)]
struct Stat {
    commit: String,
    date: DateTime<Utc>,
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

    let mut stats: HashMap<(String, DateTime<Utc>), Languages> = HashMap::new();

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
        stats.insert((commit_hash, time), langs);
    }

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    let mut all_langs = stats
        .values()
        .collect::<Vec<&Languages>>()
        .iter_mut()
        .flat_map(|lang| {
            return lang.iter().filter_map(|(langtype, lang)| {
                let mut lang = lang.clone();
                lang.total();

                if lang.lines() > 0 {
                    Some(*langtype)
                } else {
                    None
                }
            });
        })
        .unique()
        .collect::<Vec<LanguageType>>();

    #[derive(Serialize)]
    struct Header {
        commit: String,
        time: String,
        total: String,
        langs: Vec<String>,
    }

    wtr.serialize(Header {
        commit: "Commit".into(),
        time: "Time".into(),
        total: "Total (code)".into(),
        langs: all_langs.iter().map(|t| t.to_string()).collect(),
    })
    .expect("Writing header not to fail");

    for (commit, stat) in stats.iter().sorted_by_key(|(commit, _)| &commit.1) {
        wtr.serialize(Stat {
            commit: commit.0.clone(),
            date: commit.1,
            total: stat.total().code,
            langs: all_langs
                .iter_mut()
                .map(|language_type| {
                    let found_lang = stat
                        .iter()
                        .find(|(typ, lang)| typ.eq(&language_type))
                        .map(|(typ, lang)| lang.clone());
                    if let Some(mut lang) = found_lang {
                        lang.total();
                        lang.lines()
                    } else {
                        0
                    }
                })
                .collect(),
        })?;
    }

    wtr.flush()?;

    Ok(())
}
