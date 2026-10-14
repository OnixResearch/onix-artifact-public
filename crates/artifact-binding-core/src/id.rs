//! Bounded opaque identifiers and monotonic binding revisions.

use alloc::string::{String, ToString};
use core::fmt;

/// Identifier validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentifierError {
    /// The caller supplied an empty identifier.
    Empty,
    /// The caller supplied no usable identifier byte budget.
    MaximumBytesZero,
    /// The identifier exceeds the caller-selected byte budget.
    TooLong {
        /// Caller-selected maximum.
        maximum_bytes: usize,
        /// Supplied byte length.
        actual_bytes: usize,
    },
    /// The identifier contains control characters.
    ControlCharacter,
    /// Leading or trailing whitespace would create ambiguous identity.
    SurroundingWhitespace,
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid opaque identifier: {self:?}")
    }
}

fn validate(value: &str, maximum_bytes: usize) -> Result<(), IdentifierError> {
    if maximum_bytes == 0 {
        return Err(IdentifierError::MaximumBytesZero);
    }
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }
    if value.len() > maximum_bytes {
        return Err(IdentifierError::TooLong {
            maximum_bytes,
            actual_bytes: value.len(),
        });
    }
    if value.trim() != value {
        return Err(IdentifierError::SurroundingWhitespace);
    }
    if value.chars().any(char::is_control) {
        return Err(IdentifierError::ControlCharacter);
    }
    Ok(())
}

macro_rules! opaque_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Construct an identifier under an explicit caller-selected byte bound.
            ///
            /// # Errors
            ///
            /// Returns [`IdentifierError`] for empty, unbounded, oversized,
            /// control-bearing, or whitespace-ambiguous input.
            pub fn try_new(value: &str, maximum_bytes: usize) -> Result<Self, IdentifierError> {
                validate(value, maximum_bytes)?;
                Ok(Self(value.to_string()))
            }

            /// Borrow the consumer-owned identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

opaque_id!(ArtifactId, "Consumer-owned immutable artifact reference.");
opaque_id!(BindingKey, "Consumer-owned logical binding key.");
opaque_id!(
    ContractId,
    "Consumer-owned compatibility contract reference."
);
opaque_id!(SnapshotId, "Consumer-owned immutable snapshot identity.");
opaque_id!(GenerationId, "Consumer-owned generation identity.");
opaque_id!(RootId, "Consumer-owned root identity.");
opaque_id!(RootClassId, "Consumer-owned declared root-class identity.");
opaque_id!(ObservationId, "Consumer-owned observation identity.");

/// Monotonic binding revision used only for structural successor checks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingRevision(u64);

impl BindingRevision {
    /// Construct an explicit revision. Revision zero is valid when a consumer uses it.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the consumer-supplied numeric revision.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Return the exact checked successor.
    #[must_use]
    pub const fn checked_successor(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID_LIMIT: usize = 32;
    const CURRENT_REVISION: u64 = 7;
    const SUCCESSOR_REVISION: u64 = CURRENT_REVISION + 1;

    #[test]
    fn typed_ids_preserve_exact_bounded_text() {
        let artifact = ArtifactId::try_new("blake3:fixture", ID_LIMIT);
        assert_eq!(
            artifact.as_ref().map(ArtifactId::as_str),
            Ok("blake3:fixture")
        );
    }

    #[test]
    fn typed_ids_reject_empty_oversized_and_ambiguous_text() {
        assert_eq!(
            ArtifactId::try_new("", ID_LIMIT),
            Err(IdentifierError::Empty)
        );
        assert!(matches!(
            ArtifactId::try_new("identifier-that-exceeds-the-named-limit", ID_LIMIT),
            Err(IdentifierError::TooLong { .. })
        ));
        assert_eq!(
            ArtifactId::try_new(" spaced", ID_LIMIT),
            Err(IdentifierError::SurroundingWhitespace)
        );
        assert_eq!(
            ArtifactId::try_new("line\nbreak", ID_LIMIT),
            Err(IdentifierError::ControlCharacter)
        );
        assert_eq!(
            ArtifactId::try_new("valid", 0),
            Err(IdentifierError::MaximumBytesZero)
        );
    }

    #[test]
    fn revision_successor_is_checked() {
        assert_eq!(
            BindingRevision::new(CURRENT_REVISION).checked_successor(),
            Some(BindingRevision::new(SUCCESSOR_REVISION))
        );
        assert_eq!(BindingRevision::new(u64::MAX).checked_successor(), None);
    }
}
