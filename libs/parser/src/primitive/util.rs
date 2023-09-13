use core::mem::MaybeUninit;

pub(super) fn iter_to_array_unchecked<const C: usize, T, I>(mut iter: I) -> [T; C]
where
    I: Iterator<Item = T>,
{
    // SAFETY: all values are written before reading, so any uninitialized values are overwritten
    let mut array = unsafe { MaybeUninit::<[T; C]>::zeroed().assume_init() };
    for item in array.iter_mut() {
        *item = iter.next().unwrap();
    }

    array
}
