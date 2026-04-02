use crate::execute::{deplace, hasher, lecture};
use std::path::PathBuf;

const EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "img"];

pub fn execute(chemin_dossier: &str, file_tmp_result: &str) -> Result<(), Box<dyn std::error::Error>> {
    let images: Vec<PathBuf> = lecture::lecture(chemin_dossier)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| EXTENSIONS.contains(&e.to_lowercase().as_str())))
        .collect();

    println!("{} images trouvées", images.len());

    let doublons: Vec<&PathBuf> = images.iter()
        .enumerate()
        .flat_map(|(i, a)| images[i + 1..]
            .iter()
            .filter(|b| hasher::sont_identiques(a, b))
            .inspect(|b| println!("Doublon: {:?} == {:?}", a.file_name().unwrap(), b.file_name().unwrap())))
        .collect();

    println!("{} doublons trouvés", doublons.len());
    deplace::deplace(&doublons, file_tmp_result);
    Ok(())
}