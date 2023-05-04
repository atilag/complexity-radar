mod complexity;
mod report;

use anyhow::Result;
use clap::Parser;
use complexity_radar::TopChangedFilesExt;
use octocrab::Octocrab;
use report::print_report;

#[derive(Parser, Debug)]
#[clap(name = "complexity-radar")]
#[clap(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct CommandLineArguments {
    #[clap(short = 'b', long = "base-url")]
    pub base_url: Option<String>,

    #[clap(short = 'u', long = "github-user")]
    pub github_user: String,

    #[clap(short = 'r', long = "github-repo")]
    pub github_repo: String,

    #[clap(short = 'n', long = "num-rows", default_value_t = 5)]
    pub num_rows: usize,

    #[clap(short = 't', long = "token")]
    pub token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = CommandLineArguments::parse();

    let token = args.token.map_or(
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required"),
        |token| token,
    );

    let octocrab = match args.base_url {
        Some(base_url) => Octocrab::builder()
            .base_uri(base_url)?
            .personal_token(token)
            .build()?,
        _ => Octocrab::builder().personal_token(token).build()?,
    };

    let top_files = octocrab
        .get_top_changed_files(args.num_rows, &args.github_user, &args.github_repo)
        .await?;

    print_report(
        &top_files
            .into_iter()
            .map(|(code_file, change_count)| (code_file.filename, change_count))
            .collect(),
    );

    Ok(())
}
