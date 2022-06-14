#[cfg(any(feature = "web-sys"))]
pub mod web_sys_form;

#[cfg(any(feature = "reqwest"))]
pub mod reqwest_form;
