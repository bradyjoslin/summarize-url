use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Summarize the content of a web page
#[derive(StructOpt, Debug)]
#[structopt(global_setting(AppSettings::ColoredHelp), name = "summarize-url")]
pub struct App {
    /// URL to summarize.
    pub url: String,

    /// Algorithmia API Key from https://algorithmia.com/
    #[structopt(env = "ALGO_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Length of output by number of sentences.
    #[structopt(short, long, default_value = "3")]
    pub length: i32,

    /// Verbose output.
    #[structopt(short, long)]
    pub verbose: bool,
}
