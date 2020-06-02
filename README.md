# URL Summarizer

![CI](https://github.com/bradyjoslin/summarize-url/workflows/CI/badge.svg)

## Usage

Summarizes the content of a URL using [nlp/SummarizeURL/0.1.4](https://algorithmia.com/algorithms/nlp/SummarizeURL) from [Algorithmia](https://algorithmia.com/).

```text
summarize-url 0.0.1
Summarize the content of a web page

USAGE:
    summarize-url [FLAGS] [OPTIONS] <url> <api-key>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose output

OPTIONS:
    -l, --length <length>    Length of output by number of sentences [default: 3]

ARGS:
    <url>        URL to summarize
    <api-key>    Algorithmia API Key from https://algorithmia.com/ [env: ALGO_KEY]
```

## To install

```bash
cargo install --path .
```
