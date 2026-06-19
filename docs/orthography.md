# Ainu Orthographic Variation — Catalogue & Converter Mapping

A survey of the ways Ainu (アイヌ語) is written, and what a script converter such as
`ainconv` can and cannot do about each. The goal is twofold: (1) a comprehensive
**list of problems** for reference/article use, and (2) a mapping of each problem
to concrete converter behaviour, so we know which belong in the shared options
catalogue (`options.schema.json`), which are deterministic, and which are
fundamentally ambiguous (the "machine-learning territory" of automatic
conversion).

> **There is no single Ainu orthography.** Aynuwiki states it outright (特に正書法
> として定まったものはありません). Nakagawa (2006) frames *all* variation as a tug-of-war
> between three goals: **(A)** match the phonemic/Latin spelling 1:1, **(B)** match
> actual pronunciation, **(C)** stay close to ordinary Japanese kana habit. Every
> system is a different weighting of A/B/C. A converter therefore cannot be
> "correct" in the abstract — it can only target a system and offer toggles for
> the axes where systems disagree.

## Legend

| Tag | Meaning | Converter implication |
| --- | --- | --- |
| ✅ **deterministic** | One input string maps to one output unambiguously. | Implement directly (many already are). |
| ⚙️ **option** | Multiple valid outputs; choice is a style/system axis. | Belongs in the shared `options.schema.json`. |
| ⚠️ **ambiguous** | The string alone is insufficient; needs context, etymology, dialect, or pronunciation knowledge. | Cannot be resolved mechanically → lexicon/ML, or a lossy default. |
| 🐛 **technical** | An encoding/implementation hazard rather than an orthographic choice. | Normalise/handle defensively. |

"Current ainconv" notes refer to `ainconv-rs` (`src/conversion/*.rs`) at the time
of writing; the JS and Python ports differ (see `ainconv-tests` and the
cross-implementation notes at the end).

---

## A. Katakana (カタカナ)

### K1. Small (subscript) coda kana vs full-size — ⚙️ option (partly ✅)
Codas may be written with the **Katakana Phonetic Extensions** block (U+31F0–31FF,
added in Unicode 3.2 *specifically for Ainu*): ㇰ ㇱ ㇲ ㇳ ㇴ ㇵ ㇶ ㇷ ㇸ ㇹ ㇺ ㇻ ㇼ ㇽ ㇾ ㇿ,
plus small ィ/ゥ and ㇷ゚. Older/popular writing uses **full-size** kana for the same
codas (アイヌ with full ヌ, パスクル). The small-coda convention is itself a historical
innovation (traced to Nagata 1891).
- *Current ainconv:* emits small coda kana (`-k`→ㇰ, `-m`→ㇺ, `-s`→ㇱ, `-r`→ㇻㇼㇽㇾㇿ …;
  `katakana.rs` coda table). No toggle for full-size codas.
- *Action:* a `fullSizeCoda` style option for the popular register.

### K2. `-p` coda: ㇷ゚ vs プ vs ㇷ — ⚙️ option (default ✅ ㇷ゚)
`ㇷ゚` = ㇷ (U+31F7) + **U+309A combining semi-voiced mark** (e.g. カㇷ゚ *kap*). Popular
text uses full プ; a degraded form drops the mark (bare ㇷ).
- *Current ainconv:* `-p`→ㇷ゚ (`katakana.rs`, coda `p`); reverse ㇷ゚→`p` digraph handled.
- *Action:* option `pCodaStyle: handakuten | full | bare`. See also **T1**.

### K3. `/tu/` syllable: ト゚ vs ツ゚ vs トゥ vs ツ vs ド/ト — ⚙️ option
Ainu /tu/ has no native single kana. Variants by system: **ツ゚** (Kindaichi, Chiri —
phonemic camp), **ト゚** (Edo origin), **トゥ** (Haginaka 1973 → now dominant; Aomoto,
Midorimoto, Huling), **ド/ト** (Nabesawa), plain **ツ**.
- *Current ainconv:* `tu`→ト゚ (`katakana.rs:102`), coda/digraph トゥ→`tu` and ト゚→`tu`.
- *Action:* option `tuStyle: to_handakuten | tsu_handakuten | small_u | plain`.

### K4. `wi/we/wo`: ヰ/ヱ/ヲ vs ウィ/ウェ/ウォ — ✅ done (⚙️ option)
Archaic glyphs (Kindaichi) vs modern combinations (Chiri-late onward, the modern
standard).
- *Current ainconv:* **implemented** as `use_wi` / `use_we` / `use_wo` (default spells
  out ウィ/ウェ/ウォ). `ye` has no native glyph → イェ everywhere (✅ deterministic).

### K5. `-n` coda ン vs ㇴ; `-m` coda ㇺ vs ム vs ン — ⚙️ option (n done)
`-n` is overwhelmingly full **ン**; small ㇴ exists but is "seldom used". `-m` is
usually small ㇺ, but popular text uses full ム, and Midorimoto writes `-m` as **ン
before p/m** (visual assimilation — overlaps **L2**).
- *Current ainconv:* `-n`→ㇴ then normalised to ン unless `use_small_n` (✅ done).
  `-m`→ㇺ with no toggle.
- *Action:* `use_small_m` (ㇺ vs ム) and a nasal-before-labial rule shared with L2.

### K6. Diphthong / glide ambiguity: アイ = `ay` vs `a.i` — ⚠️ ambiguous
The only marker is writing the glide **small** (アィ = `ay`) vs **full** (アイ = `a.i`);
イゥ vs イウ for `iw`/`i.u`. Most casual/historical text does not observe the
small/large distinction, so it is routinely lost. Even systems disagree on whether
glides are small (Chiri) or large (Kindaichi).
- *Current ainconv:* on `latn→kana`, `-y`→ィ / `-w`→ゥ (then イ/ウ unless `use_small_i`/
  `use_small_u`). On `kana→latn` the README documents this as **lossy**: アイ→`ai`,
  イウ→`iw` are not recoverable.
- *Action:* none mechanical for the reverse direction → this is core ML territory
  (see **X1**). Forward direction is the `use_small_*` options (K4/done).

### K7. Long vowels: ー (chōonpu) vs vowel doubling vs diacritic — ⚙️ option
Largely a **Sakhalin** phenomenon (Hokkaidō has no phonemic length). Kana marks it
with ー or by doubling; Latin with macron/circumflex (see **L6**).
- *Current ainconv:* **not handled** — no ー (U+30FC) logic at all.
- *Action:* `longVowelStyle` + decide ー↔doubling↔macron mapping. Needed for Sakhalin.

### K8. Geminate / double consonant: per-coda small vs blanket ッ — ⚙️ option
`-pp-/-tt-/-kk-/-ss-`: the 1:1 camp writes ホㇷ゚パ / サッテㇰ / ラㇰコ / アㇱサㇷ゚; Midorimoto
1988 collapses all to 促音 ッ (ホッパ / ラッコ …), accepting that *matkosanu* vs
*makkosanu* both become マッコサヌ. `-t` coda specifically: ペッ vs ペㇳ both used.
- *Current ainconv:* `tt`→ッ and coda `t`→ッ (`katakana.rs`); `-T` (uppercase) →ㇳ as an
  escape hatch. No general geminate policy.
- *Action:* `geminateStyle: per_coda | sokuon` and `tCodaStyle: sokuon | small_to`.

### K9. `-s` coda: ㇱ vs ㇲ vs full ス — ⚙️ option
Both small forms (ㇱ shi / ㇲ su) and full ス are attested; Nagata used ス, Midorimoto
chooses "by sound heard".
- *Current ainconv:* `-s`→ㇱ only.
- *Action:* `sCodaStyle: shi | su | full`.

### K10. `-r` coda: echo-vowel set ㇻㇼㇽㇾㇿ vs single ㇽ vs full ル — ⚙️ option
The **single most contested point** in the kana literature. Vowel-harmonic set
(Kindaichi, Aomoto, Wiktionary) vs Chiri-late's uniform ㇽ vs Haginaka's "write the
echoed vowel plainly" (full ル forms).
- *Current ainconv:* vowel-specific echo set ㇻㇼㇽㇾㇿ (`katakana.rs` r-coda match).
- *Action:* `rCodaStyle: echo_vowel | uniform_ru | full`.

### K11. Voiced onsets / affricate-as-za: パ vs バ, ザ/ゼ/ゾ for ca/ce/co — ⚠️ ambiguous / ⚙️ option
Ainu has **no voicing contrast**, so onsets are written either always-clear
(パ/タ/カ/サ) or "as heard" with voiced kana (バ/ダ/ガ/ザ) — author-dependent and drifting
even within one author (Nabesawa). The `c` affricate before non-`i` vowels is
written with the **za-row** (ザ/ゼ/ゾ) in several systems (Edo habit).
- *Current ainconv:* clear onsets only; `ca`→チャ etc. Voiced kana on input → passed
  through untouched (not romanised back to p/t/k).
- *Action:* forward: an optional `affricateKana: cha | za` system (distinct from
  the Latin-romanization `affricateStyle` in **L4**). Reverse (voiced kana → which
  Latin) is ⚠️ — [bd gz] are allophones, generally normalise to p/t/k/c, which is
  lossy but standard.

### K12. Half-width katakana — 🐛 technical / ⚙️ option
Half-width forms (U+FF61–FF9F) exist for base kana and the half-voiced mark ﾟ
(U+FF9F), **but there are no half-width forms of the U+31F0 small codas** — so
half-width Ainu text cannot represent proper codas and falls back to full-size,
a lossy variant register.
- *Current ainconv:* reverse mapping accepts many half-width kana (ﾑ→m, ﾌ→h, ﾟ→p
  digraph …). No half-width *output*.
- *Action:* normalise half-width input to full-width before processing (mostly done
  ad hoc); document that half-width output is not representable.

### K13. Hiragana for Ainu — ✅ deterministic (rare register)
A minor practice writes Ainu in hiragana with the same small/handakuten apparatus.
- *Current ainconv:* hiragana → Latin mapping exists (`katakana.rs` hiragana table,
  incl. ゐ/ゑ/を). No Latin→hiragana.

---

## B. Romanization / Latin (ローマ字・ラテン文字)

### L1. Case: all-lowercase vs sentence-case + proper nouns — ⚙️ option (context-bound)
Linguistic/dictionary convention is **all-lowercase** (case is non-phonemic; the
romanization's job is the lookup form). Missionary/literary texts (Batchelor; Chiri
Yukie's *Ainu Shin'yōshū*) use European sentence-initial capitals. Your article's
recommendation: lowercase for short forms, capitalise for long prose, free on the
web.
- *Current ainconv:* **Latin↔Cyrillic preserves case** (verified: `Aynu itak` ↔
  `Айну итак`, `Айну` → `Aynu`); only the kana paths are caseless (`latn→kana`
  lowercases its input, `kana→latn` emits lowercase). Note the README's blanket
  "always lower case" is inaccurate for the Cyrillic path.
- *Action:* `capitalizeSentences` — but true sentence-initial caps need sentence
  segmentation (depends on punctuation handling), so it is only *semi*-deterministic.

### L2. `m` vs `n` before labials: kampi/kanpi, ampe/anpe — ⚠️ ambiguous (etymological)
Phonetic camp (発音主義): nasal → `m` before p/m always. Etymological camp (語源主義):
keep the underlying nasal across a morpheme boundary (*an*+*pe* → `anpe`). Documented
positions: Hattori → all `n`; Nakagawa 1995 → etymology-else-`m`; Tamura 1996 →
etymology-else-`n`; Ōta 2022 → all `m`; Kitahara → etymological `np`. Roots with no
boundary (kanpi/kampi) are genuinely indeterminate; ja.Wikibooks even proposes a
macron `n̄` to flag it.
- *Current ainconv:* no nasal handling; passes letters through.
- *Action:* `nasalAssimilation: phonetic | etymological`. **Phonetic is deterministic**
  (collapse to `m`); **etymological needs a morpheme lexicon** → lexicon/ML.

### L3. Glottal stop `ʼ`: written on vowel-initial syllables vs not — ⚙️ option
Phonemic system writes `’` on every vowel-initial syllable (`’aynu`, intervocalic
hiatus `’uwepeker`); AKOR ITAK does **not** (`aynu`). Glyph: typeset `’` (U+2019) vs
ASCII `'` (U+0027) — same phoneme (🐛 normalisation, see T2).
- *Current ainconv:* accepts both `'` and `’`; strips `’` on `latn→kana`; on
  `kana→latn` inserts `’` between vowels with a context rule. `=`/`'` are recognised
  "Ainu letters".
- *Action:* `writeGlottalStop: always | intervocalic | never`; always normalise the
  two apostrophe code points on input.

### L4. Affricate: `c` vs `ch` vs `č` vs `t` — ⚙️ option
`c` (modern/phonemic: Tamura, Nakagawa, AKOR, Wiktionary), `ch` (Hepburn lineage:
Batchelor, Kindaichi), `č` (Chiri 1942 only), `t` (Sakhalin before /i/).
- *Current ainconv:* `c` only (`latin.rs` consonants); reverse produces `c`.
- *Action:* `affricateStyle: c | ch | c_caron`.

### L5. Glides: `ay/aw` vs `ai/au`, `w` vs `u` — ⚙️ option
Phonemic/AKOR use `y`/`w` after vowels; Hepburn uses `i`/`u` (`ainu`, `horkeu`);
Chiri 1942 used `j`/`ŭ`.
- *Current ainconv:* `y`/`w` system; **reverse is lossy** (アイ→`ai`, see K6/X1).
- *Action:* `glideStyle: yw | iu`.

### L6. Long vowels & Sakhalin length: `aa` vs `ā` vs `â` — ⚙️ option
Doubling dominant (Tamura, AKOR, Omniglot); macron in some descriptive/Sakhalin
work; circumflex rare. **Sakhalin length is phonemic** and must be marked; Hokkaidō
length is mostly accent-driven and often unmarked.
- *Current ainconv:* no length handling; **acute accent is stripped** by
  `remove_acute_accent` (U+0301) before kana conversion.
- *Action:* `longVowelStyle: double | macron | circumflex`; pair with K7.

### L7. Boundary marker: `=` vs space vs `-` vs nothing — ⚙️ option
AKOR ITAK 1994 `=` for personal-affix/clitic boundaries (`cis=an`, `a=nukar`) is now
dominant; Dialect-Dictionary uses a space; older Hepburn fuses (`chisan`); community
writers use `-`; `・`/`⹀` also attested.
- *Current ainconv:* **strips `=`** on `latn→kana`; **preserves `=`** on Latin↔Cyrillic
  (test case `a=nukar` ↔ `а=нукар`).
- *Action:* `boundaryMarker: equals | space | hyphen | none` (and consistent handling
  across all directions — currently inconsistent).

### L8. Compound hyphenation & word spacing (分かち書き) — ⚠️ ambiguous
Latin marks word/morpheme boundaries with spaces so the dictionary form survives
surface assimilation (`or ta` vs kana オッタ); the watari-on glide differs by system
(`’uwepeker` vs `uepeker`). Where to put boundaries is author-dependent.
- *Current ainconv:* word splitting is by letter/non-letter runs only; no morphology.
- *Action:* none mechanical → lexicon/ML.

### L9. Acute accent for pitch/stress `á` — ⚙️ option
Marked "as needed" in teaching materials (FF-Ainu), dropped in dictionary headwords
and most prose. Chiri's hiragana-for-high-accent device was never adopted.
- *Current ainconv:* **always strips** acute before kana; not produced.
- *Action:* `keepAccent: bool` (currently effectively `false`, hard-coded).

### L10. Voiced obstruents `b/d/g/z`, loan `f/v` — ⚠️ ambiguous / out of scope
Native /p t c k/ voice allophonically intervocalically; writing them is an optional
"rougher" spelling. `b d g z` mainly in loanwords; `f v` non-native.
- *Current ainconv:* not specially handled.
- *Action:* normalise to voiceless for native words (lossy, standard); loanword
  policy is out of scope.

### L11. Sakhalin / Kuril romanization — ⚙️ option (dialect)
Coda `-h` ([x]; palatal [ç] after i) written `x` (Chiri 1942) vs `h` (later);
phonemic length (L6); coda inventory lacks -p/-t/-k/-r (fricativised to h). Kuril
Ainu survives mostly as Russian Cyrillic word-lists (Pallas), no living standard.
- *Current ainconv:* coda `h`/`x` → ㇵㇶㇷㇸㇹ exist in the table; no dialect mode.
- *Action:* `dialect: hokkaido | sakhalin` gating length + h-coda + `x` spelling.

### L12. Fast-speech `i`-deletion after a glide: `yay-i-` → `yay-`/`yai-` — ⚠️ ambiguous (input ✅ as equivalence class)
The high vowel `i` deletes after the palatal glide `y` in fast/connected speech, so
one lexeme surfaces as a **variant set**: `yayitupare ~ yaytupare ~ yaitupare`
"be careful"; `eyayitupare ~ eyaytupare`; `yayiraykere ~ yairaykere` "thank".
Verbatim in Tamura 1996 (s.v. *eyayitupare*: 会話などで早く言うとき…y のあとの i は落ちて
*eyaytupare* と発音). The lexical high pitch sits on that slot-II `i` (Tamura: yi イ の
部分を高く発音する).
- *Current ainconv:* treats each spelling as a distinct string; `separate()` will
  syllabify `yayitupare` and `yaytupare` differently. No notion that they are the
  same lexeme.
- *Action:* a converter can **normalise the variants to one equivalence class on
  input** (deterministic), but *recovering* which underlying form/accent a given
  surface spelling came from is lexicon/morphology territory → ⚠️. Mapping each
  variant *forward* to kana is deterministic per variant (it interacts with K6
  small/large glide ヤィ vs ヤイ and L5 `yw`/`iu`).

### L13. Three-way `i`: object clitic `i=` vs antipassive prefix `i-` vs glide/vowel `i` — ⚠️ ambiguous
One historical etymon *i surfaces three ways that look identical once the `=` is
dropped (L7) or in kana: **(a)** the inflectional **object clitic `i=`** (1SG /
4th-person object "me/one", written `=`, e.g. `i=tura` "accompany me"); **(b)** the
derivational **antipassive prefix `i-`** (no `=`, slot II, valency-reducing, e.g.
`i-nukar` → `inkar`); **(c)** plain glide/vowel `i` in a root. Because `yay-`/`i-`
saturate the object slot internally, an external `i=` object marker cannot attach
to such a verb — so `i=` "disappears" not by deletion but because the slot is
already filled (this is the user's "i= disappears due to yay-").
- *Current ainconv:* `=` is recognised as an "Ainu letter" and **stripped** on
  `latn→kana`; on `kana→latn` no `=`/clitic boundary can be reintroduced. The three
  `i`s are indistinguishable to string rules.
- *Action:* preserve `=` where the writer marked it (L7 `boundaryMarker`); making
  any further `i=`/`i-`/`i` distinction is ⚠️ (needs morphological analysis — see
  the morpheme DB). Note `separate()` should treat `=` as a syllable-domain
  boundary, not swallow it as a coda.

---

## C. Cyrillic (キリル文字)

No single Cyrillic orthography — three layers: 18th-c. Pallas/Krasheninnikov (Kuril;
acute stress, heavy ь codas, ы, extra letters), Dobrotvorsky 1875, Piłsudski/Sentoku
(Sakhalin), and a modern systematic scheme (Polivanov-adjacent, maps AKOR-Latin).

### C1. `/w/`: в vs ў vs у — ⚙️ option
Modern default `в`; phonetic-purist `ў`; also `у`. (aw=АВ, iw=ИВ …)
- *Current ainconv:* `w`↔`в`.

### C2. `/e/`: э vs е — ✅ deterministic (modern) / ⚙️ historical
Modern scheme always `э` (reserving `е` for /ye/); older notations used `е`.
- *Current ainconv:* `e`↔`э`, and `ye`→`е` digraph.

### C3. Affricate: ц vs ч vs т — ⚙️ option
`ц` (systematic/Sakhalin), `ч` (Russian-conventional/historical), `т` (t-analysis);
voiced /dʒ/ → `дж`.
- *Current ainconv:* `c`↔`ц`.

### C4. Soft sign / coda treatment — ⚙️ option / 🐛
Pallas marks any coda with ь; modern uses ь mainly after с for `-сь` (=`-s`), and
`СЬ` for palatal /ɕ/.
- *Current ainconv:* `ь`→dropped on `cyrl→latn`; not produced.

### C5. Glottal `ъ`, accents, `ы` — ⚙️ option (mostly historical)
Modern glottal `ъ` ↔ Latin `’`; acute accents only Pallas-era; `ы` only in the 18th-c.
Kuril list.
- *Current ainconv:* `’`↔`ъ` (`cyrillic.rs`); accents not handled.

---

## D. Cross-script conversion

### X1. Kana → Latin is intrinsically lossy & ambiguous — ⚠️ ML territory
The article's "機械学習による解決へ" section. Standard kana underdetermines: アイ = `ay`
vs `a.i`, イウ = `iw` vs `i.u`, トゥ = `tu` vs `tow`, voiced kana → which Latin, missing
glottal stops, missing word boundaries, missing accent. No deterministic inverse
exists for general kana.
- *Current ainconv:* documented lossy in README; produces one best-effort reading.
- *Action:* this is exactly the ML auto-conversion problem; the converter should
  expose its assumptions (and ideally confidence), not pretend determinism.

### X2. Round-trip instability — ⚠️ consequence of X1
`kana → latn → kana` need not restore the original (and `latn → kana → latn`
collapses the small/large distinctions). Latin↔Cyrillic *is* lossless.

### X3. Word segmentation for kana — ⚠️ ambiguous
Kana runs words together (no spaces); recovering Latin spacing (L8) needs morphology.

### X4. Capitalisation & accent loss across scripts — ⚙️/⚠️
Case (L1) and accent (L9) are not carried through kana; partly recoverable only with
context.

### X5. Punctuation mapping (full-width ↔ ASCII) — ✅/⚙️
。「」『』！？、　… ↔ `. " ' ! ? ,` (space). The ellipsis … is the issue-#5 case.
- *Current ainconv:* maps the above on `kana→latn` for non-letter runs; `…`→`...`
  is the opt-in `ellipsisToAscii`. Reverse (ASCII→full-width) not done.

---

## E. Technical / encoding hazards

### T1. ㇷ゚ / ト゚ / ツ゚ are multi-codepoint and have NO precomposed form — 🐛
`ㇷ゚` = base + **U+309A** (combining). There is no single-codepoint ㇷ゚, so NFC does not
help. `\p{Katakana}` drops U+309A in most engines (incl. Rust/JS/Python), tearing the
mark off; `"ㇷ゚".length == 2`. (Zenn article; this is the same class as the apostrophe
slicing bug fixed in this repo.)
- *Mitigation:* process at the grapheme-cluster level; in regex append `゚?` to
  katakana classes; never split on code-point index.

### T2. Combining vs spacing vs half-width voicing marks — 🐛
Same glyph, different bytes: combining **U+309A** vs spacing **U+309C ゚** vs
half-width **U+FF9F ﾟ** (and U+3099/309B/FF9E for dakuten).
- *Current ainconv:* normalises some of these (`is_ainu_letter` accepts the family;
  reverse maps ﾟ). *Action:* canonicalise all voicing marks to the combining forms on
  input before matching.

### T3. NFC/NFD normalisation — 🐛
- *Current ainconv:* NFD on input / NFC on output in both katakana and cyrillic paths;
  acute handled via NFD-filter-NFC. Keep this consistent everywhere.

### T4. Multibyte glottal `’` slicing — 🐛 (fixed)
`split_at(len-1)` on a syllable ending in `’` (U+2019, 3 bytes) sliced mid-character
and panicked; fixed by char-indexed syllabification (commits ddd8a1c / cdeba89).

### T5. Byte vs char vs grapheme indexing generally — 🐛
All the above reduce to: index by **grapheme cluster** for user-facing units, never by
byte; Rust `str` byte-slicing is the trap.

---

## F. Proposed options-catalogue extensions

These extend the shared `options.schema.json` (today: `ellipsisToAscii`, `useWi`,
`useWe`, `useWo`, `useSmallI`, `useSmallU`, `useSmallN`). Adding a key to the catalogue
makes the per-language **parity test** fail until every implementation supports it, so
these should land *with* implementations, not before.

| proposed key | type | direction | values / default | problem |
| --- | --- | --- | --- | --- |
| `pCodaStyle` | enum | latn→kana | `handakuten`(default) / `full` / `bare` | K2 |
| `tuStyle` | enum | latn→kana | `to_handakuten`(default) / `tsu_handakuten` / `small_u` / `plain` | K3 |
| `rCodaStyle` | enum | latn→kana | `echo_vowel`(default) / `uniform_ru` / `full` | K10 |
| `sCodaStyle` | enum | latn→kana | `shi`(default) / `su` / `full` | K9 |
| `geminateStyle` | enum | latn→kana | `per_coda`(default) / `sokuon` | K8 |
| `useSmallM` | bool | latn→kana | `false` | K5 |
| `fullSizeCoda` | bool | latn→kana | `false` | K1 |
| `longVowelStyle` | enum | both | `double`(default) / `macron` / `circumflex` / `chouonpu` | K7, L6 |
| `affricateStyle` | enum | latn (romanization) | `c`(default) / `ch` / `c_caron` | L4 |
| `affricateKana` | enum | latn→kana | `cha`(default) / `za` | K11 |
| `glideStyle` | enum | latn | `yw`(default) / `iu` | L5 |
| `writeGlottalStop` | enum | latn | `intervocalic`(default) / `always` / `never` | L3 |
| `boundaryMarker` | enum | all | `equals`(default) / `space` / `hyphen` / `none` | L7 |
| `keepAccent` | bool | all | `false` | L9 |
| `capitalizeSentences` | bool | →latn | `false` | L1 |
| `dialect` | enum | all | `hokkaido`(default) / `sakhalin` | L11, K7 |
| `cyrlW` | enum | latn↔cyrl | `v`(default) / `w_breve` / `u` | C1 |
| `cyrlAffricate` | enum | latn↔cyrl | `ts`(default) / `ch` / `t` | C3 |

Note these are mostly **enums**, whereas the current catalogue is boolean-only; the
schema's `type` field already allows expressing enum options (`type: "enum"`,
`values: [...]`).

## G. What cannot be made deterministic (ML / lexicon territory)

- **K6 / X1 / X2 / X3** — kana→Latin diphthong, glottal, voicing, and word-boundary
  recovery. The article's final section is right: this is where statistical/ML
  conversion (kana → correct Latin) is the realistic path.
- **L2 etymological m/n**, **L8 morpheme boundaries** — require a morpheme lexicon.
- **K11 / L10 voicing** on input — allophonic; reverse-mapping is lossy by nature.
- **L12 / L13 the `yayitupare` family** — a single worked example that touches
  *every* ambiguous axis at once: the surface-variant set `yayitupare ~ yaytupare ~
  yaitupare` (L12 `i`-deletion + K6 small/large glide), a deleted slot-II `i`
  carrying the accent (L9/X4), the `i=`/`i-`/glide-`i` three-way (L13), and a
  morpheme string `yay-i-tupa-re` that no string rule can recover (the bound root
  `tupa` is not even text-segmentable — see the morpheme DB, where it is recorded
  as `etymology`, not synchronic composition). From kana, none of these are
  recoverable (X1). This is the concrete case for "expose assumptions, don't guess."

A principled converter should: pick a **canonical target** (AKOR ITAK 1994 ≈
Wiktionary modern standard), accept the other systems as **input variants**, expose
the **option axes** above for output, and treat the ⚠️ items as best-effort with
documented assumptions rather than silent guesses.

## H. Sources & gaps

Key sources: Nakagawa 2006 (*アイヌ人によるアイヌ語表記への取り組み*, TUFS/ILCAA — the
definitive 4-system taxonomy); Endō 2016 (Nabesawa kana system, Minpaku); Satō 2012;
Refsing 1986; Japanese Wiktionary *アイヌ語のカナ表記*; Aynuwiki; Huling docs; the Zenn
ㇷ゚-regex article; ja.Wikibooks 表記の揺れ / キリル文字対照表; Qvarie's notes; AKOR ITAK
(北海道ウタリ協会 1994); Unicode Katakana Phonetic Extensions (U+31F0–31FF).

**Not retrievable / to verify against physical sources:**
- Kitahara *樺太アイヌ語の世界* (2021) — the `np` etymological position is confirmed
  conceptually but needs a page-level quote.
- Piłsudski's specific 1912 Cyrillic letter table (his published corpus is mostly
  Latin) — needs the physical volume / MAE St. Petersburg archive.
- Kirikae 1997 and Satō 2012 PDFs could not be mined directly (access / CID-garbled
  encoding); their relevant points survive via Nakagawa 2006.
- **The Discord discussion** (`discord.com/channels/1181228576989794324/…`) is not
  accessible to this tooling — export or paste it to fold in additional problems.
