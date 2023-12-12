use qrcode::{render::unicode, QrCode};
use std::env;
use std::io::{self, BufRead};

fn main() {
    let mut text = get_args_text();

    if text.trim().is_empty() {
        eprintln!("No arguments recieved, reading stdin.");
        text = get_stdin_text();
        if text.trim().is_empty() {
            eprintln!("Error: No input was found.\nEither pass argument(s) to the program, or send something via stdin.");
            std::process::exit(1);
        }
    }

    print(text);
}

fn get_args_text() -> String {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    return args.join(" ");
}

fn get_stdin_text() -> String {
    let mut result_string = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        result_string.push_str(&line.unwrap().trim());
        result_string.push_str("\n");
    }

    result_string = strip_ansi_escapes::strip_str(result_string);

    return result_string.trim().to_string();
}

fn print(text: String) {
    let code = QrCode::new(text).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
