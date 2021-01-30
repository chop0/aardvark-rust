use crate::var::AardvarkValue;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AardvarkError {
    Return(Box<dyn AardvarkValue>),
    Other(String)
}

impl fmt::Display for AardvarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AardvarkError::Return(_) => {
                fmt::Result::Ok(())
            }
            AardvarkError::Other(other) => {
                f.write_str(other.as_str())
            }
        }
    }
}

impl Error for AardvarkError {

}

