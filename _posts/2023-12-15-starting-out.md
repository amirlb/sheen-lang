---
layout:     post
author:     Amir
title:      "Starting out"
date:       2023-12-15 00:12:39 +0200
categories: announcement
---

What's going on here?

I was on vacation this week, and once my mind was off work it started
considering the design choices made in Rust, which I learned recently.
There are a lot of interesting concepts there: lifetimes, the trait
system, and how aggressive type-directed inlining can lead to abstractions
that don't have runtime cost.


We should recognize the earlier work on the same concepts: region
tracking, type classes and the lambda papers. But the phrasing of these
ideas in Rust is ergonomic in novel ways.

In my opinion some of the trade-offs are not at the optimal position.
And there are features that could be simpler and more powerful. There are
also other modern programming-langauge concepts that will be intersting
to include in a new language.

All in all, I talked myself into writing a compiler for a new programming
language. In the past I had a chance to write parsers and some very simple
interpreters, so this would be hard but hopefully not too far a leap.

In the upcoming posts I intend to explain the concepts of the new language,
called Sheen, and subjects related to its implementation.

Happy holidays!
