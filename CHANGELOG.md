# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.9.0] - 2016-05-19
### Added
- `get_protein_mass` to prot module: solution for `Calculating Protein Mass`

## [0.8.1] - 2016-04-12
### Fixed
- Correct work with `\n` symbol during `Inferring mRNA from Protein`

## [0.8.0] - 2016-04-12
### Added
- `get_number_of_rna_from_protein` to prot module: solution for `Inferring mRNA from Protein`

## [0.7.0] - 2016-03-18
### Added
- Dependency to `num` cargo
- `recurrence_relation_with_stop` to fib module: solution for `Mortal Fibonacci Rabbits`
### Changed
- `rosalind::fib::recurrence_relation` - return type changed to `RosalindResult<BigUint>`

## [0.6.1] - 2016-02-24
### Added
- documentation for `RosalindResult`
### Changed
- `RosalindResult` simplified - lifetime parameter removed

## [0.6.0] - 2016-02-23
### Added
- `gc` module: solution for `Computing GC Content`

## [0.5.1] - 2016-02-20
### Added
- documentation section into Cargo.toml
### Changed
- `Result` renamed into `RosalindResult`

## [0.5.0] - 2016-02-09
### Added
- `subs` module: solution for `Finding a Motif in DNA`

## [0.4.1] - 2016-02-04
### Added
- Unified type `Result`
### Changed
- All modules return `Result` type

## [0.4.0] - 2016-02-03
### Added
- `hamm` module: solution for `Counting Point Mutations`

## [0.3.1] - 2016-01-25
### Fixed
- Ignore `\n` symbol during protein translation

## [0.3.0] - 2016-01-25
### Added
- `prot` module: solution for `Translating RNA into Protein`

## [0.2.1] - 2015-12-23
### Fixed
- Fix overflow in `fib::recurrence_relation`

## [0.2.0] - 2015-12-23
### Added
- `fib` module: solution for `Rabbits and Recurrence Relations`

## [0.1.0] - 2015-12-23
### Added
- `revc` module: solution for `Complementing a Strand of DNA`

## [0.0.2] - 2015-12-23
### Changed
- Unit tests moved from `lib.rs` to modules

## [0.0.1] - 2015-12-21
### Added
- `rna` module: solution for `Counting DNA nucleotides`
- `dna` module: solution for `Transcribind DNA into RNA`
