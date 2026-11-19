use std::println;

use nu_ansi_term::Color;
use nu_ansi_term::Style;

// fn main() {
//     println!(
//         "This iss {} in color, {} in color and {} in color",
//         Color::Red.paint("red"),
//         Color::Blue.paint("blue"),
//         Color::Green.paint("green"),
//     );
// }

// fn main() {
//     println!(
//         "{} and this is not",
//         Style::new().bold().paint("This is Bold")
//     )
// }

fn main() {
    println!(
        "{}, {} and {}",
        Color::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Color::Yellow.bold().paint("This is bold new colored"),
    )
}
