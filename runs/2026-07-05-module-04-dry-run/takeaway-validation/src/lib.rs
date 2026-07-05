//! Validation problem for the trait-bound/lifetime-annotation cheat-sheet
//! (`.claude/skills/trait-lifetime-cheatsheet/SKILL.md`), deliberately
//! unrelated to `relay`/checkpoints - a config-store lookup instead of a
//! notification alert.

/// Any key/value config backend. `lookup` below is generic over this trait
/// rather than hardcoding one backend, same shape as `relay`'s `Notifier`.
pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
}

/// The result of looking a key up in a `Store`: which key was asked for,
/// what value (if any) came back. `key` borrows the caller's own key rather
/// than cloning it - nothing here needs to outlive that string.
pub struct LookupResult<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}

/// Look a key up in any `Store`. `'a` is tied only to `key`, the one
/// reference the output actually borrows - `store` doesn't share it, since
/// nothing about the store needs to outlive the returned `LookupResult`.
pub fn lookup<'a, S: Store>(store: &S, key: &'a str) -> LookupResult<'a> {
    LookupResult {
        key,
        value: store.get(key),
    }
}
