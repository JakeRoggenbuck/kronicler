pub trait Metadata<T> {
    /// Save the struct to a file
    fn save(&self);
    /// Load a struct from a file
    fn load(&self) -> T;
}
