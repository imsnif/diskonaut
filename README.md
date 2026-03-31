# diskonaut

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Display disk usage in the terminal using treemaps

Given a path on your hard-drive (which could also be the root path, eg. `/`). `diskonaut` scans it and indexes its
metadata to memory so that you can explore its contents (even while still scanning).

Once completed, you can navigate through subfolders, getting a visual treemap representation of what's taking up your
disk space. You can delete files or folders as well and `diskonaut` will track how much space you've freed up in this
session.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

Install with cargo or download the latest version from the GitHub Releases. Note that the  precompiled binaries are
optimized for small binary size (see `.goreleaser.yaml` for how exactly).

```sh
git clone https://github.com/kfkonrad/diskonaut.git
cd diskonaut
cargo install --path .
```

## Usage

To scan the current folder run:

```sh
diskonaut
```

You can specify any folder to scan:

```sh
diskonaut ~
```

To show file sizes rather than block usage on disk run `diskonaut -a`. To skip delete confirmation run `diskonaut -d`

## Maintainers

[@kfkonrad](https://github.com/kfkonrad)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2026 Kevin F. Konrad
