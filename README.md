# abusing specialization to the max

this crate showcases some interesting peculiarities of how Rust's specialization lets you do some trickery.


## Showcased here:

- Marker: specialize against a concrete type (like a Signal) to have different behavior from the general more blanket impl
- BlanketBlanket: have two blanket impls but specialize on the one where the implemented type implements a trait of interest. In this case, any type that's Copy
- MaybePartialEq: A form of BlanketBlanket where we can compare non-comparable types
- MaybeStatic: doesn't work (yet) but the goal here is to specialize on a type if it has an internal lifetime.

## What motiviated this?

In Dioxus, we have this `inline_props` macro that lets you declare a struct using arguments of a function for short-and-sweet components.
```rust
#[component]
fn Example(age: i32) -> Element {
    //...
}
```

This has some weird side-effects though - props in Dioxus are borrowed between renders. So everywhere `name` and `age` are used, they get borrowed. This means when dealing with async, you have to clone them before using them, since Futures in Dioxus must be `'static`

```rust
#[component]
fn Example(age: i32) -> Element {
    // age is actually &i32 due to how structs are unpacked
    // we need to copy it
    let age = *age;
    cx.spawn(async move {
        print!("{age}");
    });
    // ..
}
```

However, we can't just deref everything since some things can't be copied. This is particularly bad for heavy things where an implicit clone would cost a lot.

```rust
#[component]
fn Example(name: String) -> Element {
    // name is actually &String due to how structs are unpacked
    let name: String = name.clone();

    cx.spawn(async move {
        print!("{name}");
    });
    // ..
}
```

So, we need something akin to `maybe_copy`. There is not a trait for this. The code in this repository showcases a new dual-trait combo that allows you to call `rebind` on any T, and return Copied T when T is Copy, and &T when T is not Copy.

```rust
#[component]
fn Example(age: i32, name: String) -> Element {
    // No lifetime attached!
    let age: i32 = age.rebind();

    // This type can't be copied so it keeps its lifetime
    let name: &String = name.rebind();
}
```

This is *particularly* important when dealing with signals. Signals are copy types but would normally have their lifetimes stuck to them, making code like this impossible:

```rust
#[component]
fn Example(name: Signal<String>) -> Element {
    let uppercase = memo(move || name.to_ascii_uppercase());
}
```

With the rebind traits, we can fix this for all arguments while preserving the existing behavior for Clone types.
```rust
struct Props {
    name: Signal<String>
}

fn Example(cx: Scope<Props>) -> Element {
    //  generated by the component macro
    let name = (&cx.props.name).rebind();

    // now we can freely use name everywhere since its Copy
    let uppercase = memo(move || name.to_ascii_uppercase());
}
```

Cool, right?
