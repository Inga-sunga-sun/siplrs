use std::marker::PhantomData;

#[derive(Debug,Default)]
pub struct ImageBuffer<T>
{
    width: usize,
    height: usize,
    data: Vec<T>,
}

