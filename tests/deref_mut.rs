#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate derive_more;

#[derive(DerefMut)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);
// Deref implementation is needed for DerefMut
impl ::core::ops::Deref for MyBoxedInt {
    type Target = <Box<i32> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <Box<i32> as ::core::ops::Deref>::deref(&self.0)
    }
}

#[derive(DerefMut)]
struct NumRef<'a> {
    #[deref_mut(forward)]
    num: &'a mut i32,
}
// Deref implementation is needed for DerefMut
impl<'a> ::core::ops::Deref for NumRef<'a> {
    type Target = <&'a mut i32 as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <&'a mut i32 as ::core::ops::Deref>::deref(&self.num)
    }
}

#[derive(DerefMut)]
#[deref_mut(forward)]
struct NumRef2<'a> {
    num: &'a mut i32,
    #[deref_mut(ignore)]
    useless: bool,
}

// Deref implementation is needed for DerefMut
impl<'a> ::core::ops::Deref for NumRef2<'a> {
    type Target = <&'a mut i32 as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <&'a mut i32 as ::core::ops::Deref>::deref(&self.num)
    }
}

#[derive(DerefMut)]
struct MyInt(i32);

// Deref implementation is needed for DerefMutToInner
impl ::core::ops::Deref for MyInt {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(DerefMut)]
struct Point1D {
    x: i32,
}

// Deref implementation is needed for DerefMutToInner
impl ::core::ops::Deref for Point1D {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

#[derive(DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref_mut]
    vec: Vec<i32>,
}
impl ::core::ops::Deref for CoolVec {
    type Target = Vec<i32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
