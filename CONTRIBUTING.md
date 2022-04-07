# Contribution guidelines

We sincerely appreciate any and all community contributions. `Lucy` is, and always will be, a fully
open-sourced project. As such, your contribution(s) are invaluable.

To ensure that your contribution is accepted, we ask that you please open a ticket that includes a
brief description of all change(s) that you would like to introduce.

This process allows for any necessary discussions to take place and also helps to clarify the "what"
, "why" and "how" your changes can be integrated with the existing codebase.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/dark-fusion/lucy/issues), please check that it has not already
been reported by searching for some related keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/dark-fusion/lucy/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections, depending on the types of
changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/dark-fusion/lucy
cd lucy
cargo test
```

### Useful Commands

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
