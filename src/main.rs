use std::{fs, path::Path};

use clap::Parser;
use git2::{Error, Oid, Repository};

#[derive(Parser)]
#[command(
    name = "Volunteer Hours",
    author = "ap-1",
    about = "A CLI for tracking volunteer hours with .dev_hours",
    long_about = "This tool uses the .dev_hours file in hack.place() repositories to generate a descriptive report of volunteer hours."
)]
struct Cli {
    /// URL of the repository to analyze
    #[arg(short = 'u', long)]
    url: String,

    /// Your full name
    #[arg(short = 'n', long)]
    name: String,

    /// Directory to clone the repository to
    #[arg(short = 'd', long, default_value = "./repo")]
    dir: String,
}

fn track_hours(cli: &Cli) -> Result<(), Error> {
    let repo = Repository::clone(&cli.url, &cli.dir)?;
    let repo_name = repo
        .config()
        .and_then(|config| config.get_string("remote.origin.url"))
        .map(|url| {
            let mut components: Vec<&str> = url.split('/').rev().take(2).collect();

            components.reverse();
            components.join("/").trim_end_matches(".git").to_string()
        })
        .unwrap();

    let file_path = Path::new(".dev_hours");
    let date = chrono::Local::now().format("%Y-%m-%d");

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commit_ids: Vec<Oid> = revwalk.filter_map(|r| r.ok()).collect();
    let commit_ids = commit_ids.iter().rev();

    let mut total_hours = 0.0;
    let mut commit_messages: Vec<String> = Vec::new();
    let mut has_prehistory = false;
    let mut prehistory_complete = false;

    println!("{}'s Volunteer Hours for {}", &cli.name, repo_name);
    println!("Compiled by hack.place() on {}", date);

    for commit_id in commit_ids {
        let commit = repo.find_commit(*commit_id)?;
        let tree = commit.tree()?;

        if let Ok(file_entry) = tree.get_path(&file_path) {
            let blob = repo.find_blob(file_entry.id())?;
            let content = String::from_utf8_lossy(blob.content());

            let hours_line = content.lines().next().unwrap();
            let hours = hours_line.split_whitespace().last().unwrap();
            let hours = hours.parse::<f64>().unwrap();

            let current_hours = hours - total_hours;
            total_hours = hours;

            if current_hours <= 0.0 && commit.message().is_some() {
                let message = commit.message().unwrap().trim_end_matches("\n");

                commit_messages.push(message.to_string());
            } else if !commit_messages.is_empty() {
                let current_hours = (current_hours * 100.0).round() / 100.0;
                let seconds = commit.time().seconds();

                let date = chrono::NaiveDateTime::from_timestamp_opt(seconds, 0);
                let date = date.unwrap().format("%Y-%m-%d");

                if has_prehistory && !prehistory_complete {
                    println!("\n{} hours worked up until {}:", current_hours, date);
                    prehistory_complete = true;
                } else {
                    println!("\n{} hours worked on {}:", current_hours, date);
                }

                commit_messages
                    .iter()
                    .for_each(|message| println!("\t- {}", message));
                commit_messages = Vec::new();
            }
        } else {
            let message = commit.message().unwrap().trim_end_matches("\n");

            commit_messages.push(message.to_string());
            has_prehistory = true;
        }
    }

    println!("\nTotal hours: {}", total_hours);

    if has_prehistory {
        println!(
            r#"Note: Work on this repository began before hour tracking was implemented per-push.
			That history has been consolidated into the initial entry."#
        )
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = track_hours(&cli) {
        eprintln!("Error: {}", err);
    }

    fs::remove_dir_all(&cli.dir).unwrap();
}
