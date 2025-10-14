---
marp: true
theme: rose-pine-moon
paginate: true
footer: Roni Tuohino | rt@ronituohino.com | 17.10.2025
title: Async Rust
author: Roni Tuohino
---

# Async Rust

yikes

---

## Threads

---

"Don't communicate by sharing memory; share memory by communicating"  
<small>(R.Pike)</small>

---

i.e. "Don't overengineer inter-thread communication by using shared memory and
complicated, error-prone synchronisation primitives, but instead use
message-passing between goroutines ... so variables and data can be used in
sequence between those."  
<small>(SirDarius, StackOverflow)</small>

https://stackoverflow.com/questions/36391421/explain-dont-communicate-by-sharing-memory-share-memory-by-communicating

---

But what if I want to?

---

But what if I want to? :D
