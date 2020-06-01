mod app;
mod errors;

use algorithmia::Algorithmia;
use errors::{AppResult, Error};
use structopt::StructOpt;

fn main() -> AppResult<()> {
    let app = app::App::from_args();

    let input = &app.url;
    let api_key = &app.api_key;
    let length = &app.length;

    let summary = summarize_url(api_key, input, length)?;

    match app {
        app::App { verbose: true, .. } => println!("Summary of {}\n{}", input, summary),
        _ => println!("\n{}", summary),
    }
    Ok(())
}

fn summarize_url(api_key: &str, url: &str, length: &i32) -> AppResult<String> {
    let client = Algorithmia::client(api_key);
    let algo = client.algo("nlp/SummarizeURL/0.1.4");
    let _l = length.to_owned().to_string();
    let input_cfg = &vec![url, &_l];
    match algo.pipe(input_cfg) {
        Ok(response) => {
            let summary = response.to_string();
            if summary.contains("does not exist") {
                Err(Error::BadUrl(summary))
            } else {
                Ok(summary)
            }
        }
        Err(_) => Err(Error::AlgoAPI),
    }
}
