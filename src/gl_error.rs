//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  gl_error.rs
//  module:
//! gl error system

use std::error::Error;
use gl;

#[derive(Debug)]
pub enum GlError {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory
}

impl GlError {
    pub fn new() -> Result<(), GlError> {
        unsafe {
        match gl::GetError() {
            gl::NO_ERROR => Ok(()),
            gl::INVALID_ENUM => Err(GlError::InvalidEnum),
            gl::INVALID_VALUE => Err(GlError::InvalidValue),
            gl::INVALID_OPERATION => Err(GlError::InvalidOperation),
            gl::INVALID_FRAMEBUFFER_OPERATION => Err(GlError::InvalidFramebufferOperation),
            gl::OUT_OF_MEMORY => Err(GlError::OutOfMemory),
            _ => Ok(())
        }
        }
    }
}

impl Error for GlError {
    fn cause(&self) -> Option<&Error> {
        None
    }
}

use std::fmt;

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GlError::NoError => { write!(f, "No Error") },
            GlError::InvalidEnum => { write!(f, "Bad enum argument") },
            GlError::InvalidValue => { write!(f, "Bad value argument") },
            GlError::InvalidOperation => { write!(f, "Bad operation") },
            GlError::InvalidFramebufferOperation => { write!(f, "Bad framebuffer operation") },
            GlError::OutOfMemory => { write!(f, "No more gl memory") }
        }
    }
}


