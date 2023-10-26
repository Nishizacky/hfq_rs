# hfq_rs
## 概要
hfqcoをrustに書き直したものです。pythonと比べてスレッドの処理に優れている上にデータフレームはpandasではなくpolarsを使うことでかなりの速度向上が見込めます。

## 進捗
### 完了
 - josimを呼び出す関数
 - スイッチするタイミングを記録する関数
 - 特定の変数の値を変更させる関数
 - 値を変更しても正常に動作するのかスイッチするタイミングで判定する関数
   - 注: これらは単一の素子にしか動作しないので複数の回路素子をまたいで検証したい場合にそなえて別途関数を作成します。
### 作業中
 - 
### 未着手
 - ファイル全体を通してマージンを計測する関数



# hfq_rs
## Overview
This is a rewrite of hfqco in Rust. It excels in thread processing compared to Python and offers significant speed improvements by using Polars instead of Pandas for data frames.

## Progress
### Completed
- Function to call josim
- Function to record switching timing
- Function to change the value of specific variables
- Function to determine if it operates correctly even after changing values at the switching timing
  - Note: These functions only work on a single element, so separate functions will be created for testing across multiple circuit elements.

### In Progress
-

### Not Started
- Function to measure margins throughout the entire file.
