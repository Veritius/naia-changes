
//use std::any::{Any, TypeId};

//use anymap::AnyMap;

use crate::NetEvent;

pub struct Manifest {
    //typemap: AnyMap,
}

impl Manifest {
    pub fn new() -> Self {
        //let mut typemap = AnyMap::new();
        Manifest {
            //typemap,
        }
    }

    pub fn register_event<T: NetEvent>(&mut self) {
        //self.typemap.insert(event);
    }

    pub fn process(&mut self) {

    }
}