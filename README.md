# abusing specialization to the max

this crate showcases some interesting peculiarities of how Rust's specialization lets you do some trickery.


## Showcased here:

- Marker: specialize against a concrete type (like a Signal) to have different behavior from the general more blanket impl
- BlanketBlanket: have two blanket impls but specialize on the one where the implemented type implements a trait of interest. In this case, any type that's Copy
- MaybePartialEq: A form of BlanketBlanket where we can compare non-comparable types
- MaybeStatic: doesn't work (yet) but the goal here is to specialize on a type if it has an internal lifetime.
