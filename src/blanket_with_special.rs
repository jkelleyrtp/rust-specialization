use std::marker::PhantomData;

/// A trait that allows any type to be rebound during a render of a component.
///
/// This is primarily used to allow signals to be derefed when unpacked during `inline_props`.
///
/// # Example
///
/// ```rust, ignore
/// use dioxus::core::Rebind;
///
/// fn takes_signal(signal: &Signal<i32>, non_signal: &Vec<String>) {
///     // Coerces a signal to an owned version of itself via a copy
///     let a: Signal<i32> = signal.rebind();
///
///     // Coerces a non-signal to a reference of itself - this is a no-op
///     let b = non_signal.rebind();
/// }
/// ```
pub trait Rebind<'a, T = ()> {
    type Target;
    fn rebind(&self) -> Self::Target;
}

struct Signal<T> {
    p: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { p: self.p.clone() }
    }
}
impl<T> Copy for Signal<T> {}
impl<'a, T> Rebind<'a> for Signal<T> {
    type Target = Signal<T>;

    fn rebind(&self) -> Self::Target {
        *self
    }
}

/// use a marker to prevent the blanket impl from being used for non-signal types
/// Rust will try its best to not use the marker, deferring to the above impl whenever it can.
/// This will kick in just for Signals.
///
/// But, if the type isn't a Signal, then rust will elect to use the marker instead
struct RebindBorrowMarker;
impl<'a, T> Rebind<'a, RebindBorrowMarker> for &'a T {
    type Target = &'a T;

    fn rebind(&self) -> Self::Target {
        self
    }
}

struct Props {
    value: String,
    other: Signal<i32>,
}

fn rebind_a_signal(props: &Props) {
    let a = (&props.value).rebind();
    let b = (&props.other).rebind();

    is_valid_ref(a);
    is_valid_signal(b);
}

fn is_valid_ref(t: &String) {}

fn is_valid_signal(t: Signal<i32>) {}
