#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-08")]
pub mod package_2023_08;
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(feature = "package-preview-2024-05")]
pub mod package_preview_2024_05;
#[cfg(feature = "package-preview-2024-08")]
pub mod package_preview_2024_08;
#[cfg(all(feature = "default_tag", feature = "package-2023-08"))]
pub use package_2023_08::*;
