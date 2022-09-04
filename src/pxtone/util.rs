use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// Wrapper around either a `Box<T>` or `&T`
///
/// `Deref`s into `&T`
pub enum BoxOrRef<'a, T: 'a + ?Sized> {
    Ref(&'a T),
    Box(Box<T>),
}

impl<'a, T: 'a + ?Sized> Deref for BoxOrRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Ref(r) => r,
            Self::Box(ref o) => o,
        }
    }
}

impl<'a, T: 'a + ?Sized> Borrow<T> for BoxOrRef<'a, T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T: 'a> From<T> for BoxOrRef<'a, T> {
    fn from(v: T) -> Self {
        Self::Box(Box::new(v))
    }
}

impl<'a, T: 'a + ?Sized> From<&'a T> for BoxOrRef<'a, T> {
    fn from(v: &'a T) -> Self {
        Self::Ref(v)
    }
}

/// Wrapper around either a `Box<T>` or `&mut T`
///
/// `Deref`s into `&mut T`
pub enum BoxOrMut<'a, T: 'a + ?Sized> {
    Ref(&'a mut T),
    Box(Box<T>),
}

impl<'a, T: 'a + ?Sized> Deref for BoxOrMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => r,
            Self::Box(o) => o,
        }
    }
}

impl<'a, T: 'a + ?Sized> DerefMut for BoxOrMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Ref(r) => r,
            Self::Box(o) => o,
        }
    }
}

impl<'a, T: 'a + ?Sized> Borrow<T> for BoxOrMut<'a, T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T: 'a + ?Sized> BorrowMut<T> for BoxOrMut<'a, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<'a, T: 'a> From<T> for BoxOrMut<'a, T> {
    fn from(v: T) -> Self {
        Self::Box(Box::new(v))
    }
}

impl<'a, T: 'a + ?Sized> From<&'a mut T> for BoxOrMut<'a, T> {
    fn from(v: &'a mut T) -> Self {
        Self::Ref(v)
    }
}
