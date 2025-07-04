use image::{DynamicImage, GenericImageView, GrayImage, ImageReader};
use rust_lapper::{Interval, Lapper};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use image::imageops::FilterType;
use terminal_size::{Width, Height, terminal_size};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let filename = get_user_input("Enter image path to file:");

    let ascii_lume_interval = create_ascii_interval();
    let lapper = Lapper::new(ascii_lume_interval);

    let img = ImageReader::open(filename)?.decode()?;

    let target_width = get_terminal_width();
    let (original_width, original_height) = img.dimensions();

    // Calculate proportional height
    let target_height = (original_height * target_width) / original_width;
    // Resize to target dimensions first
    let small_img = image::imageops::resize(&img, target_width, target_height, FilterType::Lanczos3);

    let aspect_ratio = 2.0;
    let final_height = (target_height as f32 / aspect_ratio) as u32;

    let resized_img = image::imageops::resize(&small_img, target_width, final_height, FilterType::Lanczos3);

    let luma: GrayImage = DynamicImage::ImageRgba8(resized_img).into_luma8();

    let path = "output-image.txt";
    let f = File::create(path).expect("unable to create file");
    let mut buffer_file = BufWriter::new(f);
    
    let mut prev_y: u32 = 0;
    for (_x, y, pixel) in luma.enumerate_pixels() {
        let luminance: u8 = pixel[0];
        let ascii_char = lapper.find(luminance as u32, luminance as u32 + 1)
            .next();

        if let Some(found_interval) = ascii_char {
            if prev_y != y {
                write!(buffer_file, "\n").expect("unable to write");
                println!();
                prev_y = y;
            }
            write!(buffer_file, "{}", found_interval.val).expect("unable to write");
            print!("{}", found_interval.val);
        }
    }
    
    println!();

    get_user_input("Press Enter to exit...");

    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_terminal_width() -> u32 {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        w as u32
    } else {
        120
    }
}

fn create_ascii_interval() -> Vec<Interval<u32, char>> {
    let intervals: Vec<Interval<u32, char>> = vec![
        Interval { start: 0, stop: 8, val: '@' },
        Interval { start: 8, stop: 16, val: '#' },
        Interval { start: 16, stop: 24, val: '&' },
        Interval { start: 24, stop: 32, val: '$' },
        Interval { start: 32, stop: 40, val: '%' },
        Interval { start: 40, stop: 48, val: '*' },
        Interval { start: 48, stop: 56, val: '?' },
        Interval { start: 56, stop: 64, val: '{' },
        Interval { start: 64, stop: 72, val: '}' },
        Interval { start: 72, stop: 80, val: '(' },
        Interval { start: 80, stop: 88, val: ')' },
        Interval { start: 88, stop: 96, val: '[' },
        Interval { start: 96, stop: 104, val: ']' },
        Interval { start: 104, stop: 112, val: '+' },
        Interval { start: 112, stop: 120, val: '=' },
        Interval { start: 120, stop: 128, val: '~' },
        Interval { start: 128, stop: 136, val: '^' },
        Interval { start: 136, stop: 144, val: '>' },
        Interval { start: 144, stop: 152, val: '<' },
        Interval { start: 152, stop: 160, val: '_' },
        Interval { start: 160, stop: 168, val: '|' },
        Interval { start: 168, stop: 176, val: '/' },
        Interval { start: 176, stop: 184, val: '\\' },
        Interval { start: 184, stop: 192, val: ';' },
        Interval { start: 192, stop: 200, val: '!' },
        Interval { start: 200, stop: 207, val: ':' },
        Interval { start: 207, stop: 214, val: '-' },
        Interval { start: 214, stop: 221, val: '"' },
        Interval { start: 221, stop: 228, val: ',' },
        Interval { start: 228, stop: 235, val: '\'' },
        Interval { start: 235, stop: 242, val: '.' },
        Interval { start: 242, stop: 249, val: '`' },
        Interval { start: 249, stop: 256, val: ' ' },
    ];

    intervals
}