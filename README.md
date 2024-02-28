<div align="center">
<h1>GHASToolkit in Rust</h1>

[![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)][github]
[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/GeekMasher/ghastoolkit-rs/build.yml?style=for-the-badge)](https://github.com/GeekMasher/ghastoolkit-rs/actions/workflows/python-package.yml?query=branch%3Amain)
[![GitHub Issues](https://img.shields.io/github/issues/GeekMasher/ghastoolkit-rs?style=for-the-badge)][github-issues]
[![GitHub Stars](https://img.shields.io/github/stars/GeekMasher/ghastoolkit-rs?style=for-the-badge)][github]
[![Crates.io Version](https://img.shields.io/crates/v/ghastoolkit?style=for-the-badge)][crates-io]
[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/ghastoolkit?style=for-the-badge)][crates-io]
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)][license]

</div>

## Overview

This is the [GitHub Advanced Security][advanced-security] (GHAS) Toolkit in [Rust][rust-lang]. This toolkit is designed to help developers and security researchers to interact with the GitHub Advanced Security API.

## ✨ Features

- [Core GHAS Library][code-core]
  - [Documentation][docs]
  - GitHub Cloud and Enterprise Server support
  - API Support
    - [x] [Code Scanning][github-code-scanning]
    - [ ] 👷 [Secret Scanning][github-secret-scanning]
    - [ ] 👷 [Supply Chain][github-supplychain]
      - [ ] 👷 [Dependabot][github-dependabot] (Security Alerts)
      - [ ] 👷 [Dependency Graph][github-depgraph] (SCA / SBOMs)
      - [ ] 👷 [Security Advisories][github-advisories]
- [CLI Tool][code-cli]

## Usage

To use the library in your project, add it to your project using the following command:

```bash
cargo add ghastoolkit
```

## Install CLI

You can install the CLI many different ways but the easiest way is the following:

```bash
cargo install ghastoolkit-cli
```

### From GitHub

```bash
cargo install --git https://github.com/GeekMasher/ghastoolkit-rs
```

## Maintainers / Contributors

- [@GeekMasher](https://github.com/GeekMasher) - Author / Core Maintainer

## Support

Please create [GitHub Issues][github-issues] if there are bugs or feature requests.

This project uses [Sematic Versioning (v2)](https://semver.org/) and with major releases, breaking changes will occur.

## License

This project is licensed under the terms of the MIT open source license.
Please refer to [MIT][license] for the full terms.

<!-- Resources -->

[license]: ./LICENSE
[crates-io]: https://crates.io/crates/ghastoolkit
[docs]: https://docs.rs/ghastoolkit/latest/ghastoolkit/
[rust-lang]: https://www.rust-lang.org/
[advanced-security]: https://github.com/features/security
[code-core]: https://github.com/GeekMasher/ghastoolkit-rs/tree/main/core
[code-cli]: https://github.com/GeekMasher/ghastoolkit-rs/tree/main/cli
[github]: https://github.com/geekmasher/ghastoolkit-rs
[github-issues]: https://github.com/geekmasher/ghastoolkit-rs/issues
[github-code-scanning]: https://docs.github.com/en/code-security/code-scanning/introduction-to-code-scanning/about-code-scanning
[github-secret-scanning]: https://docs.github.com/en/code-security/secret-scanning/about-secret-scanning
[github-supplychain]: https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/about-supply-chain-security
[github-dependabot]: https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
[github-depgraph]: https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/about-the-dependency-graph
[github-advisories]: https://docs.github.com/en/code-security/security-advisories/working-with-global-security-advisories-from-the-github-advisory-database/about-the-github-advisory-database

