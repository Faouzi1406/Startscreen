#[derive(Debug)]
pub enum FilesError {
}

pub struct Files {
    pub variables:String,
    pub assci:String
}

pub fn include_files() -> Result<Files, FilesError>{
    //Include variables config in binary
    let variables = include_str!("../../start.start").to_owned();
    let assci = include_str!("../../assci.start").to_owned();

    Ok(Files {variables, assci})
}
