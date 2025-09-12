#![allow(clippy::wrong_self_convention, clippy::unnecessary_cast)]

use std::{
    ffi::CStr,
    os::raw::{c_void, c_char},
    ptr,
    marker::PhantomData,
    ops::Deref,
};
use super::vk;

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

unsafe fn insert_next(head: *mut vk::BaseOutStructure, other: *mut vk::BaseOutStructure) {
    assert!((*other).p_next.is_null());
    (*other).p_next = (*head).p_next;
    (*head).p_next = other as *mut _;
}
