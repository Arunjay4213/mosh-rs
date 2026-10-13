# Security policy

## Supported versions

mosh-rs has not made a release yet, so no version receives security fixes.
This table will list supported versions from v0.1.0 on.

| Version | Supported |
| --- | --- |
| none released | not applicable |

## Reporting a vulnerability

Please do not open a public issue, discussion, or pull request for a security problem.

Report it privately through GitHub private vulnerability reporting.
The repository is still private, and GitHub offers this form only on public repositories, so it is not available yet; it is to be turned on when the repository goes public.
The steps will then be:

1. Go to the repository's [Security tab](https://github.com/Arunjay4213/mosh-rs/security).
2. Click **Report a vulnerability**.
3. Describe the issue, the affected version or commit, and steps to reproduce.

We aim to acknowledge every report within 72 hours, and to agree on a fix and disclosure date with you after that.
You will be credited in the release notes unless you ask not to be.

## Cryptography

mosh-rs uses AES-128 in OCB3 mode (RFC 7253), ported from mosh master (decd9b7, March 2026) and wire-compatible with mosh 1.4.0, so that it interoperates with existing `mosh-server` and `mosh-client` installs.
The key, nonce layout, and packet format must match upstream exactly, and this is checked byte for byte against upstream's own code by the `ref-crypto` oracle.
Any difference between mosh-rs and upstream in key handling, nonce construction, encryption, authentication, or replay handling is treated as a vulnerability by definition, even if it looks harmless.

## Scope

In scope: the `mosh`, `mosh-client`, and `mosh-server` programs built from this repository.
Vulnerabilities in upstream mosh's protocol design itself affect upstream too; please also report those to the mosh maintainers through [mosh.org](https://mosh.org).
