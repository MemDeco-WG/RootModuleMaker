//! Unified author handling logic for core domain.
//! Provides pure functions for merging author information from multiple sources.

use crate::core::types::{Author, ModuleProp, MetaConfig};

/// Merge author information from multiple sources.
/// Priority: primary > fallback > fallback2.
/// If primary has name/email, use it; otherwise fall back to others.
pub fn merge_authors(primary: Option<&Author>, fallback: Option<&Author>, fallback2: Option<&Author>) -> Author {
    let result = Author::default();

    // Try primary first
    if let Some(auth) = primary {
        if !auth.is_default() {
            return auth.clone();
        }
    }

    // Then fallback
    if let Some(auth) = fallback {
        if !auth.is_default() {
            return auth.clone();
        }
    }

    // Finally fallback2
    if let Some(auth) = fallback2 {
        if !auth.is_default() {
            return auth.clone();
        }
    }

    result
}

/// Extract author from module.prop file (pure function, assumes content is provided).
/// In practice, this would be called after reading the file in cmds layer.
pub fn author_from_module_prop(module_prop: &ModuleProp) -> Option<Author> {
    module_prop.author.clone()
}

/// Extract author from git config (stub - pure function, assumes git info is provided).
/// In practice, git reading happens in cmds layer.
pub fn author_from_git_info(git_name: Option<&str>, git_email: Option<&str>) -> Option<Author> {
    if let (Some(name), Some(email)) = (git_name, git_email) {
        if !name.is_empty() && !email.is_empty() {
            Some(Author {
                name: name.to_string(),
                email: Some(email.to_string()),
            })
        } else {
            None
        }
    } else {
        None
    }
}

/// Extract author from meta config (stub).
pub fn author_from_meta(_meta: &MetaConfig) -> Option<Author> {
    // Placeholder: in real impl, meta might have global author defaults
    None
}

/// Validate author: ensure name is not empty, email is valid if present.
pub fn validate_author(author: &Author) -> bool {
    !author.name.trim().is_empty() && author.email.as_ref().map_or(true, |e| e.contains('@'))
}
