# stopwatch-tui

## 使い方
```
git clone https://github.com/hayato0909/stopwatch-tui
cd stopwtach-tui
cargo run
```

## 仕様
- 59分59秒までのカウントが可能
- 一時停止した場合、表示されていないが1秒以下のカウント保持を行っている
- 使用できるボタンとその動作は以下の通り
  
| キー | 動作 |
| ---- | ---- |
| s | スタート or ストップ|
| r | リセット |
| q | 終了 |

- ウィンドウのサイズ変更に対応して、常に中央に表示を維持
