use std::path::{Path};
use std::fs;

pub fn deplace(doublons: &[impl AsRef<Path>], destination_dir: impl AsRef<Path>) {
    let dest_path = destination_dir.as_ref();

    for path in doublons {
        let path = path.as_ref();

        if let Some(nom_fichier) = path.file_name() {
            let destination = dest_path.join(nom_fichier);
            
            if let Err(e) = fs::rename(path, &destination) {
                eprintln!("Erreur {:?}: {}", nom_fichier, e);
            } else {
                println!("Déplacé: {:?}", nom_fichier);
            }
        }
    }
}