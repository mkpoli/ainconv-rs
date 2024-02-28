# ainconv - Ainu language script converter

![Crates.io](https://img.shields.io/crates/v/ainconv)
![GitHub issues](https://img.shields.io/github/issues/mkpoli/ainconv-rs)
![GitHub](https://img.shields.io/github/license/mkpoli/ainconv-rs)

## Overview

This cargo crate provides a comprehensive set of functions for converting text between different writing systems of the [Ainu language](https://en.wikipedia.org/wiki/Ainu_language).

Currently, Latin (Romanization), Katakana and Cyrillic scripts are supported. We are also planning to convert between different romanization systems and Katakana variants. Currently only the more adopted version of Latin script and lossy Katakana script are supported.

Sentence conversion is planned to be supported in the future. For now, only well-formed single word is accepted. The converted string are always in lower case.

### Important Note

Conversion between Latin and Cyrillic script are lossless, however, conversion between Katakana and other scripts are lossy. This means that converting from Katakana to other scripts and then back to Katakana may not give the original string and the result may be ambiguous or even incorrect.

This is because the Katakana script used broadly for the Ainu language is intrinsically ambiguous. For example, it does not distinguish between *tow* and *tu* (both *トゥ*), *iw* and *i.u* (both *イウ*), *ay* and *a.i* (both *アイ*), etc. Some alternative Katakana scripts are proposed to solve this problem, but none of them are widely adopted. We are planning to support some of these alternative scripts in the future.

## Installation

Install the package using cargo

```bash
cargo add ainconv
```

or add the following line to your `Cargo.toml` file

```toml
[dependencies]
ainconv = "0.1.0"
```

## Usage

### Word Conversion

```rust
use ainconv::{
    convert_kana_to_latn,
    convert_latn_to_kana,
    convert_cyrl_to_latn,
    convert_latn_to_cyrl,
    convert_kana_to_cyrl,
    convert_cyrl_to_kana,
    // ...
}

println!("{}", convert_kana_to_latn("イランカラㇷ゚テ")); // "irankarapte"
println!("{}", convert_latn_to_kana("irankarapte")); // "イランカラㇷ゚テ"
println!("{}", convert_cyrl_to_latn("иранкараптэ")); // "irankarapte"
println!("{}", convert_latn_to_cyrl("irankarapte")); // "иранкараптэ"
println!("{}", convert_cyrl_to_kana("иранкараптэ")); // "イランカラㇷ゚テ"
println!("{}", convert_kana_to_cyrl("イランカラㇷ゚テ")); // "иранкараптэ"
```

### Extra Functionality

#### Script Detection

Detect the script of a given string.

```rust
use ainconv::detect;

println!("{}", detect("aynu")); // "Latn"
println!("{}", detect("アイヌ")); // "Kana"
println!("{}", detect("айну")); // "Cyrl"
```

#### Syllable Splitting

```rust
use ainconv::separate;

println!("{:?}", separate("eyaykosiramsuypa")); // ["e", "yay", "ko", "si", "ram", "suy", "pa"]
```

## License

[MIT License](LICENSE) (c) 2024 mkpoli

## See also

* [ainconv - npm](https://www.npmjs.com/package/ainconv): The JavaScript version of this package
* [Module:ain-kana-conv - ウィクショナリー日本語版](https://ja.wiktionary.org/wiki/%E3%83%A2%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%AB:ain-kana-conv): The original Lua Scribunto module in the Japanese Wiktionary