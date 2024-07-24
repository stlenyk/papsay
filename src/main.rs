use std::io::{self, Read};

use atty::Stream;
use clap::Parser;
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
            s => Ok(Self(std::fs::read_to_string(s).map_err(|e| e.to_string())?)),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    message: Option<String>,
    #[arg(short, long, default_value = "utf8")]
    /// One of: ascii, utf8, or a path to a file containing a custom papież.
    papież: Papież,
}

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

fn main() {
    let args = Args::parse();
    let message = args.message;

    let message = if !atty::is(Stream::Stdin) {
        let mut message = Vec::new();
        io::stdin().read_to_end(&mut message).unwrap();
        let message = String::from_utf8(message).unwrap();
        let message = message.trim_end();
        message.to_owned()
    } else if let Some(message) = message {
        message
    } else {
        let lines_database = include_str!("../transkrypcja-bez-didaskaliów.txt")
            .lines()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let distribution = rand_distr::Normal::<f32>::new(3.0, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let n_lines = distribution.sample(&mut rng).round() as usize;
        let idx = rng.gen_range(0..lines_database.len());

        lines_database[idx..(idx + n_lines).min(lines_database.len())].join("\n")
    };

    println!("{}", pappify(&message, &args.papież));
}
