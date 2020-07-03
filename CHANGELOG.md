# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]

## [0.6.0] - 2020-07-03

### Added
* Add a visual indication when running as root (https://github.com/imsnif/diskonaut/pull/57) - [@c3st7n](https://github.com/c3st7n)
* Change delete key to DELETE (https://github.com/imsnif/diskonaut/pull/59) - [@maxheyer](https://github.com/maxheyer)

## [0.5.0] - 2020-06-27

### Added
* Add an "Are you sure you want to quit?" modal (https://github.com/imsnif/diskonaut/pull/44) - [@mhdmhsni](https://github.com/mhdmhsni)

### Fixed
* Fix some small_files rendering edge-cases (https://github.com/imsnif/diskonaut/pull/55) - [@imsnif](https://github.com/imsnif)

## [0.4.0] - 2020-06-26

### Added
* Support emacs keybindings (https://github.com/imsnif/diskonaut/pull/40) - [@redzic](https://github.com/redzic)
* Make enter select largest folder if nothing is selected (https://github.com/imsnif/diskonaut/pull/45) - [@redzic](https://github.com/redzic)
* Keep track of tile selection in previous folder (https://github.com/imsnif/diskonaut/pull/53) - [@therealprof](https://github.com/therealprof)

### Fixed
* Do not scan in parallel when running tests (https://github.com/imsnif/diskonaut/pull/43) - [@redzic](https://github.com/redzic)
* Prevent crashes for multibyte characters on grid (https://github.com/imsnif/diskonaut/pull/51) - [@goto-bus-stop](https://github.com/goto-bus-stop)
* Show quit shortcut in legend (https://github.com/imsnif/diskonaut/pull/46) - [@olehs0](https://github.com/olehs0)

## [0.3.0] - 2020-06-21

### Fixed
* Remove unneeded dev dependency (https://github.com/imsnif/diskonaut/pull/35) - [@ignatenkobrain](https://github.com/ignatenkobrain)
* Improve scanning speed (https://github.com/imsnif/diskonaut/pull/38) - [@imsnif](https://github.com/imsnif)
* Refactor movement methods (https://github.com/imsnif/diskonaut/pull/31) - [@phimuemue](https://github.com/phimuemue)

## [0.2.0] - 2020-06-18

### Fixed
* Cross platform file size calculation (https://github.com/imsnif/diskonaut/pull/28) - [@Freaky](https://github.com/Freaky)
* Bumped insta dependency to 0.16.0, bumped cargo-insta dependency to 0.16.0 (https://github.com/imsnif/diskonaut/pull/25) - [@tim77](https://github.com/tim77)
* Bumped tui dependency to 0.9 (https://github.com/imsnif/diskonaut/pull/30) - [@silwol](https://github.com/silwol)

## [0.1.0] - 2020-06-17

Initial release with all the things.
