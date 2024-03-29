//! check [profile.txt](https://github.com/poly000/cavestory-save-lib/blob/master/profile.txt) for value validation.

/// Custom types to ensure data be valid
pub mod items;

mod profile;
mod game_profile;

pub use profile::Profile;
pub use profile::ProfileError;
pub use game_profile::GameProfile;

#[cfg(feature = "strum")]
pub use strum;