use image::imageops::FilterType;
use std::path::Path;

const THRESHOLD: u32 = 5;

pub fn sont_identiques(path1: &Path, path2: &Path) -> bool {
    let h1 = hash(path1).ok();
    let h2 = hash(path2).ok();
    
    match (h1, h2) {
        (Some(a), Some(b)) => (a ^ b).count_ones() <= THRESHOLD,
        _ => false,
    }
}

fn hash(path: &Path) -> Result<u64, image::ImageError> {
    let img = image::open(path)?
        .resize_exact(8, 8, FilterType::Nearest)
        .grayscale()
        .into_luma8();

    let pixels = img.as_raw();
    let moyenne = pixels.iter().map(|&p| p as u32).sum::<u32>() / 64;

    let hash = pixels.iter().enumerate().fold(0u64, |acc, (i, &p)| {
        if p as u32 > moyenne { acc | (1 << i) } else { acc }
    });

    Ok(hash)
}