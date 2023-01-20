/// A trait that should be implemented by each module to instantiate it and set up the storage.
pub trait Instanced {
    fn instance(namespace: &str) -> Self;
}
