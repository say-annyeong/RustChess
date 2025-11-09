pub trait Dimension<const D: usize> {
    fn dimensions() -> usize { D }
}