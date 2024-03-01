use executor::Result;

/// A timer which can be used to make repeated time-based calls
pub struct Timer {
    /// Prevents this struct from being constructed elsewhere
    _priv: (),
}

impl Timer {
    /// Creates a new [`Timer`]
    pub fn new() -> Result<Self> {
        Ok(Timer { _priv: () })
    }
}
