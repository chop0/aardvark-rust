#![feature(in_band_lifetimes)]

use crate::var::{AardvarkFunction, AardvarkValue};
use crate::node::base::AardvarkStatement;
use crate::frame::Frame;
use crate::error::AardvarkError;
use std::any::Any;
use std::collections::HashMap;

mod frame;
mod node;
mod error;
mod var;

pub type AardvarkResult<T> = Result<T, AardvarkError>;

fn main() {
    let mut frame = Frame {
        variables: HashMap::new(),
        parent: None
    };

    println!("Hello, world!");
}
