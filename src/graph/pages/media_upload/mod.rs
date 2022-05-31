#[cfg(any(feature = "reqwst"))]
pub mod video_by_reqwest;

//#[cfg(any(feature = "seed_async"))]
#[cfg(any(feature = "web_sis", feature = "seed_async"))]
pub mod video_by_web_sys;
