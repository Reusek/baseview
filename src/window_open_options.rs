use crate::{Parent, Size};

/// The dpi scaling policy of the window
#[derive(Debug)]
pub enum WindowScalePolicy {
    /// Use the system's dpi scale factor
    SystemScaleFactor,
    /// Use the given dpi scale factor (e.g. `1.0` = 96 dpi)
    ScaleFactor(f64),
}

/// The options for opening a new window
#[derive(Debug)]
pub struct WindowOpenOptions {
    pub title: String,

    /// The logical size of the window
    pub size: Size,

    /// The dpi scaling policy
    pub scale: WindowScalePolicy,

    pub parent: Parent,
}