#[cfg(feature = "blocking-client")]
mod blocking;
#[cfg(not(feature = "http-client-curl"))]
mod capabilities;
#[cfg(feature = "blocking-client")]
mod git;
