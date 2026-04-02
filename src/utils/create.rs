use std::fs;
use std::io;

pub fn create(file_tmp_result: &str, file_result: &str) -> io::Result<()> {
    for dir in [file_tmp_result, file_result] {
        match fs::create_dir(dir) {
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                println!("Le dossier '{}' existe déjà.", dir);
            }
            Err(e) => {
                eprintln!("\nImpossible de créer '{}': {} ({:?})\n", dir, e, e.kind());
                return Err(e);
            }
        }
    }
    Ok(())
}