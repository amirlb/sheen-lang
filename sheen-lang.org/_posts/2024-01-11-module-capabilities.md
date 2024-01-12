---
layout:     post
author:     Amir
title:      "Module capabilities"
date:       2024-01-12 15:03:40 +0200
categories: sheen modules
---

One matter I want to explore in Sheen is protection against malicious dependencies.

This is sometimes called "supply chain attacks", but that term doesn't describe the
problem. The trick is to find a popular package and add malicious code. Depending on
where the package is used, the code can leak data, or take control over developers'
computers, or encrypt data and demand a ransom to restore it.

How do you add code to an existing package? One way is to find an open-source package
and solve a bug in it, and the exploit is added along with the bugfix. Or if the
maintainers are careful, create a useful new open-source package, add it as a dependency,
and include the exploit in some regular update. Another way, in languages where
packages are organized in repositories, is to copy the name of a package from one to
the other, and mark it so that package management software will install the hacked
version.


Some people argue that this isn't a real issue. They're
[wrong](#appendix-this-is-a-real-problem) and a technical solution is needed. Due to
[Rice's theorem](https://en.wikipedia.org/wiki/Rice%27s_theorem) it's not possible
to verify that the package does what is expected of it. We can only verify some
properties to limit the impact of such hacks.

My current plan for Sheen is to use ML-style
[functors](https://en.wikipedia.org/wiki/Standard_ML#Functors)
to define and pass capabilities between different modules, as well as designing the
standard library to assist with
[sandboxing](https://en.wikipedia.org/wiki/Sandbox_(computer_security)).


## Removing ambient authority

Part of the problem is that every module in every package can do anything the program
can do. This is preventable: if you can't `import`/`use`/`require`/`include` filesystem
access functions and network functions, you can't leak user data.

This is implemented in Javascript with the
[Shadow Realms](https://github.com/tc39/proposal-shadowrealm) system. In Javascript in
the browser user data is generally stored in global variables such as `document`
and `localStorage`, and sending data over the network is done using the global function
`fetch`. Global variables are actually properties of an implicit global object, and a
"realm" is a different global object. If a realm is created without these properties,
methods imported in that realm cannot leak user data, only return wrong results or hang.

This method requires calling `eval` in a complicated pattern during importing, which
makes it harder to use correctly for security, and harder to analyze statically. In a
system with a more common `import`-type statement the same idea looks a bit different.

Take Rust for example. Access to the filesystem happens through `std::fs` and its
submodules. If this module didn't have a name it couldn't be `use`d by malicious code
and it couldn't access files.
In order to allow the program to still access the filesystem, we say that there are
different namespaces for modules in different places in the program. A module can get
named parameters which are other modules. Then the main module receives `fs` as a
parameter and passes it to any module that needs it.

To flesh out this idea we borrow terminology from SML. A module exports types,
constants and functions that may be based on these types, and sub-modules. The
signature of the module is its interface: the list of type names, the types of the
constants and functions, and the signatures of the sub-modules. A functor is a
module-level function, that is executed and creates modules on compile time. When
defining a functor we list the module parameters and their signatures, and the
signature and implementation of the generated module.

Here is an example, with provisional Sheen syntax:

```
# In the standard library

signature FS:
    type File
    def open(path: str) -> File
    def read(file: File, buffer: slice[byte]) -> int
    ...
```

and

```
module main
requires fs: FS

import sqlite3(fs)
import regex  # Note no fs
```

Note that it's also to provide modules with fewer capabilities with the same signature,
for example limiting access to files in a single directory. This is explained further
in a [later section](#standard-library-design).

This system requires cataloguing the system capabilities to a discrete set: console,
filesystem, networking, threads, exec, audio, windowing, an so on. If we want to split
to finer capabilities it's a breaking change in the standard library. Another thing
this requires is that packages split their interface to the pure parts and impure
parts: a packages for handling CSV files should export a pure module (without
parameters) and a module that reads files (with FS access).


## Foreign modules

> Thanks to Dror for his help with this section

If we want to import binary modules that are already compiled (from other languages),
it's impossible to verify they only use a given set of capabilities. There are four
ways to go that I can see:

* Trust the module documentation that it only uses the capabilities it says it does

* Avoid foreign modules entirely. This limits our choices as there's a lot of useful
    code out there. But Javascript managed it for most of its existence, now only
    allowing importing Wasm modules, which have no direct access to system capabilities.
    And Java managed to mostly avoid foreign code for years too.

* Run foreign modules in a separate process with permissions tailored to the capabilities
    they say they needs. This is a bit heavy when passing data to functions in these
    modules as it needs to be moved the the shared-memory space. Such copying may be
    necessary in Sheen anyway due to the memory model, we don't want foreign code to
    store pointers that we might free automatically.

* Track the access to system capabilities. There are several ways to do that, all
    complicated:

    * Trap all system calls that happen during program execution and inspect the code
        that invoked them. If it was from a foreign module, check that it uses an allowed
        system call or raise an exception.

    * System calls in normal code happen using libc. This is linked dynamically so we
        can use LD_PRELOAD to override it with our own implementation that validates
        only allowed interfaces are used.

    * It's also possible to change the linking to libc in C source code by renaming a la
        [Knit](https://www-old.cs.utah.edu/flux/papers/knit-osdi00/),
        or even in the binary code using a similar technique, effectively implementing
        a linker on our own.

I'm currently leaning towards partly trusting library owners: verifying at the source
level that all imports are compatible with the stated capabilities, and compiling
packages based on this source code (or allowing signed builds that prove they use the
same source tree). This still does not protect us against malicious actors but helps
not make innocent mistakes.

But this is still widely open, all options have their merits.


## Standard library design

In addition to this coarse capabilities system that's easily verifiable, it's possible
to define functions and functors (parametric modules) that enable more fine-grained
separation of authority.

The simplest example is a JSON library. This library doesn't need to know about files at
all: it can get streams of bytes or characters and return objects. If it's convenient
enough to pass streams instead of files, the JSON library won't need the `filesystem`
capability.

A more sophisticated idea is to build sandboxes using modules that have the same
signature but fewer capabilities. For example we can define a functor to limit access
to a single sub-directory like

```
signature FS_Subdir:
    include FS
    def set_allowed_directory(path: str) -> None

functor LimitToDirectory -> FS:
    requires fs: FS

    type File = fs.File

    allowed_directory: Option[str] = ""

    def set_allowed_directory(path: str) -> None:
        allowed_directory = path

    def open(path: str) -> File:
        match allowed_directory:
            case None:
                raise "Allowed directory not set yet"
            case Some(dir):
                if not path.startswith(allowed_directory):
                    raise ...

        return fs.open(path)

    def read(file: File, buffer: slice[byte]) -> int:
        return fs.read(file, buffer)

    ...
```

and then use it in the main program as

```
module main
requires fs: FS

module SqliteFS = LimitToDirectory(fs)
import sqlite3(fs = SqliteFS)

# initialization code
SqliteFS.set_allowed_directory(db_dir)
```

Similarly we may want to allow networking, but limited to a subnet such as `10.0.0.0/8`
or `192.168.0.0/16`. If we want to allow only to connect through HTTPS to a specific
domain it's probably better done at a function level, or by wrapping the HTTP client
module, rather than as a capability-level sandbox.

Injecting a function like that is messier than supplying a stream to a JSON library
since it likely means adding parameters to a lot of intermediate functions that don't
care about the networking call. Maybe this means we need dynamic scoping in the language
to make this more convenient, but I can't think of a syntax that's both concise enough
to use and explicit enough to tell at a glance what capabilities are used by each module.

As a general note, this kind of sandboxing doesn't provide full protection against
hacks, but requires an attacker to gain additional privileges elsewhere in order to
exploit.

This kind of idea is more friendly to changes: new limits are added functions. The
downside is that it's less amenable to static analysis. These kind of features are more
easily added to other languages too, but is not common yet. Maybe because it's less
beneficial with a capabilities system as it's always possible to import the full module
and escape the sandbox.


------

### Appendix: This is a real problem

Here are some objections I read in internet discussions over the years:

* Why not just verify all the code used in a project or product?

    * As software gets more complicated, projects rely on more and more external packages,
        making it less economical to review all code

    * These external packages get a lot updates themselves, fixing bugs or security
        issues, or managing their own dependencies

    * Hackers get better at obfuscating the harmful code, so it looks innocent on casual look

    * Programming is becoming more popular, meaning that most developers are novices.
        And detecting the warning signs for harmful code is not taught formally in most
        courses.

* Can't we trust the package repositories to provide only safe packages?

    * To be fair, package repositories do this pretty well, and are safe to use

    * But it's becoming harder, since more code is added all the time, and interactions
        between packages can carry the same risks as code inside packages

    * Package repositories are managed by the language communities. This limits their
        resources to mainly volunteers and sometimes a few paid developers for the
        infrastructure, but not security experts.

    * It's not a good idea to put commercial entities in charge of the repositories.
        They may give good service for a while but they are not reliable: the benefit
        companies get from serving the community is reputation, which they use later
        to promote their products. If it's a package repository this probably means
        pushing paid products over free packages.

        * <p>
            And in order to be profitable, the profits from the resource and the
            reputation must eventually be higher than the investment in it
            </p>

    * Giving control to public or government entities may work well in the long term but
        digital infrastructure isn't treated this way nowadays.
