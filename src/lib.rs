pub enum Algorithm {
    Lz4,
    Zstd,
    Brotli,
}

pub fn compress(input: &[u8], algo: Algorithm) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match algo {
        Algorithm::Lz4 => Ok(input.to_vec()),
        Algorithm::Zstd => Ok(input.to_vec()),
        Algorithm::Brotli => Ok(input.to_vec()),
    }
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(input.to_vec())
}
