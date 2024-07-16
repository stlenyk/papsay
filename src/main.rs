use clap::Parser;

#[derive(Parser)]
struct Args {
    message: String,
}

fn pappify(s: &str) -> String {
    let message = s.replace('\t', "    ");

    let message_lines = textwrap::wrap(&message, 40);
    let n_cols = message_lines.iter().map(|s| s.len()).max().unwrap_or(0);

    let message_text = match message_lines.len() {
        0 => "< >".to_owned(),

        1 => format!("< {} >", message_lines[0]),

        _ => {
            let first_line = message_lines.first().unwrap();
            let first_line = format!(
                "/ {}{} \\",
                first_line,
                " ".repeat(n_cols - first_line.len())
            );

            let last_line = message_lines.last().unwrap();
            let last_line = format!("\\ {}{} /", last_line, " ".repeat(n_cols - last_line.len()));

            let middle_lines = message_lines[1..message_lines.len() - 1]
                .iter()
                .map(|s| format!("| {}{} |\n", s, " ".repeat(n_cols - s.len())))
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
