/// A trait that should be implemented by each module to instantiate it and set up the storage.
pub trait Instanced {
    /// Instantiates the module.
    fn instance(namespace: &str) -> Self;
}
