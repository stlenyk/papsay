use std::{
    fs,
    io::{self, Read},
    sync::LazyLock,
};

use atty::Stream;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use rand::Rng;
use rand_distr::Distribution;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
struct Papież(String);

impl std::str::FromStr for Papież {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Self(include_str!("../papieże/ascii.pap").to_owned())),
            "utf8" => Ok(Self(include_str!("../papieże/utf8.pap").to_owned())),
            s => Ok(Self(fs::read_to_string(s).map_err(|e| e.to_string())?)),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    message: Option<Vec<String>>,

    /// One of: ascii, utf8 or a path to a file containing a custom papież
    #[arg(short, long, default_value = "utf8")]
    papież: Papież,

    /// Generate shell completions
    #[arg(
        short,
        long,
        long_help = "Generate shell completions
        Example:
        $ papsay --completions zsh > _papsay
        $ sudo mv _papsay /usr/local/share/zsh/site-functions"
    )]
    completions: Option<Shell>,
}

/// Adds a border around the message and appends the papież at the bottom
fn pappify(message: &str, papież: &Papież) -> String {
    let message = message.replace('\t', "    ");

    let message_lines = textwrap::wrap(&message, 40);
    let lines_graphemes: Vec<Vec<_>> = message_lines
        .iter()
        .map(|s| s.graphemes(true).collect())
        .collect();
    let n_cols = lines_graphemes.iter().map(|s| s.len()).max().unwrap_or(0);

    let message_text = match lines_graphemes.len() {
        0 => "< >".to_owned(),

        1 => format!("< {} >", lines_graphemes[0].concat()),

        _ => {
            let first_line = lines_graphemes.first().unwrap();
            let first_line = format!(
                "/ {}{} \\",
                first_line.concat(),
                " ".repeat(n_cols - first_line.len())
            );

            let last_line = lines_graphemes.last().unwrap();
            let last_line = format!(
                "\\ {}{} /",
                last_line.concat(),
                " ".repeat(n_cols - last_line.len())
            );

            let middle_lines = lines_graphemes[1..lines_graphemes.len() - 1]
                .iter()
                .map(|s| format!("| {}{} |\n", s.concat(), " ".repeat(n_cols - s.len())))
                .collect::<Vec<_>>()
                .join("");

            format!("{}\n{}{}", first_line, middle_lines, last_line)
        }
    };

    let top_border = format!(" {} ", "_".repeat(n_cols + 2));
    let bot_border = format!(" {} ", "-".repeat(n_cols + 2));
    let message_text = format!("{}\n{}\n{}", top_border, message_text, bot_border);

    format!("{}\n{}", message_text, papież.0)
}

// Credits:
// https://www.reddit.com/r/2137/comments/a5z1k9/wywiad_z_ziarnem_zapisany_w_formie_dramatu/
static ZIARNO_DATABASE: LazyLock<Vec<&str>> = LazyLock::new(|| {
    include_str!("../transkrypcja-bez-didaskaliów.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .collect()
});

fn main() {
    let args = Cli::parse();

    if let Some(shell) = args.completions {
        clap_complete::generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_PKG_NAME"),
            &mut io::stdout(),
        );
        return;
    }

    let message = args.message.map(|m| m.join(" "));

    // Case for when input is being piped
    // E.g. echo "Hello, World!" | papsay
    let message = if !atty::is(Stream::Stdin) {
        let mut message = Vec::new();
        io::stdin().read_to_end(&mut message).unwrap();
        let message = String::from_utf8(message).unwrap();
        let message = message.trim_end();
        message.to_owned()
    } else if let Some(message) = message {
        message
    } else {
        let distribution = rand_distr::Normal::<f32>::new(3.0, 1.0).unwrap();
        let mut rng = rand::rng();
        let n_lines = distribution.sample(&mut rng).round() as usize;
        let idx = rng.random_range(0..ZIARNO_DATABASE.len());

        ZIARNO_DATABASE[idx..(idx + n_lines).min(ZIARNO_DATABASE.len())].join("\n")
    };

    println!("{}", pappify(&message, &args.papież));
}
