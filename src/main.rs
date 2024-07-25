use image::imageops::FilterType;
use image::DynamicImage;

fn main() {
    // Load the image from a file
    let img = image::open("./icon-256x256.png");
    match img{
        Ok(v)=>{
            println!("Image opened");
            // Resize dimensions
            let new_width = 800;
            let new_height = 700;

            // Resize the image
            let resized_img = resize_image(&v, new_width, new_height);

            // Save the resized image to a file
            match resized_img.save(format!("./resized_icon-{new_width}x{new_height}.png")){
                Ok(_)=>{
                    println!("Image resized")
                },
                Err(e)=>println!("Failed to save resized image: {e}")
            }
        },
        Err(e)=>println!("Failed to open image: {e}")
    }

}

fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}
