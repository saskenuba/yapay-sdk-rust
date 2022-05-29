# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.1 (2022-05-29)

### Refactor

 - <csr-id-bea195316e37402674108bf2c7b95101d7c07662/> removed print calls that were panicking on multi-thread..
   executors, and replaced with tracing dep

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - removed print calls that were panicking on multi-thread.. ([`bea1953`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/bea195316e37402674108bf2c7b95101d7c07662))
</details>

## v0.3.0 (2022-05-28)

<csr-id-9e66ae0066fe1789d2f8b5a52a83996c0ccc4f4d/>
<csr-id-c2f8fa7d808a09dbcba4f6cfe9878759ed930ea9/>

### New Features

 - <csr-id-3daa7dffe991c1143b2e0b77e7bf49602438ab77/> implemented TryFrom<Vec<u8>> for `YapayWebhook` to allow it...
   to be extracted from raw payloads.

### Other

 - <csr-id-9e66ae0066fe1789d2f8b5a52a83996c0ccc4f4d/> removed unused deps and traits


### Chore (BREAKING)

 - <csr-id-c2f8fa7d808a09dbcba4f6cfe9878759ed930ea9/> removed pub visibilty to allow proper re-export of structs


### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release yapay-sdk-rust v0.3.0 ([`5d75b68`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/5d75b6810d1d0d6c9369179bfeb12acbd7017205))
    - removed pub visibilty to allow proper re-export of structs ([`c2f8fa7`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/c2f8fa7d808a09dbcba4f6cfe9878759ed930ea9))
    - implemented TryFrom<Vec<u8>> for `YapayWebhook` to allow it... ([`3daa7df`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/3daa7dffe991c1143b2e0b77e7bf49602438ab77))
    - removed unused deps and traits ([`9e66ae0`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/9e66ae0066fe1789d2f8b5a52a83996c0ccc4f4d))
</details>

## v0.2.0 (2022-05-28)

### Documentation

 - <csr-id-6636633ec54386822205b3ce1135c4a1d14d5743/> updated README
 - <csr-id-b8bdaedef278163ffe4115d896d2c69e6f4b18c3/> updated docs/readme

### New Features

 - <csr-id-3d1ffe0e1f1e1cae75fe713a7909334f7bbe30fc/> Added `set_process_url` builder fn
 - <csr-id-abbb13b5acbe107767ca45b4abba3f2a7782e71c/> added TransactionStatus enum, and replaced it on webhook

### Bug Fixes

 - <csr-id-c0e57a7ea452bea6580e8e16169071c9504c8268/> clippy lints

### New Features (BREAKING)

 - <csr-id-08bb5f2bbbb497fc15b4a9ebdef69498a37db536/> docs, fixes +
   * Added tests
* Updated some functions docs;
* Checkout can handle available_payment_methods correctly now;
* Renamed Webhook to YapayWebhook;

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release yapay-sdk-rust v0.2.0 ([`353a644`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/353a6440e47b857a7e60a2e178fa991f790423ad))
    - updated README ([`6636633`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/6636633ec54386822205b3ce1135c4a1d14d5743))
    - Added `set_process_url` builder fn ([`3d1ffe0`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/3d1ffe0e1f1e1cae75fe713a7909334f7bbe30fc))
    - added TransactionStatus enum, and replaced it on webhook ([`abbb13b`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/abbb13b5acbe107767ca45b4abba3f2a7782e71c))
    - clippy lints ([`c0e57a7`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/c0e57a7ea452bea6580e8e16169071c9504c8268))
    - docs, fixes + ([`08bb5f2`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/08bb5f2bbbb497fc15b4a9ebdef69498a37db536))
    - updated docs/readme ([`b8bdaed`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/b8bdaedef278163ffe4115d896d2c69e6f4b18c3))
</details>

## v0.1.2 (2022-05-25)

<csr-id-d3b4a8a366abad541e423d1db910bdc0768833ed/>

### New Features

 - <csr-id-21e24a1437c0091e20dfc66c74b7c37572017191/> added webhook strucutre, to be used on extractors

### Refactor

 - <csr-id-d3b4a8a366abad541e423d1db910bdc0768833ed/> re-added tests to credit_card payment, clippy lints


### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release yapay-sdk-rust v0.1.2 ([`7c87eea`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/7c87eea46344385a0b19e8397a60eb18e0cd215b))
    - re-added tests to credit_card payment, clippy lints ([`d3b4a8a`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/d3b4a8a366abad541e423d1db910bdc0768833ed))
    - added webhook strucutre, to be used on extractors ([`21e24a1`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/21e24a1437c0091e20dfc66c74b7c37572017191))
</details>

## v0.1.1 (2022-05-24)

### New Features

 - <csr-id-2955828f5edd52b378ff6558d717e36d52300787/> added create checkout endpoint, refactor
   Added `new` fns to YapayProduct, YapayCustomer, YapayTransaction;
   Added `create_checkout` endpoint with custom preferences;
   Removed unused parameters on multiple fns
   
   BREAKING CHANGE

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release yapay-sdk-rust v0.1.1 ([`9133567`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/913356719f57f87c9343f712e79f1cad9a49d7f2))
    - added create checkout endpoint, refactor ([`2955828`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/2955828f5edd52b378ff6558d717e36d52300787))
</details>

## v0.1.0 (2022-05-20)

<csr-id-9fcd1e0353cb804b29e39de1fe379342db3a9eda/>
<csr-id-db5e7f50f89b888ef078042486d6ee661e13250f/>
<csr-id-64d0024ab01fbdbe158ad0451f0b68cdc32101d8/>

### Chore

 - <csr-id-9fcd1e0353cb804b29e39de1fe379342db3a9eda/> added .envrc to gitignore

 - <csr-id-db5e7f50f89b888ef078042486d6ee661e13250f/> first commit


### Chore

 - <csr-id-64d0024ab01fbdbe158ad0451f0b68cdc32101d8/> added .envrc to gitignore


### Documentation

 - <csr-id-18a6e9c3851f1c5e8b4ca4ba7ac906018d9c7fc0/> fixed wrong name
 - <csr-id-8ec1827e0534c5604e10d969e70e216eafd230b9/> fixed wrong name, added CHANGELOG

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release yapay-sdk-rust v0.1.0 ([`0dca689`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/0dca689c6082883c4e901ade5be53d1296e5f5a4))
    - added .envrc to gitignore ([`64d0024`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/64d0024ab01fbdbe158ad0451f0b68cdc32101d8))
    - fixed wrong name, added CHANGELOG ([`8ec1827`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/8ec1827e0534c5604e10d969e70e216eafd230b9))
    - first commit ([`db5e7f5`](https://github.comgit//saskenuba/yapay-sdk-rust/commit/db5e7f50f89b888ef078042486d6ee661e13250f))
</details>

