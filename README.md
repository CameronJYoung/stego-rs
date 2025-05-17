# stego-rs

# Usage

## Encoding

```rust
let mut wav_media = WavPcmCoverMedia::open("./example.wav").unwrap();
let lsb_strategy = LsbStrategy::new();

lsb_strategy.encode("Hellooooo2", &mut wav_media).unwrap();

wav_media.save("./out.wav").unwrap()
```

## Decoding

```rust
let mut wav_media = WavPcmCoverMedia::open("./out.wav").unwrap();
let lsb_strategy = LsbStrategy::new();

println!("{}", lsb_strategy.decode(&mut wav_media).unwrap());
```