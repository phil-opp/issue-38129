#![no_std]


/**
# Failure

```
#![no_std]

use issue_38129::*;

fn main() {}
```

error: an inner attribute is not permitted in this context
 --> <anon>:2:3
  |
2 | #![no_std]
  |   ^
*/
pub fn foo() {}

/**
# This works

```
#![no_std]

use issue_38129::*;

extern crate issue_38129; // add the import manually

fn main() {}
```

(the "language item required" errors mean that `no_std` was recognized)
*/
pub fn bar1() {}

/**
# This works too

```
#![no_std]

use core::*;

fn main() {}
```

(the "language item required" errors mean that `no_std` was recognized)
*/
pub fn bar2() {}

/**
# This also works

```
#![no_std]

use issue_38129::*;

// extern crate

fn main() {}
```

(The "language item required" errors mean that `no_std` was recognized.
Of course, an "unresolved import error" occurs here.
But why did the comment influence the recognition of the `no_std` attribute?!)
*/
pub fn bar3() {}
