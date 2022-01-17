#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {}

#[cfg(feature = "async-std")]
#[async_std::main]
async fn main() {}

#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn main() {}
