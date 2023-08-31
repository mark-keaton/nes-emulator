pub trait AdjustBy1 {
    fn decrement(&mut self) -> ();
    fn increment(&mut self) -> ();
}

pub trait Comparison {
    fn is_zero(self) -> bool;
}
