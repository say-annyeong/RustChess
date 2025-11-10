pub trait Dimension<const D: usize> {
    #[inline(always)]
    fn dimensions() -> usize { D }
}