# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.4](https://github.com/JasterV/test-context/compare/v0.5.3...v0.5.4) - 2025-11-10

### Other

- internal refactor & update docs ([#64](https://github.com/JasterV/test-context/pull/64))

## [0.5.3](https://github.com/JasterV/test-context/compare/v0.5.2...v0.5.3) - 2025-11-06

### Fixed

- Taking mutable ownership when skipping the teardown doesn't work ([#60](https://github.com/JasterV/test-context/pull/60))

## [0.5.2](https://github.com/JasterV/test-context/compare/v0.5.1...v0.5.2) - 2025-11-06

### Added

- make it so immutable references & full ownership can be taken depending on context ([#58](https://github.com/JasterV/test-context/pull/58))

### Other

- update CHANGELOGS

## [0.5.1](https://github.com/JasterV/test-context/compare/v0.5.0...v0.5.1) - 2025-11-04

### Fixed

- Regression in version 0.5.0 ([#55](https://github.com/JasterV/test-context/pull/55))

### Added
- Add a CHANGELOG by @JasterV
- Add a rust-toolchain.toml by @JasterV

## [0.5.0] - 2025-10-29

### Added
- Added the ability to work with rstest by @killpop3770 in [#51](https://github.com/JasterV/test-context/pull/51)

### Changed
- Explain why #[test_context] must come before #[tokio::test]; clarify attribute expansion order by @vilinski in [#49](https://github.com/JasterV/test-context/pull/49)
- Update README.md by @JasterV in [#47](https://github.com/JasterV/test-context/pull/47)
- Update dependabot & add release-plz workflow by @JasterV

### Fixed
- Manual drop on tests by @JasterV

### New Contributors
* @killpop3770 made their first contribution in [#51](https://github.com/JasterV/test-context/pull/51)
* @vilinski made their first contribution in [#49](https://github.com/JasterV/test-context/pull/49)

## [0.4.0] - 2025-01-27

### Changed
- Upgrade to 0.4.0 by @JasterV in [#46](https://github.com/JasterV/test-context/pull/46)
- Support generic types in test_context macro by @JasterV in [#45](https://github.com/JasterV/test-context/pull/45)
- Support generic types in test_context macro by @rookiecj in [#44](https://github.com/JasterV/test-context/pull/44)

### New Contributors
* @rookiecj made their first contribution in [#44](https://github.com/JasterV/test-context/pull/44)

## [0.3.0] - 2024-02-27

### Added
- Add support for the skip_teardown keyword by @JasterV in [#40](https://github.com/JasterV/test-context/pull/40)

### Changed
- Upgrade to 0.3.0 by @JasterV in [#41](https://github.com/JasterV/test-context/pull/41)
- Merge pull request #39 from JasterV/refactor/test-context-macro by @JasterV in [#39](https://github.com/JasterV/test-context/pull/39)
- Clean up the macro implementation by @JasterV

## [0.2.0] - 2024-02-26

### Changed
- Merge pull request #38 from JasterV/chore/upgrade-to-0.2.0-version by @JasterV in [#38](https://github.com/JasterV/test-context/pull/38)
- Upgrade to 0.2.0 version by @JasterV
- Merge pull request #37 from JasterV/refactor/remove-async-trait-support by @JasterV in [#37](https://github.com/JasterV/test-context/pull/37)

### Removed
- Remove support for async-trait crate by @JasterV

## [0.1.6] - 2024-02-26

### Changed
- Merge pull request #36 from JasterV/chore/update-to-0-1-6 by @JasterV in [#36](https://github.com/JasterV/test-context/pull/36)
- Update to 0.1.6 by @JasterV
- Merge pull request #35 from JasterV/chore/update-patch-version by @JasterV in [#35](https://github.com/JasterV/test-context/pull/35)
- Update patch version by @JasterV
- Merge pull request #34 from JasterV/refactor/cargo-workspace by @JasterV in [#34](https://github.com/JasterV/test-context/pull/34)
- Update workspace properties by @JasterV
- Restructure the workspace members organization by @JasterV
- Merge pull request #33 from JasterV/chore/update-package-information by @JasterV in [#33](https://github.com/JasterV/test-context/pull/33)
- Update crate information by @JasterV
- Merge pull request #31 from JasterV/fix/doc-tests-clippy-error by @JasterV in [#31](https://github.com/JasterV/test-context/pull/31)
- Merge pull request #29 from JasterV/dependabot/cargo/syn-tw-2 by @JasterV in [#29](https://github.com/JasterV/test-context/pull/29)
- Merge branch 'main' into dependabot/cargo/syn-tw-2 by @JasterV
- Merge pull request #23 from yotamofek/syn2 by @JasterV in [#23](https://github.com/JasterV/test-context/pull/23)
- Merge branch 'main' into syn2 by @JasterV
- Merge branch 'main' into syn2 by @JasterV
- Update syn by @yotamofek
- Update syn requirement from ^1 to ^2 by @dependabot[bot]
- Merge pull request #30 from JasterV/chore/update-to-2021-edition by @JasterV in [#30](https://github.com/JasterV/test-context/pull/30)
- Update to 2021 rust edition by @JasterV
- Merge pull request #28 from JasterV/JasterV-patch-1 by @JasterV in [#28](https://github.com/JasterV/test-context/pull/28)
- Create dependabot.yml by @JasterV
- Merge pull request #27 from JasterV/JasterV-patch-1 by @JasterV in [#27](https://github.com/JasterV/test-context/pull/27)
- Update README.md by @JasterV
- Update README.md by @markhildreth
- Merge pull request #19 from SomeoneToIgnore/patch-1 by @markhildreth in [#19](https://github.com/JasterV/test-context/pull/19)
- Use proper World capitalization in the main example by @SomeoneToIgnore

### Fixed
- Clippy unit test on doctests warnings by @JasterV

### New Contributors
* @JasterV made their first contribution in [#36](https://github.com/JasterV/test-context/pull/36)
* @dependabot[bot] made their first contribution
* @yotamofek made their first contribution
* @SomeoneToIgnore made their first contribution

## [0.1.4] - 2022-07-19

### Changed
- Merge pull request #17 from markhildreth/bump-to-0.1.4 by @markhildreth in [#17](https://github.com/JasterV/test-context/pull/17)
- Bumped to 0.1.4 by @markhildreth
- Merge pull request #16 from markhildreth/use-original-argument-list by @markhildreth in [#16](https://github.com/JasterV/test-context/pull/16)
- Copy original argument list directly from input token stream. by @markhildreth

## [0.1.3] - 2021-02-27

### Added
- Add basic impl TestContext for AsyncTestContext by @Shadow53

### Changed
- Merge pull request #11 from markhildreth/bump-to-0.1.3 by @markhildreth in [#11](https://github.com/JasterV/test-context/pull/11)
- Bumped to v0.1.3 by @markhildreth
- Merge pull request #10 from markhildreth/add-sync-async-impl-docs by @markhildreth in [#10](https://github.com/JasterV/test-context/pull/10)
- Documented behavior of AsyncContext with normal function. by @markhildreth
- Merge pull request #9 from Shadow53/impl-sync-for-async by @markhildreth in [#9](https://github.com/JasterV/test-context/pull/9)
- Run cargo fmt by @Shadow53

### New Contributors
* @Shadow53 made their first contribution

## [0.1.2] - 2021-02-07

### Added
- Added badges by @markhildreth

### Changed
- Merge pull request #7 from markhildreth/bump-to-0.1.2 by @markhildreth in [#7](https://github.com/JasterV/test-context/pull/7)
- Bumped to version v0.1.2 by @markhildreth
- Merge pull request #6 from markhildreth/badges by @markhildreth in [#6](https://github.com/JasterV/test-context/pull/6)
- Merge pull request #5 from markhildreth/fixed-typo by @markhildreth in [#5](https://github.com/JasterV/test-context/pull/5)
- Merge branch 'main' into fixed-typo by @markhildreth
- Merge pull request #4 from markhildreth/future-dependency by @markhildreth in [#4](https://github.com/JasterV/test-context/pull/4)
- Forced futures to be imported through test_context crate. by @markhildreth
- Fixed typo in documentation by @markhildreth

## [0.1.1] - 2021-01-18

### Changed
- Merge pull request #3 from markhildreth/v0.1.1 by @markhildreth in [#3](https://github.com/JasterV/test-context/pull/3)
- Bumped to version v0.1.1 by @markhildreth
- Merge pull request #2 from markhildreth/github-workflows by @markhildreth in [#2](https://github.com/JasterV/test-context/pull/2)
- Updated github ci.yml by @markhildreth
- Merge pull request #1 from markhildreth/allow-returns by @markhildreth in [#1](https://github.com/JasterV/test-context/pull/1)
- Allow return of wrapped functions value. by @markhildreth

## [0.1] - 2021-01-18

### Changed
- Initial version by @markhildreth
- Initial commit by @markhildreth

### New Contributors
* @markhildreth made their first contribution

[unreleased]: https://github.com/JasterV/test-context/compare/test-context-v0.5.0...HEAD
[test-context-v0.5.0]: https://github.com/JasterV/test-context/compare/v0.5.0...test-context-v0.5.0
[0.5.0]: https://github.com/JasterV/test-context/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/JasterV/test-context/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/JasterV/test-context/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/JasterV/test-context/compare/v0.1.6...v0.2.0
[0.1.6]: https://github.com/JasterV/test-context/compare/v0.1.4...v0.1.6
[0.1.4]: https://github.com/JasterV/test-context/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/JasterV/test-context/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/JasterV/test-context/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/JasterV/test-context/compare/v0.1...v0.1.1

<!-- generated by git-cliff -->
