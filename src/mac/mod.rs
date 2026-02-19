#[cfg(feature = "prebuild")]
include!("prebuilt.rs");

#[cfg(not(feature = "prebuild"))]
include!(concat!(env!("OUT_DIR"), "/mac_generated.rs"));
