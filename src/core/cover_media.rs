use crate::core::error::StegoCoverMediaError;

/// Defines the interface for cover media that we intend to encode/decode. Implementors must provide
/// methods for reading bytes, writing back bytes and creating a clone with new bytes.
pub trait StegoCoverMedia {
    /// Reads bytes for a given cover media
    ///
    /// # Returns
    ///
    /// A slice of bytes
    fn read_bytes(&self) -> &[u8];

    /// Reads bytes for a given cover media
    ///
    /// # Arguments
    ///
    /// - `new_bytes` - The bytes you wish to write to this struct
    ///
    /// # Returns
    ///
    /// A result of either Ok or a StegoCoverMediaError
    fn write_bytes(&mut self, new_bytes: &[u8]) -> Result<(), StegoCoverMediaError>;

    /// Reads bytes for a given cover media
    ///
    /// # Arguments
    ///
    /// - `new_bytes` - The bytes you wish to write to the cloned struct
    ///
    /// # Returns
    ///
    /// A result of either cover media or a `StegoCoverMediaError`
    fn clone_with_bytes(&self, new_bytes: &[u8]) -> Result<Box<dyn StegoCoverMedia>, StegoCoverMediaError>;
}
