#![allow(clippy::wrong_self_convention)]

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_void, c_char, c_int};
use std::ptr;
use std::marker::PhantomData;
use std::ops::Deref;
use super::vk;

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

unsafe fn insert_next(mut head: *mut vk::BaseOutStructure, other: *mut vk::BaseOutStructure) {
    assert!((*other).p_next.is_null());
    (*other).p_next = (*head).p_next;
    (*head).p_next = other as *mut _;
}

