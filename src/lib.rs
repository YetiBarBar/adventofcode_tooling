use std::num::ParseIntError;

use thiserror::Error;

pub mod inputparser;
pub use crate::inputparser::{
    read_lines, read_lines_to_t_iterator, read_lines_to_vec_t, read_single_string_to_t_vec,
};

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Incorrect parsing")]
    ParsingError,
    #[error("Invalid int parsing")]
    ParseIntError(#[from] ParseIntError),
    #[error("IoError")]
    IoError(#[from] std::io::Error),
}
