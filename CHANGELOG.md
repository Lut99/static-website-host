# CHANGELOG
This file keeps track of notable changes to the Static Website Host codebase.

Note that this project uses [semantic versioning](https://semver.org). Breaking changes are indicated with **(BREAKING)**.


## v0.1.0 - TODO
Initial release!

### Added
- The main `static-website-host` binary.
- Added the www-path, which responds the target file to any GET-request under `/`.
    - It automatically adds `index.html` when a directory is referred.
- A custom 404 not-found page can be set.
- A Docker (Compose) file for building the binary as a container.
