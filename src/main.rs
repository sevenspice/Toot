use std::path::PathBuf;
use anyhow::{Result, Context, anyhow};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[clap(
    author  = "@BananaPepperTK", 
    version = "1.0.0", 
    name    = "Toot",
    about   = "Tooting."
)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name="FILE")]
    setting_filepath: Option<PathBuf>,
    #[clap(short, long)]
    toot: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize
}

#[derive(Deserialize)]
struct Mastodon {
    hostname: String,
    access_token: String
}

fn run(settings: Mastodon, toot_text: &str, verbose: usize) -> Result<u16> {
    let params = [("status", toot_text)];
    let client = reqwest::blocking::Client::new();
    let response = client.post(format!("https://{}/api/v1/statuses", settings.hostname))
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("Authorization", format!("Bearer {}", settings.access_token))
        .form(&params)
        .send()?;

    let status: reqwest::StatusCode = response.status();
    if verbose == 1 {
        println!("{:?}", response.status());
        println!("{:?}", response.headers());
        println!("{:?}", response.text());
    }

    if status.as_u16() != 200 {
        return Err(anyhow!("Request failed."));
    }

    Ok(status.as_u16())
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(file_path) = cli.setting_filepath.as_deref() {
        let settings_string = std::fs::read_to_string(file_path).context(format!("Failed to open setting file: {:?}", file_path))?;
        let settings: Mastodon = toml::from_str(&settings_string).context(format!("Failed to parse toml file \n {}", settings_string))?;
        match run(settings, &cli.toot, cli.verbose) {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{:?}", error)
        }
    }

    Ok(())
}
