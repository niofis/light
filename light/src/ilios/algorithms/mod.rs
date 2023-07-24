pub mod path_tracing;
pub mod whitted;

#[derive(Debug)]
pub enum Algorithm {
    Whitted,
    PathTracing,
}
