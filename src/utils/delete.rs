use std::fs;
use std::io;
use std::path::Path;

pub fn delete<P: AsRef<Path>>(path1: P, path2: P) -> io::Result<()> {
    for path in [path1.as_ref(), path2.as_ref()] {
        match fs::remove_dir_all(path) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                eprintln!("rien à supprimer pour {:?}", path);
            }
            Err(e) => {
                eprintln!("\nImpossible de supprimer {:?}: {} ({:?})\n", path, e, e.kind());
                return Err(e);
            }
        }
    }
    Ok(())
}