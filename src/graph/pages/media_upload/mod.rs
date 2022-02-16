#[cfg(any(feature = "reqwest_async"))]
pub mod video_by_reqwest;

//#[cfg(any(feature = "seed_async"))]
#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
pub mod video_by_seed;
