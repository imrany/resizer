use image::imageops::FilterType;
use image::DynamicImage;
use std::{
    fs,
    path::Path
};

fn main() {
    // Load the image from a file
    let image_path=Path::new("./icon-256x256.png");
    let img = image::open(&image_path);
    match img{
        Ok(v)=>{
            println!("Image opened",);
            // Resize dimensions
            let new_width = 800;
            let new_height = 700;

            // Resize the image
            let resized_img = resize_image(&v, new_width, new_height);

            let create_resized_image_folder=fs::create_dir("./resized");
            match create_resized_image_folder{
                Ok(_)=>{
                    // Save the resized image to a file
                    match resized_img.save(format!("./resized/{}-{new_width}x{new_height}.{}",image_path.file_stem().unwrap().to_str().unwrap(),image_path.extension().unwrap().to_str().unwrap())){
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
