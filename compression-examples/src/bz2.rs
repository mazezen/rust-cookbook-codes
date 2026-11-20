use std::io;

pub fn compress_bz2() -> io::Result<()> {
    use bzip2::Compression;
    use bzip2::read::BzEncoder;
    use std::fs::File;

    let mut input = File::open("input.txt")?;
    let output = File::create("input.txt.bz2")?;
    let mut encoder = BzEncoder::new(output, Compression::best());
    let _ = io::copy(&mut input, &mut encoder)?;

    Ok(())
}

pub fn decompress_bz2() -> io::Result<()> {
    use bzip2::read::BzDecoder;
    use std::fs::File;

    let input = File::open("input.txt.bz2")?;
    let mut decoder = BzDecoder::new(input);
    let mut output = File::create("input.txt")?;
    let _ = io::copy(&mut decoder, &mut output);
    Ok(())
}

fn main() {
    let _ = compress_bz2();
    let _ = decompress_bz2();
}
