use image::imageops::FilterType;
use image::DynamicImage;
use std::{
    fs,
    path::PathBuf
};
use clap::Parser;

/// A image resizing CLI tool
#[derive(Parser, Debug)]
#[command(author="Imrany <imranmat254@gmail.com>", version, about="A image resizing CLI tool.", long_about = None)]
struct Args {
    /// Source image path
    #[arg(short, long)]
    input: PathBuf,

    /// Output folder path
    #[arg(short, long)]
    output: PathBuf,

}

fn main() {
    let args = Args::parse();

    let img = image::open(&args.input);
    match img{
        Ok(v)=>{
            // Resize dimensions
            let square_dimensions:Vec<u32>=vec![16,32,48,72,96,128,144,152,192,384,512,57,60,76,114,120,180];
            for i in 0..square_dimensions.len(){
                let new_width=square_dimensions[i];
                let new_height=square_dimensions[i];
                // Resize the image
                let resized_img = resize_image(&v, new_width, new_height);

                let create_resized_image_folder=fs::create_dir(&args.output);
                match create_resized_image_folder{
                    Ok(_)=>{
                        // Save the resized image to a file
                        match resized_img.save(format!("./{}/{}-{new_width}x{new_height}.{}",&args.output.to_str().unwrap(),args.input.file_stem().unwrap().to_str().unwrap(),args.input.extension().unwrap().to_str().unwrap())){
                            Ok(_)=>{
                                println!("./{}/{}-{new_width}x{new_height}.{}",&args.output.to_str().unwrap(),args.input.file_stem().unwrap().to_str().unwrap(),args.input.extension().unwrap().to_str().unwrap())
                            },
                            Err(e)=>eprintln!("Failed to save resized image: {e}")
                        }
                    },
                    Err(_)=>{
                        //eprintln!("{}",e);
                        // Save the resized image to a file
                        match resized_img.save(format!("./{}/{}-{new_width}x{new_height}.{}",&args.output.to_str().unwrap(),args.input.file_stem().unwrap().to_str().unwrap(),args.input.extension().unwrap().to_str().unwrap())){
                            Ok(_)=>{
                                println!("./{}/{}-{new_width}x{new_height}.{}",&args.output.to_str().unwrap(),args.input.file_stem().unwrap().to_str().unwrap(),args.input.extension().unwrap().to_str().unwrap())
                            },
                            Err(e)=>println!("Failed to save resized image: {e}")
                        }
                    }
                }
            }
        },
        Err(e)=>println!("Failed to open image: {e}")
    }

}

fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}
