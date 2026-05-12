# About

Better-sms (Better Shared Mutable State) is a lightweight and very small, but comfortable library for the Rust language, created to make it easier to work with shared mutable state.

# How to use

Below is a simple example of usage, more details in the [documentation](https://docs.rs/better-sms)

```
use crate::{
    arc::ArcCreate,
    mutex::{MutexCreate, MutexGuardWork, MutexWork},
};

fn main() {
    let variable = 0;
    variable
        .create_mutex()
        .create_arc()
        .lock_unw()
        .use_guard(|data| *data += 1);

    let variable = 0;
}
```

This code is equivalent to the following 
```
use std::sync::{Arc, Mutex};

fn main() {
    let variable = 0;
    let arc_mutex = Arc::new(Mutex::new(variable));
    let mut guard = arc_mutex.lock().unwrap();
    *guard += 1;
    drop(guard);
}
```

# Installation

```
cargo add better-sms
```
