use cxx::{UniquePtr, memory::UniquePtrTarget};
// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use moveit::{CopyNew, New, MoveNew, DerefMove};

use std::{mem::MaybeUninit, pin::Pin, ops::Deref};

/// A trait which is used to receive any C++ parameter passed by value.
/// This trait is implemented both for references `&T` and for `UniquePtr<T>`,
/// subject to the presence or absence of suitable copy and move constructors.
/// This allows you to pass in parameters by copy (as is ergonomic and normal
/// in C++) retaining the original parameter; or by move semantics thus
/// destroying the object you're passing in. Simply use a reference if you want
/// copy semantics, or the item itself if you want move semantics.
/// It is not recommended that you implement this trait.
pub trait ValueParam<T> {
    unsafe fn new(self, this: Pin<&mut MaybeUninit<T>>);
}

impl<T> ValueParam<T> for &T
where
    T: CopyNew,
{
    unsafe fn new(self, this: Pin<&mut MaybeUninit<T>>) {
        crate::moveit::new::copy(self).new(this);
    }
}

impl<'a, T> ValueParam<T> for UniquePtr<T>
where
    T: 'a + MoveNew + UniquePtrTarget,
    &'a mut T: DerefMove + Deref<Target=T>
{
    unsafe fn new(mut self, this: Pin<&mut MaybeUninit<T>>) {
        crate::moveit::new::mov(self.pin_mut()).new(this)
    }
}
