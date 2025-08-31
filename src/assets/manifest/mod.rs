mod id;
mod loader;
#[allow(clippy::module_inception)]
mod manifest;
mod plugin;

pub use self::{id::Id, loader::ManifestLoader, manifest::Manifest, plugin::ManifestPlugin};
