use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new application
    New {
        /// Application name
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::New { name }) => build_app(name),
        None => {
            let app_categories = &[
                "React SPA",
                "Eleventy website",
                "Full-stack Golang",
                "Full-stack Next.js",
            ];

            let ui_libraries = &["tailwindcss", "shadcn-ui"];

            let deployment_platforms = &["docker-swarm", "kamal"];

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick your flavor")
                .default(0)
                .items(&app_categories[..])
                .interact()
                .unwrap();

            println!("Enjoy your {}!", app_categories[selection]);

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Optionally pick your UI library")
                .default(0)
                .items(&ui_libraries[..])
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                println!("Enjoy your {}!", app_categories[selection]);
            } else {
                println!("You didn't select anything!");
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick your flavor, hint it might be on the second page")
                .default(0)
                .max_length(2)
                .items(&deployment_platforms[..])
                .interact()
                .unwrap();

            build_app(app_categories[selection]);

            println!(
                "Enjoy your {} with {}!",
                app_categories[selection], ui_libraries[selection]
            );
        }
    }

    // Continued program logic goes here...
}

fn build_app(name: &str) {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .expect("Failed to read user input");

    if confirmation {
        println!("Creating new app: {name}");
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        pb.set_message("Creating new app...");
        thread::sleep(Duration::from_secs(5));
        pb.finish_with_message("Done");

        // fn is_hidden(entry: &walkdir::DirEntry) -> bool {
        //     entry
        //         .file_name()
        //         .to_str()
        //         .map(|s| s.starts_with("."))
        //         .unwrap_or(false)
        // }

        // let walker = walkdir::WalkDir::new("/Users/ariss/Repository").into_iter();
        // for entry in walker.filter_entry(|e| !is_hidden(e)) {
        //     let entry = entry.unwrap();
        //     println!("{}", entry.path().display());
        // }
    } else {
        println!("Sayonara!!!");
    }
}
