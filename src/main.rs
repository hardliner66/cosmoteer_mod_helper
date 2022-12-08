use std::path::PathBuf;

use clap::Parser;
use question::Answer;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    /// If set, doesn't ask for confirmation
    no_confirm: bool,
    /// The file to open
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(file) = args.file {
        let text = std::fs::read_to_string(file)?;

        let links: Vec<String> = ron::from_str(&text)?;

        let choice = if args.no_confirm {
            question::Question::new(&format!(
                "This will open {} new mods in your browser. Do you want to continue?",
                links.len()
            ))
            .confirm()
        } else {
            Answer::YES
        };

        if choice == Answer::YES {
            for link in links {
                open::that(link)?;
            }
        }
    } else {
        let user_dirs = directories::UserDirs::new().unwrap();

        let mut path = user_dirs.home_dir().to_path_buf();
        path.push("Saved Games/Cosmoteer/76561198095968405/settings.rules");
        let text = std::fs::read_to_string(path)?;
        if let Some(pos) = text.find("EnabledMods") {
            let new_text = &text[pos..];
            let pos = new_text.find('[').unwrap();
            let new_text = &new_text[pos + 1..];
            let pos = new_text.find(']').unwrap();
            let mods_text = &new_text[..pos];
            let mods = mods_text
                .trim()
                .lines()
                .flat_map(|l| {
                    let without_quotes = l.replace('"', "");
                    let split = without_quotes.split('/');
                    split.last().map(|c| {
                        format!(
                            "https://steamcommunity.com/sharedfiles/filedetails/?id={}",
                            c
                        )
                    })
                })
                .collect::<Vec<_>>();
            std::fs::write("./cosmoteer-mods.ron", ron::to_string(&mods)?)?;
        } else {
            eprintln!("No mods enabled!");
        }
    }
    Ok(())
}
