use image::{open, imageops, DynamicImage, GenericImageView};

fn main() {
    // load the image
    let img: DynamicImage = open("images/cat.png").unwrap();

    // resize the image to a smaller size
    let new_width = 80;
    let new_height = 80;
    let img = img.resize(new_width, new_height, imageops::FilterType::Nearest);

    // ascii characters to use for different intensities
    let ascii_map = [
        ' ', '.', ',', '-', '~', ':', ';', '=', '+', '*', '#', '%', '@', '&', '8', 'W'
    ];

    // get image dimensions
    let (width, height) = img.dimensions();
    println!("Width: {}, Height: {}", width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;

            let brightness = (r + g + b) / 3;
            let index = (brightness as usize * (ascii_map.len() - 1)) / 255;
            
            print!("{}", ascii_map[index]);
        }
        println!();
    }
}
