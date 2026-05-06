use curvy_wav::*;

fn main() {
    let wav = WavStream::from_file("curvy_wav/samples/Alesis-Fusion-Shakuhachi-C5.wav").unwrap();
    println!("{:#?}", wav);
}
