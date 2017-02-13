use core::array::FixedSizeArray;

pub trait ArrayLength<T> {

    type Array: FixedSizeArray<T>;
}