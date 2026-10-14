# Extraction decision

## Success contract

The extraction succeeds only if one pure core can validate shared ordered chunk geometry while preserving algorithm and digest identity differences. It must support Mantle's FastCDC/BLAKE3 profile, Molten's fixed-size/BLAKE3 profile, and ChaosControl's externally segmented SHA-256 parts without declaring those formats equivalent.

False completion includes a universal serialized manifest, an implicit digest default, a generic “content hash” that erases BLAKE3 versus SHA-256, a FastCDC implementation without a version/gear-table owner, format-specific paths or evidence fields, or a geometry result promoted into transfer/integrity proof.

## Approach registry

| Family | Mechanism | Result |
|---|---|---|
| Pure geometry core | Validate ordered lengths under explicit fixed, FastCDC-bound, or external-parts modes; tag digest algorithms separately | Selected |
| Universal wire manifest | Normalize all three consumers into one serialized schema | Rejected: it would transfer schema authority and erase real identity/algorithm differences |
| Shared chunker implementation | Extract one concrete fixed or FastCDC splitter | Rejected for this phase: fixed and content-defined boundaries are materially different, and FastCDC version/gear-table identity remains consumer-owned |
| No extraction | Keep size summation, fixed-final checks, overflow handling, and digest tags in every consumer | Rejected: the deterministic geometry state machine is duplicated and testable without product policy |

## Adversarial audit

The selected core is checked against:

- empty artifacts allowed versus rejected;
- non-empty artifacts with no chunks and empty artifacts with chunks;
- zero-size fixed profiles and malformed FastCDC bounds;
- wrong non-final fixed sizes and oversized final chunks;
- FastCDC non-final chunks below minimum and any chunk above maximum;
- consumer total-byte and chunk-count bounds;
- checked-sum overflow and declared/reconstructed mismatch;
- derived contiguous offsets;
- identical 32-byte values tagged as BLAKE3 versus SHA-256;
- malformed digest byte lengths.

The core does not parse hex, hash bytes, validate paths, load parts, or recompute whole-artifact digests. Those checks remain necessary in consumers.

## Non-claims

A validated geometry does not prove that bytes exist, chunk hashes match, external part order is authentic, chunks reconstruct the intended artifact, FastCDC used a particular algorithm version, transforms are safe, a transfer completed, or any consumer evidence/readiness claim is true.
