//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  resources.rs
//  module:
//! ressources trait and functions

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

pub type Resource<T> = Rc<T>;
pub type MutResource<T> = Rc<RefCell<T>>;
pub type ThreadResource<T> = Arc<T>;
pub type MutThreadResource<T> = Arc<Mutex<T>>;
