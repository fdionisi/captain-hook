<div align="center">
  <h1>Captain Hook 🪝</h1>
  <p>
    <b>
      Cross-platform, modern and native git hooks.
    </b>
  </p>
  <sub>
    Inspired by
    <a href="https://github.com/typicode/husky" target="_blank">husky</a>.
  </sub>
</div>

## Abstract

[Husky](https://github.com/typicode/husky) is a great tool, but it requires
[Node.js](https://github.com/nodejs/node) to be installed. This requirement may
prevent developers from using it on a project that does not use Node.js. This
implementation aims to provide an executable that can be used in any git project
despite the language or runtime available. The coding philosophy is to avoid
dependencies to keep the size of the binary to a minimum, making it a CLI
application that is accessible to anyone.

## Prerequisites

To run `captain-hook` you need to have [git](https://git-scm.com/ "git")
installed on your machine.

## Quick Install

It is possible to install `captain-hook` in two flavours:

- With Shell:
  ```sh
  sh -c "$(curl -fsSL https://captain-hook.sh/install)"
  ```

  To update the Captain Hook itself, rerun the above script. It will replace the
  current version with the latest one.

- With Cargo:
  ```sh
  cargo install captain-hook
  ```

  To update the Captain Hook with Cargo, remember to force re-installing the
  binary.

  ```sh
  cargo install -f captain-hook
  ```

## License

_Captain Hook_ is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
