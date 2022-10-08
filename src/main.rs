use ansi_term::{Colour::RGB, Style};
use image::{DynamicImage, ImageError, Pixel};

fn usage(bin_name: &String) {
    println!("usage: {} IMAGE_PATH [IMAGE_PATH ...]", bin_name);
    std::process::exit(1);
}

fn print_color(text: &String, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    print!(
        "{}",
        Style::new()
            .on(RGB(bg.0, bg.1, bg.2))
            .fg(RGB(fg.0, fg.1, fg.2))
            .paint(text)
    );
}

fn preprocess_image(path: &String, width: u16, height: u16) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;
    Ok(img.thumbnail(width.into(), height.into()))
}

fn draw_img(path: &String, w: u16, h: u16) {
    let img = match preprocess_image(path, w, h) {
        Ok(result) => result,
        Err(error) => {
            println!("Failed to preprocess image: {}", error);
            std::process::exit(1);
        }
    };

    let rgb = match img.as_rgb8() {
        Some(result) => result,
        None => {
            println!("Failed to convert image {} to RGB", path);
            std::process::exit(1);
        }
    };

    // TODO: allow painting multiple images in a grid
    let rows = rgb.rows();

    let even = rows.clone().step_by(2);
    let odd = rows.clone().skip(1).step_by(2);

    for (row1, row2) in even.zip(odd) {
        for (top, bot) in row1.zip(row2) {
            let bg = top.to_rgb();
            let fg = bot.to_rgb();
            print_color(
                &"\u{25AC}".to_string(),
                (fg[0], fg[1], fg[2]),
                (bg[0], bg[1], bg[2]),
            );
        }
        println!("");
    }
    // TODO: if #rows is odd, we currently skip the last row
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
    }
    let paths = &args[1..];

    let (term_w, term_h) = match termion::terminal_size() {
        Ok(result) => result,
        Err(error) => {
            println!("Failed to get terminal size: {}", error);
            std::process::exit(1);
        }
    };

    // Image should be viewable within current window, and allow for one extra row for the command
    // prompt. Multiply height with two, since we represent 2 pixels per character (foreground /
    // background).
    let w = term_w;
    let h = term_h * 2 - 1;

    for path in paths {
        draw_img(path, w, h);
    }
}
