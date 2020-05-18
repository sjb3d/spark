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

