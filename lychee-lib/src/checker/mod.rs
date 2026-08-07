//! Checker Module
//!
//! This module contains all checkers, which are responsible for checking the status of a URL.

use std::path::{Path, PathBuf};

pub(crate) mod file;
pub(crate) mod mail;
pub(crate) mod website;
pub(crate) mod wikilink;

/// Returns fallback candidates in configured extension order.
///
/// For each extension, the appended form is tried before the replaced form.
/// Only the appended form is produced for a path without an extension, or with
/// one containing whitespace, which is taken to be part of the file name.
pub(crate) fn fallback_candidates<'a>(
    path: &'a Path,
    extensions: &'a [String],
) -> impl Iterator<Item = PathBuf> + 'a {
    let can_replace = path.extension().is_some_and(|extension| {
        !extension
            .as_encoded_bytes()
            .iter()
            .any(u8::is_ascii_whitespace)
    });

    extensions.iter().flat_map(move |extension| {
        // Operate on the file name so trailing slashes are normalized like
        // `set_extension`: `/a/b/` becomes `/a/b.html`, not `/a/b/.html`.
        let appended = path.file_name().map(|file_name| {
            let mut file_name = file_name.to_os_string();
            file_name.push(".");
            file_name.push(extension);
            path.with_file_name(file_name)
        });

        let replaced = can_replace.then(|| {
            let mut candidate = path.to_path_buf();
            candidate.set_extension(extension);
            candidate
        });

        appended.into_iter().chain(replaced)
    })
}
