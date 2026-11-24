use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<Digest, Error> {
    let mut buf_reader = BufReader::new(File::open(filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn is_iso(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("iso"))
}

fn main() -> Result<(), Error> {
    let paths: Vec<PathBuf> = WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
        .map(|e| e.into_path())
        .collect();

    let digests = paths
        .par_iter()
        .map(|path| compute_digest(path).map(|digest| (digest, path)))
        .collect::<Result<Vec<_>, Error>>()?;

    for (sha, path) in digests {
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
