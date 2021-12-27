use std::path::PathBuf;
use std::fmt;

use structopt::StructOpt;
use serde::Deserialize;
use futures::{future};
use console::Term;

mod config;

const HEAD: &str = "
Found the following expansions:
";

/// Find acronym meaning.
#[derive(Debug, StructOpt)]
#[structopt(name = "args", about = "Provide an acronym to search for")]
struct Cli {
    /// The acronym to search for
    acronym: String,

    /// Context to search in
    #[structopt(short, long)]
    context: Option<String>,

    /// Config file location
    #[structopt(long)]
    config: Option<String>,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

/// Definition of the acronym data representation.
///
/// Derived from NASA https://github.com/nasa/NASA-Acronyms
#[derive(Deserialize, Debug)]
struct Acronym {
    abbreviation: String,
    acronym_id: Option<u32>,
    expansion: String,
}

/// Search results per context
#[derive(Debug)]
struct SearchResult {
    context: String,
    value: Option<Acronym>,
}

impl fmt::Display for SearchResult {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {


        if let Some(acroynym) = &self.value {
            formatter.write_fmt(format_args!("✅ Context {}: ", &self.context))?;
            formatter.write_str(&acroynym.expansion)?;
        } else {
            formatter.write_fmt(format_args!("❌ Context {}: ", &self.context))?;
            formatter.write_str(" no results")?;
        }

        formatter.write_str("\n")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let needle = &args.acronym;
    let config = crate::config::SlangConfig::new(args.config.map(PathBuf::from))?;
    let urls = config.get_sources(args.context)?;

    // Async concurrent lookup against all configured resources. Assume sources
    // are limited and join all the futures without buffering.
    // TODO: add caching of source urls and move to separate module
    let results = future::join_all(
        urls.into_iter().map(|(context, url)| {
            async move {
                let response = reqwest::get(url).await?;
                let body = response.json::<Vec<Acronym>>().await?;

                // Consume the response JSON and find the acronym.
                let result = SearchResult {
                    value: body.into_iter().find(|acronym| &acronym.abbreviation == needle),
                    context
                };

                // Explicit type to enable error propagation operator use.
                Ok::<SearchResult, Box<dyn std::error::Error>>(result)
            }
        })
    ).await;

    // Write to stdout
    let stdout = Term::stdout();
    stdout.write_line(HEAD)?;

    stdout.write_line(
        &results
            .into_iter()
            .fold("".to_owned(), |mut acc: String, result: Result<SearchResult, Box<dyn std::error::Error>>| {
                let found = match result {
                    Ok(found) => found.to_string(),
                    Err(err) => err.to_string()
                };

                acc += &found;
                acc
            })
    )?;

    stdout.flush()?;

    Ok(())
}