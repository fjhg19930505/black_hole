/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       ObjId.rs
//  Author:        	 sll
//  Version:         1.0
//  Date:            2017/08/07
//  Description:
//  Others:
//  History:
*************************************************/
use std::ops::{Deref};
use crate::scene_server::core::object::IObject;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ObjId {
    ident_: u32,
    serial_: u32,
}

impl ObjId {
    pub fn is_null(&self) -> bool{
        self.ident_ == 0 && self.serial_ == 0
    }

    pub fn equal_to(&self, other: &ObjId) -> bool{
        self.ident_ == other.ident_ && self.serial_ == other.serial_
    }

    pub fn new_null() -> ObjId {
        ObjId{ident_: 0, serial_: 0}
    }

}

#[derive(Debug)]
struct ObjIdDbg {
    obj_: ObjId,
    obj_ptr_: Box<IObject>,
}

impl ObjIdDbg {
    fn new(id: ObjId, obj_ptr: Box<IObject>) -> ObjIdDbg {
        ObjIdDbg{obj_: id, obj_ptr_: obj_ptr}
    }
}

impl Deref for ObjIdDbg {
    type Target = ObjId;
    fn deref(&self) -> &Self::Target {
         &self.obj_
    }
}
