use std::process;
use thiserror::Error;

#[derive(Debug)]
pub struct CliOutput(pub Result<String, TigerSharkError>);

#[derive(Error, Debug)]
pub enum TigerSharkError {
    #[error("FilePath Error: `{0}`")]
    FilePathError(String),
    #[error("CLI Error: `{0}`")]
    CliError(String),
    #[error("DB Error: `{0}`")]
    DbError(String),
    #[error("Not Found: `{0}`")]
    NotFound(String),
    #[error("Some Asset keys missing for command `{0}`: `{1}`")]
    AssetKeysError(String, String),
    #[error("unknown error")]
    Unknown,
}

// exit the program with:
// - a message
// - an exitcode (101:error, 0:ok)
pub fn exit_or_panic(cli_output: CliOutput) {
    match cli_output.0 {
        Ok(o) => {
            print!("{:?}", o);
            process::exit(0);
        }
        Err(e) => {
            print!("{:?}", e);
            process::exit(101);
        }
    }
}
