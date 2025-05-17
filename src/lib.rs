pub mod core;
pub mod strategies;
pub mod cover_media;

#[cfg(test)]
mod tests {
    use crate::core::strategy::Strategy;
    use crate::cover_media::wav_pcm::WavPcmCoverMedia;
    use crate::strategies::lsb::LsbStrategy;

    #[test]
    fn tmp() {
        let mut wav_media = WavPcmCoverMedia::open("./example.wav").unwrap();
        let lsb_strategy = LsbStrategy::new();

        lsb_strategy.encode("Hellooooo2", &mut wav_media).unwrap();

        wav_media.save("./out.wav").unwrap()
    }

    #[test]
    fn tmp_2() {
        let mut wav_media = WavPcmCoverMedia::open("./out.wav").unwrap();
        let lsb_strategy = LsbStrategy::new();

        println!("{}", lsb_strategy.decode(&mut wav_media).unwrap());
    }
}