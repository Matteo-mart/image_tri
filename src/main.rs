mod utils;
mod execute;

fn main() {
    println!("Hello, world!");
    
    let chemin_dossier = match utils::arg_commande::arg_commande() {
        Ok(chemin) => chemin,
        Err(e) => {
            eprintln!("Erreur: {}", e);
            return;
        }
    };
    let file_tmp_result = "file_tmp_result";

    let resultats = [
        utils::utils::utils(file_tmp_result).map_err(|e| format!("[utils] {}", e)),
        execute::execute::execute(&chemin_dossier, file_tmp_result).map_err(|e| format!("[execute] {}", e)),
    ];
    println!("\nDossier crée pour les images à jeter: \n{}\n", file_tmp_result);

    let erreurs: Vec<_> = resultats.iter()
        .filter_map(|r| r
        .as_ref()
        .err())
        .collect();
    
    if erreurs.is_empty() {
        println!("\nAucune erreur\n");
    } else {
        erreurs
            .iter()
            .for_each(|e| eprintln!("\nErreur: {}", e));
    }

    println!("\nFINIS\n");

}
