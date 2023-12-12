use qrcode::{render::unicode, QrCode};
use std::env;

fn main() {
    print(get_text());
}

fn get_text() -> String {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.is_empty() {
        println!("Error: No text provided!");
        std::process::exit(1);
    }

    return args.join(" ");
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
