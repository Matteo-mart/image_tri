use std::fs::{read_dir, ReadDir};

pub fn lecture(chemin_dossier: &str) -> Result<ReadDir, std::io::Error> {
    let dossier = read_dir(chemin_dossier)?;
    println!("Dossier '{}' ouvert avec succès", chemin_dossier);
    Ok(dossier)
}