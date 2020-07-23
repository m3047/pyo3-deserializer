# pyo3-deserializer
## _An example deserializer written in Rust, callable from Python_

_Copyright (c) 2020 Fred Morris Tacoma WA 98445 USA. Apache 2.0 license_

Because I needed it to be faster, _of course!_ In the past I would have done this in _C_, but I stumbled across [PyO3](https://github.com/pyo3/pyo3).
I was initially skeptical but it worked pretty darned well. The one thing which would have made the experience truly better was better insight
upfront as to what the lift and result would look like. In that spirit, here's what I produced.

#### TLDR

Assuming you've got a functional _Rust_ and _Cargo_ environment, invoke `run.sh` in the toplevel directory.

### Pros and Cons

On the plus side:

* **Supports the usual _Python_ types:**
  * strings
  * ints and floats
  * lists
  * tuples
  * dicts
  * value-or-`None` semantics
* **Doc strings**
* **Lets you create _Python_ classes in _Rust_**
* **Binary `.so` imports as a "typical" _Python_ module**

On the other hand:

* **Can't subclass the classes** Not a deal breaker, otherwise they work and act like normal python classes. _Rust_ doesn't support subclassing, so it doesn't violate the spirit of _Rust_.
* **_Node_ -like ecosystem** I don't consider this a plus.
* **4 MB binary `.so` file** `cargo build` produces an `.so` which is over 4MB, even with `--release`.

### The Use Case

I've got observation data sitting in files where each line is a record consisting of tab-separated values. The first few fields are always
present, then there are an arbitrary number of attribute + value pairings.

The immediate calling code in _Python_ looks a lot like this:

```
with open('data.tsv') as f:
    for line in f:
        observation = BaseDevice(line)
        if not observation.valid():
            continue
        # Do some stuff
```

### Some specifics

Notice the first thing I do after turning the line into an object is to make sure it's valid! I could have wrapped it in a `try:` but my instinct
is that it's faster this way.

#### attribute visibility

The object attributes are native _Rust_ types. They're made visible by the `#[py03(get)]` macro. It supports put (`#[pyo3(get,put)]`) as well, but
I don't want/need that. Frankly the reason they're visible at all is for debugging and screwing around. I don't actually access the attributes
this way in production.

#### no `__init__`, use `new`

The constructor is the `new()` method, decorated with the `#[new]` method.

#### `&str` versus `String`

You'll notice that the constructor takes a `String` but the `attr()` method takes a `&str` and there's a reason for this. We're storing the string passed
to the constructor, and we want it allocated in the (_Python_) heap.

#### value or `None` semantics

`Option<T>` implements this. You need to declare the type which will be returned when a value is present, for example `Option<u32>` to return an
unsigned integer. `Option` is a Rust _enumeration_, which is a misnomer because they're not numerate. In any case it can be either a `Some<T>` or `None`.

#### `Err(_)`

The `Err<T>` type expects a value, but we don't care what the value is. _Rust_ will complain if you declare a variable and never use it, but
allows the special variable name `_` for unused values.
