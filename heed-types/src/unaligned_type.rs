use std::borrow::Cow;

use heed_traits::{BytesDecode, BytesEncode};
use bytemuck::{Pod, bytes_of, try_from_bytes};

/// Describes a slice that is totally borrowed and doesn't
/// depends on any [memory alignment].
///
/// If you need to store a slice that does depend on memory alignment
/// and that can be big it is recommended to use the [`CowType`].
///
/// To store slices, you must look at the [`CowSlice`],
/// [`OwnedSlice`] or [`UnalignedSlice`] types.
///
/// [memory alignment]: std::mem::align_of()
/// [`CowType`]: crate::CowType
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice
/// [`CowSlice`]: crate::CowSlice
pub struct UnalignedType<T>(std::marker::PhantomData<T>);

impl<T: Pod> BytesEncode for UnalignedType<T> {
    type EItem<'a> = T;

    fn bytes_encode<'a, 'b>(item: &'b Self::EItem<'a>) -> Option<Cow<'a, [u8]>> {
        Some(Cow::Owned(bytes_of(item).to_vec()))
    }
}

impl<'a, T: Pod + 'a> BytesDecode<'a> for UnalignedType<T> {
    type DItem = &'a T;

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        try_from_bytes(bytes).ok()
    }
}

unsafe impl<T> Send for UnalignedType<T> {}

unsafe impl<T> Sync for UnalignedType<T> {}
