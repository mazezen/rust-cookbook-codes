pub fn compress_gzip() -> Result<(), std::io::Error> {
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::fs::File;
    use std::io;

    let mut input = File::open("input.txt")?;
    let output = File::create("input.txt.gz")?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    let _ = io::copy(&mut input, &mut encoder);
    let _ = encoder.finish();
    Ok(())
}

pub fn decompress_gzip() -> Result<(), std::io::Error> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io;

    let input = File::open("input.txt.gz")?;
    let mut decoder = GzDecoder::new(input);
    let mut output = File::create("input2.txt")?;
    let _ = io::copy(&mut decoder, &mut output);
    Ok(())
}

fn main() {
    let _ = compress_gzip();
    let _ = decompress_gzip();
}
