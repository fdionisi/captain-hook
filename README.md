<div align="center">
  <h1>Captain Hook 🪝</h1>
  <p>
    <b>
      Cross-platform, modern and native git hooks.
    </b>
  </p>
  <sub>
    Built on top of
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

## Notes

- Upgrading from `0.1.*` to `0.2.0` involves breaking changes.
  <details>

  Version `0.2.0` is built on top of [husky](https://github.com/typicode/husky) and follows its files and directories naming convention.

  During the `install` command, by default Captain Hook places all files in the `.husky` directory rather than the `.hooks`. The boot script for each hook is now called `_/husky.sh` instead of `_/captain-hook.sh`. Consequently, during the `add` command, the new hook will be prefixed with `. "$(dirname "$0")/_/husky.sh"`.
  </details>

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

_Captain Hook_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
