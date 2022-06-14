#[cfg(any(feature = "reqwest"))]
pub mod video_by_reqwest;

#[cfg(any(feature = "web-sys"))]
pub mod video_by_web_sys;
