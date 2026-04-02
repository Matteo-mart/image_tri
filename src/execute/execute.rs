use crate::execute::{deplace, hasher, lecture};
use std::path::PathBuf;

const EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "img"];

pub fn execute(chemin_dossier: &str, file_tmp_result: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Lecture des fichiers dans le dossier: {}", chemin_dossier);

    let images: Vec<PathBuf> = lecture::lecture(chemin_dossier)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| EXTENSIONS.contains(&e.to_lowercase().as_str())))
        .collect();

    println!("{} images trouvées", images.len());

    let mut doublons: Vec<&PathBuf> = Vec::new();

    for (i, a) in images.iter().enumerate() {
        println!("Comparaison de l'image {:?}", a.file_name().unwrap());
        for b in &images[i + 1..] {
            if hasher::sont_identiques(a, b) {
                println!("Doublon détecté: {:?} == {:?}", a.file_name().unwrap(), b.file_name().unwrap());
                doublons.push(b);
            }
        }
    }

    println!("{} doublons trouvés", doublons.len());

    println!("Déplacement des doublons vers le dossier temporaire: {}", file_tmp_result);
    deplace::deplace(&doublons, file_tmp_result);

    println!("Traitement terminé.");
    Ok(())
}