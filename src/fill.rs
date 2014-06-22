use {
    BackEnd,
    ImageSize,
};

/// Fills a shape using a back-end.
pub trait Fill<'a> {
    /// Fill shape using back-end.
    fn fill<B: BackEnd<I>, I: ImageSize>(&'a self, back_end: &mut B);
}
