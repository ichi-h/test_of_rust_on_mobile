# test_of_rust_on_mobile

RustのコードをAndroid/iOS向けにコンパイルするテスト。

モバイル端末上で音声ファイルをraw形式へデコードするためのライブラリを作成するテスト。  
現状、Android向けの動的リンクのコンパイルまで達成。  
Rust側からJava側にfloat配列を渡す方法が不明のため、サンプルデータを文字列で渡すような形を取っている。  
最終的には、波形情報、サンプリングレート、ビット深度などの情報をひとまとめにした構造体を返せるようになることが最終的な目的。

## タスク

- 実際にJavaから動的リンクを呼べるか
- Rustで作った配列をJavaから呼べるか

## 疑問点

- 動的リンクからAndroidのファイルにアクセスできるか

## 参考

- [RustをクロスコンパイルしてAndroidで動かす - totechite's blog](https://totechite.hatenablog.com/entry/2019/01/07/000000)
- [pdeljanov/Symphonia](https://github.com/pdeljanov/Symphonia)
