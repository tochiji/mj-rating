# 麻雀のレート計算プログラム

仲間内で使う用です。

## レート計算式

開始レートは $1500$ とし、対局するごとにレートが変動していく。

```math
変動後Rt = 変動前Rt + (順位点 + (同卓者の平均Rt - 自分のRt) \div 80) \times 0.2
```

順位点は下記の通り。
- 1着 +50pt
- 2着 +10pt
- 3着 -20pt
- 4着 -40pt

大体の場合、1着を取れば `+10.0`、2着を取れば `+2.0`、3着を取れば `-3.0`、4着を取れば `-8.0` 程度レートが変動する。
