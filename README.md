# hfq_rs
## 概要
hfqcoをrustに書き直したものです。pythonと比べてスレッドの処理に優れている上にデータフレームはpandasではなくpolarsを使うことでかなりの速度向上が見込めます。
hfqco: josimで動作するように作成した回路ファイルを解析し指定された回路素子の動作マージン（どの値まで回路が正常に動作するか）を探すライブラリです。



## 進捗
### 完了
 - josimを呼び出す関数
 - スイッチするタイミングを記録する関数
 - 特定の変数の値を変更させる関数
 - 値を変更しても正常に動作するのかスイッチするタイミングで判定する関数
   - 注: これらは単一の素子にしか動作しないので複数の回路素子をまたいで検証したい場合にそなえて別途関数を作成します。
 - ファイル全体を通してマージンを計測する関数
 - API->こちらのレポジトリを利用してください
### 作業中
なし 
### 未着手
なし

---

# hfq_rs

## Overview
This project is a rewrite of hfqco in Rust. Compared to Python, it excels in handling threads and achieves significant speed improvements by using Polars instead of Pandas for dataframes.

### What is hfqco?
hfqco is a library designed to analyze circuit files created for josim and identify the operating margins of specified circuit elements (the range within which the circuit functions correctly).

## Progress
### Completed
 - Functions to call josim.
 - Functions to record the switching timing.
 - Functions to change the values of specific variables.
 - Functions to determine if the circuit operates normally by checking the switching timing, even after changing the values.
   - Note: These functions currently work only on single elements, and separate functions will be created to verify across multiple circuit elements.
 - Functions to measure margins throughout the entire file.
 - API: Please use this repository for API.

### In Progress
 - None

### Not Started
 - None
