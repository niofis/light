pub mod path_tracing;
pub mod whitted;

pub enum Algorithm {
    Whitted,
    PathTracing,
}
