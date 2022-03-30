pub trait Instanced {
    fn instance(namespace: &str) -> Self;
}
