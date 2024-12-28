mod build {
    use std::time::Instant;

    pub fn now() -> Instant {
        Instant::now()
    }
}

pub use build::*;
