use std::borrow::Cow;

use bytemuck::{Pod, PodCastError, try_cast_slice, pod_collect_to_vec};
use heed_traits::{BytesDecode, BytesEncode};

/// Describes a slice that must be [memory aligned] and
/// will be reallocated if it is not.
///
/// A [`Cow`] type is returned to represent this behavior.
///
/// If you need to store a slice that doesn't depends on any
/// memory alignment it is recommended to use the [`UnalignedSlice`].
///
/// if you don't want to be bored with the [`Cow`] type you can
/// use the [`OwnedSlice`].
///
/// [memory aligned]: std::mem::align_of()
/// [`Cow`]: std::borrow::Cow
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice
pub struct CowSlice<T>(std::marker::PhantomData<T>);

impl<T: Pod> BytesEncode for CowSlice<T> {
    type EItem<'a> = &'a [T];

    fn bytes_encode<'a, 'b>(item: &'b Self::EItem<'a>) -> Option<Cow<'a, [u8]>> {
        try_cast_slice(item).map(Cow::Borrowed).ok()
    }
}

impl<'a, T: Pod + 'a> BytesDecode<'a> for CowSlice<T> {
    type DItem = Cow<'a, [T]>;

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        match try_cast_slice(bytes) {
            Ok(items) => Some(Cow::Borrowed(items)),
            Err(PodCastError::SizeMismatch) => None,
            Err(_) => Some(Cow::Owned(pod_collect_to_vec(bytes))),
        }
    }
}

unsafe impl<T> Send for CowSlice<T> {}

unsafe impl<T> Sync for CowSlice<T> {}
