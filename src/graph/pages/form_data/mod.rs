#[cfg(any(feature = "web_sis", feature = "seed_async"))]
pub mod web_sys_form;

#[cfg(any(feature = "reqwst"))]
pub mod reqwest_form;
