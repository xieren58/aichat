use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List all roles
    #[clap(short = 'L', long)]
    pub list_roles: bool,
    /// Specify the role that the AI will play
    #[clap(short, long)]
    pub role: Option<String>,
    /// Input text, if no input text, enter interactive mode
    text: Vec<String>,
}

impl Cli {
    pub fn text(&self) -> Option<String> {
        let text = self
            .text
            .iter()
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        if text.is_empty() {
            return None;
        }
        Some(text)
    }
}
