#[cfg(any(feature = "web_sys_async", feature = "seed_async"))]
pub mod web_sys_form;

#[cfg(any(feature = "reqwest_async"))]
pub mod reqwest_form;