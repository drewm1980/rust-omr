
/// An image data container used internally.
/// Images are 8-bit single channel for now.
pub struct Image {
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
    /// The buffor containing the image data.
    pub pixels: Vec<u8>,
}

