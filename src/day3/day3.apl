alphabet ← 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
solve1 ← {+/{(alphabet∊1⌷⍵)/⍳52}¨{(((((⍵/⍨⊢)∊(⍵/⍨(~⊢))) ((1∘(⍴⍨),0∘(⍴⍨))(≢⍵)÷2))),((≢⍵)÷2)⍴0)/⍵}¨⍵}
