use image::RgbaImage;
use crate::core::cover_media::CoverMedia;
use crate::core::error::StegoCoverMediaError;

pub struct PngCoverMedia {
    data: Vec<u8>,
    height: u32,
    width: u32
}

impl PngCoverMedia {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            data: Vec::new(),
            height,
            width,
        }
    }

    pub fn open(path: &str) -> Result<Self, StegoCoverMediaError> {
        // Get the PNG using the image library
        let image = match image::open(path) {
            Ok(image) => image,
            Err(e) => {
                return Err(StegoCoverMediaError::BadFile(e.to_string()))
            }
        };

        // Extract RGB data
        let rgb_img = image.to_rgba8();

        // Convert to a nicer type
        let rgb_data_bytes: Vec<u8> = rgb_img.pixels().flat_map(|p| p.0).collect();

        Ok(Self {
            data: rgb_data_bytes,
            height: image.height(),
            width: image.width()
        })
    }

    pub fn save(&self, output_path: &str) -> Result<(), StegoCoverMediaError> {
        let new_image: RgbaImage = match RgbaImage::from_raw(self.width, self.height, self.data.clone()) {
            Some(i) => i,
            None => return Err(StegoCoverMediaError::GenerateFileFailure)
        };

        match new_image.save(output_path) {
            Ok(_) => Ok(()),
            Err(_) => return Err(StegoCoverMediaError::WriteFileFailure)
        }
    }
}

impl CoverMedia for PngCoverMedia {
    fn read_bytes(&self) -> &[u8] {
        &self.data
    }

    fn write_bytes(&mut self, new_bytes: &[u8]) -> Result<(), StegoCoverMediaError> {
        self.data.clear();
        self.data.extend_from_slice(new_bytes);
        Ok(())
    }

    fn clone_with_bytes(&self, new_bytes: &[u8]) -> Result<Box<dyn CoverMedia>, StegoCoverMediaError> {
        Ok(Box::new(Self {
            data: new_bytes.to_vec(),
            height: self.height,
            width: self.width
        }))
    }
}