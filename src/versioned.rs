pub trait Versioned {
    const MIN_MAJOR: u16;
    const MIN_MINOR: u16;
    const MAX_MAJOR: u16;
    const MAX_MINOR: u16;
}
