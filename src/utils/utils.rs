use crate::utils;

pub fn utils(file_result: &str, file_tmp_result: &str) -> Result<(), Box<dyn std::error::Error>> {
    utils::delete::delete(file_result, file_tmp_result)?;
    utils::create::create(file_result, file_tmp_result)?;
    Ok(())
}