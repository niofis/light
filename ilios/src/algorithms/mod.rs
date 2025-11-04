pub mod path_tracing;
pub mod whitted;

#[derive(Clone, Debug)]
pub enum Algorithm {
    Whitted,
    PathTracing,
}
