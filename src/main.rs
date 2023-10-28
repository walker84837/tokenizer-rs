use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

mod tokens;

#[derive(StructOpt, Debug)]
struct Cli {
    /// File to read the tokens from
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,

    /// Tokenizer mode:
    /// p50k_base: use for code models, 
    /// cl100k_base: use for ChatGPT models, 
    /// p50k_edit: use for edit models, 
    /// r50k_base: use for GPT-3 models
    #[structopt(short = "t", long = "mode")]
    mode: String,

    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    let mut file = File::open(cli.input_file)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let contents: &str = file_contents.as_str();
    
    let mut tokenizer = tokens::Tokenizer::new();
    let tokens: Vec<usize> = tokenizer.get_tokens(cli.mode, contents);
    let len: i32 = tokens.len().try_into().unwrap();

    match cli.verbose {
        true => {
            println!("Token total count: {}", len);
            println!("Tokens' size: {:?}", tokens);
        },

        false => {
            println!("Token total count: {}", len);
        }
    }

    Ok(())
}
