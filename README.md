# pasm
pasm(Poco ASseMbler)は慶應義塾大学 理工学部 情報工学科 天野英晴教授の開発した教育用RISC命令セットであるPocoのアセンブラです。
授業で使用した教科書([作りながら学ぶコンピュータアーキテクチャ](https://www.am.ics.keio.ac.jp/pocobook/))に命令セットの詳細と、
CPUのVerilog実装が記載されています。
PocoにはRubyで書かれたshapaというアセンブラが既に存在しますが、
アセンブラに興味があったため、Rustで新しいものを開発しました。
# 使用方法
以下はCargo経由でpasmを使用する場合のコマンドです。
コンパイルしてバイナリを実行する場合は```cargo run --```を```./pasm```等に読みかえると動かすことができます。
使用可能なすべてのコマンドは
```Shell
cargo run -- --help
```
で確認することができます。

アセンブル結果はdatファイルに保存されますが、フォーマットは単なるテキストファイルです。
そのため、verilogで実装したCPUのテスト用プログラムに用いることができます。
## 出力ファイル名を指定しない場合
```Shell
cargo run -- [FILENAME].asm
```
カレントディレクトリに```a.dat```がアセンブル結果として保存されます。

## 出力ファイル名を指定する場合
```Shell
cargo run -- [INPUT_FILENAME].asm -o [OUTPUT_FILENAME].dat
```
```-o```オプションを渡すことで出力ファイル名を指定することができます。

## 使用例
アセンブリ言語で記述されたプログラム(mult.asm)。
メモリ上の2番地と3番地に保存された値の積を計算して、メモリ上の0番地に保存する。
```
LDIU r0, #2
LD r1, (r0)
LDIU r0, #3
LD r2, (r0)
:loop
ADD r3, r1
ADDI r2, #-1
BNZ r2, loop
LDIU r0, #0
ST r4, (r0)
:end
BEZ r2, end
```
コマンド
```Shell
cargo run -- mult.asm -o mult.dat
```
出力ファイル(mult.dat)
```
01001_000_00000010   // LDIU r0, #2
00000_001_000_01001  // LD r1, (r0)
01001_000_00000011   // LDIU r0, #3
00000_010_000_01001  // LD r2, (r0)
00000_011_001_00110  // ADD r3, r1
01100_010_11111111   // ADDI r2, #-1
10001_010_11111110   // BNZ r2, loop
01001_000_00000000   // LDIU r0, #0
00000_100_000_01000  // ST r0, (r4)
10000_010_00000000   // BEZ r2, end
```
