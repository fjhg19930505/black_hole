/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IBaseLogicInfo.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base logic info interface define
//  Others:
//  History:
*************************************************/
use super::base_logic_creator::{IBaseLogicCallBack, IBaseLogicCreator};
use crate::runtime::inlines;
use crate::share::any_list::IArrayList;
use std::borrow::Borrow;
use std::os::raw::c_void;

// 逻辑方法信息
#[derive(Debug)]
pub struct ICallBackInfo {
    name_: String,
    hash_: u32,
    mid_func_: *const c_void,
    return_table_: bool,
}

impl ICallBackInfo {
    fn get_name(self) -> String {
        self.name_
    }

    fn get_mid_func(&self) -> *const c_void {
        self.mid_func_
    }

    fn get_return_table(&self) -> bool {
        self.return_table_
    }

    fn get_hash(&self) -> u32 {
        self.hash_
    }

    fn set_name(&mut self, value: String) {
        self.name_ = value;
    }

    fn set_hash(&mut self, value: u32) {
        self.hash_ = value;
    }

    fn set_mid_func(&mut self, value: *const c_void) {
        self.mid_func_ = value;
    }

    fn set_return_table(&mut self, value: bool) {
        self.return_table_ = value;
    }
}

#[derive(Debug)]
pub struct IBaseLogicInfo {
    creator_: Option<Box<IBaseLogicCreator>>,
    space_name_: String,
    logic_name_: String,
    call_back_infos_: Vec<Box<ICallBackInfo>>,
}

impl IBaseLogicInfo {
    pub fn get_creator(&self) -> Option<Box<IBaseLogicCreator>> {
        self.creator_
    }

    pub fn get_space_name(&self) -> String {
        self.space_name_
    }

    fn get_logic_name(&self) -> String {
        self.logic_name_
    }

    fn get_callback_count(&self) -> usize {
        self.call_back_infos_.len()
    }

    fn get_callback_list(&self, result: &mut IArrayList) -> usize {
        result.clear();
        self.inner_get_callback_list(result);
        result.get_count()
    }

    fn get_callback_info(&self, name: &str) -> Option<Box<ICallBackInfo>> {
        let mut index: usize = 0;

        if !self.find_callback_index(name, index) {
            return None;
        }

        Some(self.call_back_infos_[index])
    }

    fn inner_get_callback_list(&self, result: &mut IArrayList) {
        let size = self.call_back_infos_.len();

        for i in 0..size {
            result.add_str(self.call_back_infos_[i].get_name().as_str());
        }
    }

    fn find_callback_index(&self, name: &str, mut index: usize) -> bool {
        let hash = inlines::get_hash_value_case(name);

        for callback in self.call_back_infos_ {
            if callback.get_hash() == hash
                && String::from(callback.name_) == String::from(name).borrow()
            {
                index += 1;
                return true;
            }
        }
        false
    }

    fn set_creator(&self, value: Option<Box<IBaseLogicCreator>>) {
        self.creator_ = value;
    }

    fn set_space_name(&self, value: &str) {
        self.space_name_ = String::from(value);
    }

    fn set_logic_name(&self, value: &str) {
        self.logic_name_ = String::from(value);
    }

    fn add_callback_info(&mut self, name: &str, mid_func: *const c_void, ret_table: bool) {
        self.call_back_infos_.push(Box::new(ICallBackInfo {
            name_: String::from(name),
            hash_: inlines::get_hash_value_case(name),
            mid_func_: mid_func,
            return_table_: ret_table,
        }));
    }

    fn add_callback_link(&mut self, call_back: Option<Box<IBaseLogicCallBack>>) -> usize {
        let mut count: usize = 0;
        let mut temp = call_back;
        loop {
            match temp {
                None => {break;},
                IBaseLogicCallBack => {
                    let temp_callback = temp.unwrap();
                    temp = temp_callback.next_;
                    count += 1;
                }
            }
        }
        let mut index = count - 1;

        temp = call_back;
        loop {
            match temp {
                None => {break;},
                IBaseLogicCallBack => {
                    let mut data = &self.call_back_infos_[index];
                    let temp_callback = temp.unwrap();
                    data.set_name(temp_callback.name_);
                    data.set_hash(inlines::get_hash_value_case(temp_callback.name_.as_ref()));
                    data.set_mid_func(temp_callback.mid_func_);
                    data.set_return_table(temp_callback.return_table_);

                    index -= 1;
                    temp = temp_callback.next_;
                },
            }
        }

        count
    }
}
