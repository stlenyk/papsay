use clap::Parser;
use clap_stdin::MaybeStdin;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "-")]
    message: MaybeStdin<String>,
}

fn pappify(s: &str) -> String {
    let message = s.replace('\t', "    ");

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
    let pap_text = include_str!("../papjesz.pap");

    format!("{}\n{}", message_text, pap_text)
}

fn main() {
    let args = Args::parse();
    let message = pappify(&args.message);
    println!("{}", message);
}
