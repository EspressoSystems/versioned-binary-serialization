use std::array::TryFromSliceError;

use snafu::Snafu;

use crate::version::Version;

#[derive(Clone, Debug, Snafu)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum Error<NativeError: std::error::Error + 'static> {
    #[snafu(display("{source}"))]
    VersionHeader { source: TryFromSliceError },
    #[snafu(display("{source}"))]
    ExternSerializer { source: NativeError },
    #[snafu(display("expected version {wanted}, got {version}"))]
    VersionMismatch { version: Version, wanted: Version },
}
