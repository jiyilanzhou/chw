pub trait Initialize {
    // Associate Type
    type output;
    fn new() -> Self::output;
}
