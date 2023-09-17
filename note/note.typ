#set text(font: ("Junicode", "Noto Serif CJK JP"))
#import "@preview/prooftrees:0.1.0"

#let Int = math.mono("Int")
#let to = $#box(width: -9pt) -> $

型を
$T ::= Int | (T, ..., T) to T$
で定義した (`ty.rs` 中の `enum Ty`) ところに，
2 つの部分型付け

+ $(T'_1, ..., T'_n) to T <: ((T_1, ..., T_m) to T'_1, ..., (T_1, ..., T_m) to T'_n) to (T_1, ..., T_m) to T$

+ $T <: (T_1, ..., T_m) to T$

を導入する．たとえば `id` $: (Int) to Int$，`add` $: (Int, Int) to Int$ なので `add(id, 1)` は
#{
  let Int = math.mono("I")
  prooftrees.tree(
    prooftrees.axi[`add` $: (Int, Int) to Int$],
    prooftrees.axi[],
    prooftrees.uni(right_label: "1")[$(Int, Int) to Int <: ((Int) to Int, (Int) to Int) to (Int) to Int$],
    prooftrees.bin[`add` $: ((Int) to Int, (Int) to Int) to (Int) to Int$],
    prooftrees.axi[`id` $: (Int) to Int$],
    prooftrees.axi[`1` $: Int$],
    prooftrees.axi[],
    prooftrees.uni(right_label: "2")[$Int <: (Int) to Int$],
    prooftrees.bin[`1` $: (Int) to Int$],
    prooftrees.tri[`add(id, 1)` $: (Int) to Int$],
  )
}
となる（$Int$ が長いので $mono(I)$ と書いた）．