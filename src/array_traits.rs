pub trait AsPtr<Raw = Self> {
    fn as_ptr(&self) -> *const Raw;
}

impl<T> AsPtr<T> for T {
    fn as_ptr(&self) -> *const T {
        self as *const T
    }
}

pub trait Len {
    fn len(&self) -> usize;
}

impl Len for [f32; 8] {
    fn len(&self) -> usize {
        8
    }
}

impl Len for [f32; 16] {
    fn len(&self) -> usize {
        16
    }
}
