use image::imageops::FilterType;
use image::DynamicImage;
use std::{
    fs,
    path::Path
};
use clap::Parser;

/// A image resizing CLI tool
#[derive(Parser, Debug)]
#[command(author="Imrany <imranmat254@gmail.com>", version, about="A image resizing CLI tool.", long_about = None)]
struct Args {
    /// Source image path
    #[arg(short, long)]
    input: Path,

    /// Output folder path
    #[arg(short, long)]
    output: Path,

}

fn main() {
    let args = Args::parse();

    let img = image::open(&args.input);
    match img{
        Ok(v)=>{
            println!("Image opened",);
            // Resize dimensions
            let new_width = 800;
            let new_height = 700;

            // Resize the image
            let resized_img = resize_image(&v, new_width, new_height);

            let create_resized_image_folder=fs::create_dir(&arg.output);
            match create_resized_image_folder{
                Ok(_)=>{
                    // Save the resized image to a file
                    match resized_img.save(format!("./{}/{}-{new_width}x{new_height}.{}",&args.output,image_path.file_stem().unwrap().to_str().unwrap(),image_path.extension().unwrap().to_str().unwrap())){
                        Ok(_)=>{
                            println!("Image resized")
                        },
                        Err(e)=>println!("Failed to save resized image: {e}")
                    }
                },
                Err(_)=>eprintln!("Cannot create a folder for the resized images")
            }
        },
        Err(e)=>println!("Failed to open image: {e}")
    }

}

fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}
