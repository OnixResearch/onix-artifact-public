#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
//! Disposable feasibility candidate for one-file durable publication.
//!
//! The crate is incubation evidence. It is not an Artifact workspace package
//! and grants no product authority.

extern crate alloc;

pub mod core;
pub mod corpus;
pub mod pilot;
#[cfg(feature = "std")]
pub mod shell;

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::core::{
        Classification, ClassificationError, CleanupDisposition, DurabilityMode,
        PublicationDisposition, PublicationRequest, ReplacementMode,
    };
    use crate::corpus::{CONFORMANCE_CORPUS, CorpusCase, ExpectedDisposition, FaultPoint};
    use crate::pilot::{
        ExistingContent, MANTLE_REVISION, MantlePilotRequest, ONIXOS_REVISION, OnixOverwritePolicy,
        OnixPilotRequest, map_mantle_outcome, map_mantle_request, map_onix_outcome,
        map_onix_request,
    };
    use crate::shell::{
        MechanicalIoFailure, PublicationIo, RenameDisposition, StageCreateDisposition,
        StageNameSource, publish_one_file,
    };
    use alloc::format;
    use alloc::string::String;

    const PAYLOAD: &[u8] = b"bounded-publication-payload";
    const PAYLOAD_LIMIT: usize = 128;
    const OVER_BOUND_LIMIT: usize = 1;
    const COLLISION_LIMIT: usize = 3;
    const FINAL_MODE: u32 = 0o640;
    const EXPECTED_REVISION_BYTES: usize = 40;
    const TEST_PERMISSION_MASK: u32 = 0o777;
    const INVALID_PERMISSION_MODE: u32 = 0o1_000;

    #[derive(Clone, Debug)]
    struct SequenceNames {
        next: usize,
        unsafe_name: bool,
    }

    impl StageNameSource for SequenceNames {
        fn next_stage_leaf(&mut self) -> String {
            if self.unsafe_name {
                return "../unsafe-stage".into();
            }
            let value = format!(".candidate-stage-{}", self.next);
            self.next = self.next.saturating_add(1);
            value
        }
    }

    #[derive(Clone, Debug)]
    struct FakeIo {
        no_replace_supported: bool,
        parent_sync_supported: bool,
        destination_exists: bool,
        collisions_remaining: usize,
        fault: FaultPoint,
        cleanup_fails: bool,
        stage_exists: bool,
        create_calls: usize,
        destination_committed: bool,
    }

    impl FakeIo {
        fn from_case(case: CorpusCase) -> Self {
            Self {
                no_replace_supported: case.no_replace_supported,
                parent_sync_supported: case.parent_sync_supported,
                destination_exists: case.destination_exists,
                collisions_remaining: case.collisions_before_success,
                fault: case.fault,
                cleanup_fails: case.cleanup_fails,
                stage_exists: false,
                create_calls: 0,
                destination_committed: false,
            }
        }
    }

    impl PublicationIo for FakeIo {
        fn supports_no_replace(&self) -> bool {
            self.no_replace_supported
        }

        fn supports_parent_sync(&self) -> bool {
            self.parent_sync_supported
        }

        fn create_stage(
            &mut self,
            _leaf: &str,
            _private_mode: u32,
        ) -> Result<StageCreateDisposition, MechanicalIoFailure> {
            self.create_calls = self.create_calls.saturating_add(1);
            if self.fault == FaultPoint::StageCreate {
                return Err(MechanicalIoFailure);
            }
            if self.collisions_remaining > 0 {
                self.collisions_remaining = self.collisions_remaining.saturating_sub(1);
                return Ok(StageCreateDisposition::AlreadyExists);
            }
            self.stage_exists = true;
            Ok(StageCreateDisposition::Created)
        }

        fn write_all(&mut self, _bytes: &[u8]) -> Result<(), MechanicalIoFailure> {
            if self.fault == FaultPoint::PayloadWrite {
                Err(MechanicalIoFailure)
            } else {
                Ok(())
            }
        }

        fn apply_permissions(&mut self, _mode: u32) -> Result<(), MechanicalIoFailure> {
            if self.fault == FaultPoint::PermissionApply {
                Err(MechanicalIoFailure)
            } else {
                Ok(())
            }
        }

        fn sync_payload(&mut self) -> Result<(), MechanicalIoFailure> {
            if self.fault == FaultPoint::PayloadSync {
                Err(MechanicalIoFailure)
            } else {
                Ok(())
            }
        }

        fn rename_stage(
            &mut self,
            _stage_leaf: &str,
            _destination_leaf: &str,
            replacement: ReplacementMode,
        ) -> Result<RenameDisposition, MechanicalIoFailure> {
            if self.fault == FaultPoint::Rename {
                return Err(MechanicalIoFailure);
            }
            if replacement == ReplacementMode::NoReplace && self.destination_exists {
                return Ok(RenameDisposition::DestinationExists);
            }
            self.stage_exists = false;
            self.destination_committed = true;
            Ok(RenameDisposition::Committed)
        }

        fn sync_parent(&mut self) -> Result<(), MechanicalIoFailure> {
            if self.fault == FaultPoint::ParentSync {
                Err(MechanicalIoFailure)
            } else {
                Ok(())
            }
        }

        fn cleanup_stage(&mut self, _stage_leaf: &str) -> Result<(), MechanicalIoFailure> {
            if self.cleanup_fails {
                return Err(MechanicalIoFailure);
            }
            self.stage_exists = false;
            Ok(())
        }
    }

    fn request(case: CorpusCase) -> PublicationRequest {
        let maximum_payload_bytes = if case.payload_over_bound {
            OVER_BOUND_LIMIT
        } else {
            PAYLOAD_LIMIT
        };
        PublicationRequest {
            destination_leaf: "published.bin".into(),
            payload_bytes: PAYLOAD.len(),
            maximum_payload_bytes,
            collision_attempt_limit: COLLISION_LIMIT,
            final_mode: FINAL_MODE,
            replacement: case.replacement,
            durability: case.durability,
        }
    }

    fn disposition_matches(
        result: &Result<Classification, ClassificationError>,
        expected: ExpectedDisposition,
    ) -> bool {
        match (result, expected) {
            (
                Ok(Classification {
                    disposition: PublicationDisposition::CommittedVisibilityOnly,
                    ..
                }),
                ExpectedDisposition::Visible,
            )
            | (
                Ok(Classification {
                    disposition: PublicationDisposition::CommittedAndParentSynchronized,
                    ..
                }),
                ExpectedDisposition::Durable,
            )
            | (
                Ok(Classification {
                    disposition: PublicationDisposition::DestinationExists { .. },
                    ..
                }),
                ExpectedDisposition::DestinationExists,
            )
            | (
                Ok(Classification {
                    disposition: PublicationDisposition::CommittedDurabilityUnknown,
                    ..
                }),
                ExpectedDisposition::CommittedDurabilityUnknown,
            )
            | (Err(ClassificationError::InvalidRequest(_)), ExpectedDisposition::InvalidRequest)
            | (Err(ClassificationError::InvalidStageLeaf), ExpectedDisposition::InvalidStageName) => {
                true
            }
            (
                Ok(Classification {
                    disposition: PublicationDisposition::NotCommitted { failure, .. },
                    ..
                }),
                ExpectedDisposition::NotCommitted(expected_failure),
            ) => *failure == expected_failure,
            _ => false,
        }
    }

    // r[verify durable_file_publication.validation.corpus]
    #[test]
    fn shared_corpus_preserves_commit_cleanup_and_preflight_state() {
        for case in CONFORMANCE_CORPUS {
            let mut io = FakeIo::from_case(*case);
            let mut names = SequenceNames {
                next: 0,
                unsafe_name: case.unsafe_stage_name,
            };
            let result = publish_one_file(&mut io, &mut names, &request(*case), PAYLOAD);
            assert!(
                disposition_matches(&result, case.expected),
                "case {} produced {result:?}",
                case.id
            );
            if case.cleanup_fails {
                assert!(result.as_ref().is_ok_and(|classification| matches!(
                    classification.disposition,
                    PublicationDisposition::NotCommitted {
                        cleanup: CleanupDisposition::Failed,
                        ..
                    }
                )));
            }
            if matches!(
                case.expected,
                ExpectedDisposition::InvalidRequest
                    | ExpectedDisposition::InvalidStageName
                    | ExpectedDisposition::NotCommitted(
                        crate::core::PrimaryFailure::NoReplaceUnsupported
                            | crate::core::PrimaryFailure::ParentSyncUnsupported
                    )
            ) {
                assert_eq!(
                    io.create_calls, 0,
                    "case {} mutated during preflight",
                    case.id
                );
            }
            if case.expected == ExpectedDisposition::CommittedDurabilityUnknown {
                assert!(io.destination_committed);
            }
        }
    }

    #[test]
    fn malformed_requests_and_impossible_phase_order_mutate_nothing() {
        let malformed = PublicationRequest {
            destination_leaf: "../escape".into(),
            payload_bytes: PAYLOAD.len(),
            maximum_payload_bytes: 0,
            collision_attempt_limit: 0,
            final_mode: INVALID_PERMISSION_MODE,
            replacement: ReplacementMode::Replace,
            durability: DurabilityMode::VisibilityOnly,
        };
        let mut io = FakeIo {
            no_replace_supported: true,
            parent_sync_supported: true,
            destination_exists: false,
            collisions_remaining: 0,
            fault: FaultPoint::None,
            cleanup_fails: false,
            stage_exists: false,
            create_calls: 0,
            destination_committed: false,
        };
        let mut names = SequenceNames {
            next: 0,
            unsafe_name: false,
        };
        let malformed_result = publish_one_file(&mut io, &mut names, &malformed, PAYLOAD);
        assert!(matches!(
            malformed_result,
            Err(ClassificationError::InvalidRequest(_))
        ));
        assert_eq!(io.create_calls, 0);

        let valid = PublicationRequest {
            destination_leaf: "published.bin".into(),
            payload_bytes: PAYLOAD.len(),
            maximum_payload_bytes: PAYLOAD_LIMIT,
            collision_attempt_limit: COLLISION_LIMIT,
            final_mode: FINAL_MODE,
            replacement: ReplacementMode::Replace,
            durability: DurabilityMode::DurabilityRequired,
        };
        let impossible = crate::core::classify_publication(
            &valid,
            &[crate::core::PhaseObservation::ParentSynchronized],
        );
        assert!(matches!(
            impossible,
            Err(ClassificationError::InvalidTransition { .. })
        ));

        let wrong_bytes = PublicationRequest {
            payload_bytes: PAYLOAD.len().saturating_add(1),
            ..valid
        };
        let wrong_bytes_result = publish_one_file(&mut io, &mut names, &wrong_bytes, PAYLOAD);
        assert!(matches!(
            wrong_bytes_result,
            Err(ClassificationError::PayloadByteCountMismatch { .. })
        ));
        assert_eq!(io.create_calls, 0);
    }

    // r[verify durable_file_publication.pilots.mantle]
    // r[verify durable_file_publication.pilots.onixos]
    // r[verify durable_file_publication.validation.losslessness]
    #[test]
    fn both_pilots_retain_exact_mechanical_outcomes() {
        assert_eq!(MANTLE_REVISION.len(), EXPECTED_REVISION_BYTES);
        assert_eq!(ONIXOS_REVISION.len(), EXPECTED_REVISION_BYTES);
        for case in CONFORMANCE_CORPUS {
            let mut io = FakeIo::from_case(*case);
            let mut names = SequenceNames {
                next: 0,
                unsafe_name: case.unsafe_stage_name,
            };
            let result = publish_one_file(&mut io, &mut names, &request(*case), PAYLOAD);
            if let Ok(classification) = result {
                let mantle = map_mantle_outcome(&classification, ExistingContent::Different);
                let onix = map_onix_outcome(&classification);
                assert_eq!(mantle.mechanical, classification.disposition);
                assert_eq!(onix.mechanical, classification.disposition);
            }
        }
    }

    #[test]
    fn mantle_existing_content_interpretation_stays_in_the_adapter() {
        let case = CONFORMANCE_CORPUS
            .iter()
            .find(|case| case.id == "destination-exists")
            .copied();
        assert!(case.is_some());
        if let Some(case) = case {
            let mut io = FakeIo::from_case(case);
            let mut names = SequenceNames {
                next: 0,
                unsafe_name: false,
            };
            let classification = publish_one_file(&mut io, &mut names, &request(case), PAYLOAD);
            assert!(classification.is_ok());
            if let Ok(classification) = classification {
                let identical = map_mantle_outcome(&classification, ExistingContent::Identical);
                let different = map_mantle_outcome(&classification, ExistingContent::Different);
                assert_eq!(
                    identical.product,
                    crate::pilot::MantlePilotDisposition::IdempotentExisting
                );
                assert_eq!(
                    different.product,
                    crate::pilot::MantlePilotDisposition::ImmutableConflict
                );
                assert_eq!(identical.mechanical, different.mechanical);
            }
        }
    }

    #[test]
    fn consumer_request_mappings_preserve_owned_policy_without_product_fields() {
        let mantle = map_mantle_request(&MantlePilotRequest {
            object_leaf: "object.json".into(),
            payload_bytes: PAYLOAD.len(),
            maximum_payload_bytes: PAYLOAD_LIMIT,
            collision_attempt_limit: COLLISION_LIMIT,
            final_mode: FINAL_MODE,
        });
        assert_eq!(mantle.replacement, ReplacementMode::NoReplace);
        assert_eq!(mantle.durability, DurabilityMode::DurabilityRequired);

        let onix = map_onix_request(&OnixPilotRequest {
            destination_leaf: "state.json".into(),
            payload_bytes: PAYLOAD.len(),
            maximum_payload_bytes: PAYLOAD_LIMIT,
            collision_attempt_limit: COLLISION_LIMIT,
            exact_mode: None,
            existing_mode: Some(FINAL_MODE),
            create_mode: PRIVATE_MODE_FALLBACK,
            overwrite: OnixOverwritePolicy::Refuse,
            durability_required: false,
        });
        assert_eq!(onix.final_mode, FINAL_MODE);
        assert_eq!(onix.replacement, ReplacementMode::NoReplace);
        assert_eq!(onix.durability, DurabilityMode::VisibilityOnly);
    }

    const PRIVATE_MODE_FALLBACK: u32 = 0o600;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    mod linux {
        use std::fs::File;
        use std::os::unix::fs::PermissionsExt;

        use super::*;
        use crate::shell::LinuxDirectory;

        #[test]
        fn real_open_directory_supports_replace_no_replace_permissions_and_parent_sync() {
            let directory = tempfile::tempdir();
            assert!(directory.is_ok());
            let directory = directory.unwrap_or_else(|error| unreachable!("fixture: {error}"));
            let opened = File::open(directory.path());
            assert!(opened.is_ok());
            let opened = opened.unwrap_or_else(|error| unreachable!("fixture: {error}"));
            let parent = LinuxDirectory::from_open_directory(opened);
            assert!(parent.is_ok());
            let mut parent = parent.unwrap_or_else(|error| unreachable!("fixture: {error}"));
            let mut names = SequenceNames {
                next: 0,
                unsafe_name: false,
            };
            let durable_request = PublicationRequest {
                destination_leaf: "published.bin".into(),
                payload_bytes: PAYLOAD.len(),
                maximum_payload_bytes: PAYLOAD_LIMIT,
                collision_attempt_limit: COLLISION_LIMIT,
                final_mode: FINAL_MODE,
                replacement: ReplacementMode::NoReplace,
                durability: DurabilityMode::DurabilityRequired,
            };
            let published = publish_one_file(&mut parent, &mut names, &durable_request, PAYLOAD);
            assert!(published.as_ref().is_ok_and(|classification| matches!(
                classification.disposition,
                PublicationDisposition::CommittedAndParentSynchronized
            )));
            let bytes = std::fs::read(directory.path().join("published.bin"));
            assert!(
                bytes
                    .as_ref()
                    .is_ok_and(|bytes| bytes.as_slice() == PAYLOAD)
            );
            let metadata = std::fs::metadata(directory.path().join("published.bin"));
            assert!(metadata.as_ref().is_ok_and(|metadata| {
                metadata.permissions().mode() & TEST_PERMISSION_MASK == FINAL_MODE
            }));

            let existing = publish_one_file(&mut parent, &mut names, &durable_request, PAYLOAD);
            assert!(existing.as_ref().is_ok_and(|classification| matches!(
                classification.disposition,
                PublicationDisposition::DestinationExists {
                    cleanup: CleanupDisposition::Removed,
                }
            )));
            let preserved = std::fs::read(directory.path().join("published.bin"));
            assert!(
                preserved
                    .as_ref()
                    .is_ok_and(|bytes| bytes.as_slice() == PAYLOAD)
            );
        }
    }
}
