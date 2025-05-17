use crate::core::cover_media::CoverMedia;
use crate::core::error::{StegoStrategyError};

/// Defines the interface for steganography strategies. Implementors must provide methods for encoding
/// and decoding messages into/from a `StegoCoverMedia` implementation.
pub trait StegoStrategy {
    /// Encodes a message into the given cover and returns the updated cover media.
    ///
    /// # Arguments
    ///
    /// * `message` - The message you want encoding into the media.
    /// * `media` - The media you wish to be encoded.
    ///
    /// # Returns
    ///
    /// A `Result` containing ok or an error.
    fn encode(&self, message: &str, media: &mut dyn CoverMedia) -> Result<(), StegoStrategyError>;

    /// Decodes a message from the given cover and returns it as a string.
    ///
    /// # Arguments
    ///
    /// * `media` - The media you wish to be decoded.
    ///
    /// # Returns
    ///
    /// A `Result` containing the message as a string or an error.
    fn decode(&self, media: &dyn CoverMedia) -> Result<String, StegoStrategyError>;
}