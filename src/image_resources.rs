use egui::{ TextureHandle, Context, ColorImage };
use std::path::Path;
use image::ImageReader;

pub struct ImageResources {
    pub board: TextureHandle,

    /*
    // Pawn
    pub sente_fu: TextureHandle,
    pub gote_fu: TextureHandle,

    // Silver
    pub sente_gi: TextureHandle,
    pub gote_gi: TextureHandle,

    // King
    pub sente_gy: TextureHandle,
    pub gote_gy: TextureHandle,

    // Rook
    pub sente_hi: TextureHandle,
    pub gote_hi: TextureHandle,

    // Bishop
    pub sente_ka: TextureHandle,
    pub gote_ka: TextureHandle,

    // Knight
    pub sente_ke: TextureHandle,
    pub gote_ke: TextureHandle,

    // Gold
    pub sente_ki: TextureHandle,
    pub gote_ki: TextureHandle,

    // Lance
    pub sente_ky: TextureHandle,
    pub gote_ky: TextureHandle,

    // Promoted Silver
    // Promoted Knight
    // Promoted Lance
    // Dragon 
    // Promoted Pawn
    // Horse
    */
}

impl ImageResources {
    pub fn new(ctx: &Context) -> Self {
        let board_image = load_image_from_path(Path::new("src/images/boards/painting1.jpg"))
        .unwrap_or_else(|err| {
            // Handle error, maybe use a default image if loading fails
            eprintln!("Error loading image: {}", err);
            egui::ColorImage::default() // or another fallback image
        });

        Self {
            board: ctx.load_texture(
                "board", 
                // expects ColorImage, not Result<ColorImage, ImageError>
                board_image,
                Default::default(),
            ),
        }
    }
}

pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
