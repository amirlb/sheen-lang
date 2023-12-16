---
layout:     post
author:     Amir
title:      "Coarse lifetime analysis"
date:       2023-12-16 22:44:55 +0200
categories: sheen theory
---

The main pain point I had with Rust is lifetime annotations. Sometimes it's
very hard to get code to compile. Some of the trouble I had are due to:

* Syntax issues, the auto-ref and auto-deref rules are not intuitive, and
  there's some noise related to boxing and cloning
* Need to tag data structures with the lifetimes of objects they contain,
  which necessiates changes in several files when adding struct members
* The Rust data model requires the programmer to decide on the memory
  representation and doesn't encourage sharing pointers to data
* In some cases it's not possible to prove to the compiler that the code is
  correct, and it's not easy to identify these cases, so we somtimes clone
  or wrap objects with a reference count unnecessarily

Why not make it simpler? Because it has to be able to express complicated use
patterns. There's an alternative -- used in many other languages -- of
wrapping all the structs in `Rc<>` boxes. Or in `Gc<>` boxes. If we do this,
forgetting for a minute that the Rust syntax doesn't encourage it, we no
longer have to worry about copying, sharing, and ownership.

Sheen implements a mixed system. Each reference in the system has an owner,
which is either a variable or the GC. Function arguments that are passed by
reference (and are not copyable) have possible three ownership tags, that are
inferred automatically but can also be specified:

```
struct User:
    id: integer
    name: string  # Having a string field prevents this from being copyable

struct Message:
    user: owned &User
    contents: string

def inspects(user: &User):
    print(user.name)

def claims(user: &User):
    return Message(user, "hello")

def require_shared(user: &User):
    return (user, user)
```

If a function claims ownership of a value, it may no longer be used after the
function is called. The function is responsible for freeing the value or
returning another object that owns the value.

If a function requires that a reference be shared, it means that it must be
in managed memory, and the compiler automatically allocates the structure
using the GC.

Passing structs by value (when they aren't copyable) can only be done if the
parameter is inspected or claimed. Sharing data passed by value can only be
done by cloning. By-value parameters may be passed in registers, but if the
same struct is passed somewhere by reference it's allocated on the stack.

This system can handle non-shared references correctly. It also supports
patterns that are a bit harder to express in Rust, for example including the
source URL in every DOM object. Shared references can sometimes be detected
as "inspecting" and don't count, otherwise to share a reference it must be
managed by the GC. My intuition is that it's rare to have such allocations in
the core parts of programs, therefore Sheen programs will use the GC mostly
for long-lived objects, which will then have low overhead. I will measure
this after implementing.

I'm still writing sample programs to figure out if everything is covered,
and trying to formalize the rules in the mean time. See you soon with more
details!
