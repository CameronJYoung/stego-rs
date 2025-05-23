use crate::core::error::CoverMediaError;

/// Defines the interface for cover media that we intend to encode/decode. Implementors must provide
/// methods for reading bytes, writing back bytes and creating a clone with new bytes.
pub trait CoverMedia {
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
    /// A result of either Ok or a CoverMediaError
    fn write_bytes(&mut self, new_bytes: &[u8]) -> Result<(), CoverMediaError>;

    /// Reads bytes for a given cover media
    ///
    /// # Arguments
    ///
    /// - `new_bytes` - The bytes you wish to write to the cloned struct
    ///
    /// # Returns
    ///
    /// A result of either cover media or a `CoverMediaError`
    fn clone_with_bytes(&self, new_bytes: &[u8]) -> Result<Box<dyn CoverMedia>, CoverMediaError>;
}
