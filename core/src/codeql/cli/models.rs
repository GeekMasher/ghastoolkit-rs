use std::collections::HashMap;

/// JSON representation of the languages supported by the CodeQL CLI
///
/// ```bash
/// codeql resolve languages --format json
/// ```
pub(crate) type ResolvedLanguages = HashMap<String, Vec<String>>;
