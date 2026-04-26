# Python Implementation Team Guide

## 役割

Python implementation team は、Python 配布、Python language adapter、Python 生成規則、uv / Pytest / Ruff 導線の実装品質を保つ責任を持つ。

## 担当範囲

- `python/mds`: Python package distribution。
- `python/mds_lang_py`: Python language adapter distribution。
- `packages/lang-py`: npm distribution 向け Python adapter 境界。
- Python の `.py` / `.pyi` / `test_*.py` 生成規則、import 生成、fixture。

## ルール

- Python 固有の import 形式、stub 出力、test file 命名は Python adapter に閉じ込める。
- core の Markdown model、`Expose` / `Uses` schema、config の意味を Python 側で変更しない。
- Python package の public API は package root の `__init__.py` で明示し、内部 module を暗黙公開しない。
- test は implementation source から分離し、Pytest discovery に合わせて `tests/` 配下または `test_*.py` として配置する。
- generated fixture の期待値は、absolute package import、`.pyi` stub、header、末尾 LF まで固定する。
- Python 実装変更では `uv` / `pytest` / `ruff` 導線が存在する場合に必ず実行する。導線がない場合は未実施理由を task に残す。

## 固有知識

- Parser + 生成 MVP の Python 既定 pattern は、`src-md/pkg/foo.py.md` から Source `src/pkg/foo.py`、Types `src/pkg/foo.pyi`、Test `tests/pkg/test_foo.py` を生成する。
- `internal` import は生成先 source root からの absolute package import として生成する。
- Python の `Types` は MVP では `.pyi` stub として扱う。
- default import / alias 相当の拡張は MVP 外であり、実装前に spec を追加する。

## 関連資料

- `../architecture.md`
- `../patterns/impl-adapter-boundary.md`
- `../specs/packages-lang-py/SPEC-adapter-python-generation.md`
- `../specs/shared/SPEC-parser-generation-mvp-phase.md`
- `../validation.md`
