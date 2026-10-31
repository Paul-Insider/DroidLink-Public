# Third-party boundary

This public Rust crate has no external Cargo dependencies and does not bundle
scrcpy, Android Debug Bridge, device drivers, codecs or application assets.
The Rust standard library and toolchain retain their respective licenses.

DROIDLINK's product work may interact with scrcpy and Android Debug Bridge.
Those projects belong to their respective authors; this repository does not
relicense their code. Any future redistribution must retain the applicable
upstream licenses and notices.

References:
- scrcpy: https://github.com/Genymobile/scrcpy
- Android Debug Bridge: https://developer.android.com/tools/adb
- Rust: https://www.rust-lang.org/policies/licenses

GitHub Actions configuration references external actions/tools; their licenses
and service terms remain separate from this repository's evaluation license.
