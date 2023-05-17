use fs_extra::dir::{copy, CopyOptions};
use git2::{ObjectType, Repository};
use plotters::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use tempfile::tempdir;
use tokei::{Config, Languages};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    // Copy the repository to the temporary directory
    let repo_path = Path::new(".");
    let temp_repo_path = dir.path();
    let mut options = CopyOptions::new(); // Initializes the copy options
    options.overwrite = true; // Overwrite existing files in destination
    copy(repo_path, temp_repo_path, &options)?;

    let repo = Repository::open(temp_repo_path)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    revwalk.push_head()?;

    let mut data: HashMap<String, Vec<u64>> = HashMap::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        repo.checkout_tree(&commit.tree().unwrap().into_object(), None)?;

        let mut languages = Languages::new();
        let config = Config::default();
        languages.get_statistics(&[repo_path.to_str().unwrap()], &[], &config);

        for (language_type, language) in languages {
            if language_type != tokei::LanguageType::Yaml {
                let entry = data.entry(language_type.to_string()).or_insert(Vec::new());
                entry.push(language.code as u64);
            }
        }
    }

    let max_loc = data
        .values()
        .map(|v| *v.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(0);

    // Drawing
    let root = BitMapBackend::new("lines_of_code.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Lines of Code Over Time", ("Arial", 50).into_font())
        .build_cartesian_2d(
            0..data.values().map(|v| v.len()).max().unwrap() as i32,
            0..(2 * max_loc) as i32,
        )?;

    chart.configure_mesh().draw()?;

    for (lang, counts) in &data {
        chart
            .draw_series(LineSeries::new(
                counts
                    .iter()
                    .enumerate()
                    .map(|(i, y)| (i as i32, *y as i32)),
                &RED,
            ))?
            .label(lang)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    }

    chart.configure_series_labels().draw()?;

    Ok(())
}