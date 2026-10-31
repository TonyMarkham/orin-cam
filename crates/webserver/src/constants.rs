pub const ADDRESS: &str = "0.0.0.0:8080";
pub(crate) const VIEWER_DIR: &str = "crates/viewer/dist";
pub(crate) const HELLO_ROUTE: &str = "/";
pub(crate) const VIEWER_ROUTE: &str = "/viewer";
pub(crate) const WS_ROUTE: &str = "/ws";
pub(crate) const IMAGE_ROUTE: &str = "/image";

pub(crate) const MAX_IMAGE_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
