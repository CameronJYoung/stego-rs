use hound::{WavReader, WavSpec, WavWriter};
use crate::core::cover_media::CoverMedia;
use crate::core::error::CoverMediaError;

pub struct WavPcmCoverMedia {
    data: Vec<u8>,
    wav_spec: WavSpec
}

impl WavPcmCoverMedia {
    pub fn new(wav_spec: WavSpec) -> Self {
        Self {
            data: Vec::new(),
            wav_spec
        }
    }

    pub fn open(path: &str) -> Result<Self, CoverMediaError> {
        // Get the wav reader using the hound crate
        let mut wav_reader = match WavReader::open(path) {
            Ok(w) => w,
            Err(e) => return Err(CoverMediaError::BadFile(e.to_string()))
        };

        // Get PCM data in bytes
        let mut pcm_bytes: Vec<u8> = Vec::new();
        for sample in wav_reader.samples::<i32>() { // TODO: This will fail if the supplied audio is not 32 bits. When we implement file type specific config we should make this configurable
            let sample = match sample {
                Ok(s) => s,
                Err(e) => return Err(CoverMediaError::BadFile(e.to_string()))
            };

            pcm_bytes.extend(&sample.to_le_bytes());
        };

        Ok(Self {
            data: pcm_bytes,
            wav_spec: wav_reader.spec()
        })
    }

    pub fn save(&self, output_path: &str) -> Result<(), CoverMediaError> {
        // Create a hound writer
        let mut new_wav_writer = match WavWriter::create(output_path, self.wav_spec) {
            Ok(w) => w,
            Err(_) => return Err(CoverMediaError::WriteFileFailure)
        };

        // Chunk the final encoded data into samples and add it to the writer
        for chunk in self.data.chunks_exact(4) {
            let bytes: [u8; 4] = match chunk.try_into() {
                Ok(t) => t,
                Err(_) => return Err(CoverMediaError::GenerateFileFailure)
            };

            let sample = i32::from_le_bytes(bytes);

            if let Err(_) = new_wav_writer.write_sample(sample) {
                return Err(CoverMediaError::GenerateFileFailure);
            }
        }

        // Write the file
        if let Err(_) = new_wav_writer.finalize() {
            return Err(CoverMediaError::GenerateFileFailure);
        }

        Ok(())
    }
}

impl CoverMedia for WavPcmCoverMedia {
    fn read_bytes(&self) -> &[u8] {
        &self.data
    }

    fn write_bytes(&mut self, new_bytes: &[u8]) -> Result<(), CoverMediaError> {
        self.data.clear();
        self.data.extend_from_slice(new_bytes);
        Ok(())
    }

    fn clone_with_bytes(&self, new_bytes: &[u8]) -> Result<Box<dyn CoverMedia>, CoverMediaError> {
        Ok(Box::new(Self {
            data: new_bytes.to_vec(),
            wav_spec: self.wav_spec
        }))
    }
}