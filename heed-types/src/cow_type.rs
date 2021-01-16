use std::borrow::Cow;

use heed_traits::{BytesDecode, BytesEncode};
use bytemuck::{Pod, PodCastError, bytes_of, bytes_of_mut, try_from_bytes};

/// Describes a type that must be [memory aligned] and
/// will be reallocated if it is not.
///
/// A [`Cow`] type is returned to represent this behavior.
///
/// If you need to store a type that doesn't depends on any
/// memory alignment it is recommended to use the [`UnalignedType`].
///
/// If you don't want to be bored with the [`Cow`] type you can
/// use the [`OwnedType`].
///
/// To store slices, you must look at the [`CowSlice`],
/// [`OwnedSlice`] or [`UnalignedSlice`] types.
///
/// [memory aligned]: std::mem::align_of()
/// [`Cow`]: std::borrow::Cow
/// [`UnalignedType`]: crate::UnalignedType
/// [`OwnedType`]: crate::OwnedType
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice
/// [`CowSlice`]: crate::CowSlice
pub struct CowType<T>(std::marker::PhantomData<T>);

impl<T: Pod> BytesEncode for CowType<T> {
    type EItem<'a> = T;

    fn bytes_encode<'a, 'b>(item: &'b Self::EItem<'a>) -> Option<Cow<'a, [u8]>> {
        Some(Cow::Owned(bytes_of(item).to_vec()))
    }
}

impl<'a, T: Pod + 'a> BytesDecode<'a> for CowType<T> {
    type DItem = Cow<'a, T>;

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        match try_from_bytes(bytes) {
            Ok(item) => Some(Cow::Borrowed(item)),
            Err(PodCastError::SizeMismatch) => None,
            Err(_) => {
                let mut item = T::zeroed();
                bytes_of_mut(&mut item).copy_from_slice(bytes);
                Some(Cow::Owned(item))
            },
        }
    }
}

unsafe impl<T> Send for CowType<T> {}

unsafe impl<T> Sync for CowType<T> {}
