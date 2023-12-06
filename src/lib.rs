//

/// Need to compare something that might not be comparable?
///
/// Have no fear! MaybePartialEq is here!
///
/// This should not be possible but it is.
mod maybe_partialeq;

/// Need to have a form of specialization with cross-cutting blanket impls?
///
/// Have no fear! BlanketWithBlanket is here!
///
/// This should *really* not be possible but it is.
///
/// Used by dioxus to Copy `Copy` types during `inline_props`.
mod blanket_with_blanket;

/// Need to have a form of specialization with a cross cutting impl and a concrate impl?
///
/// Have no fear! BlanketWithBlanket is here!
///
/// This should *really* not be possible but it is.
///
/// Used by dioxus to Copy `Copy` types during `inline_props`.
mod blanket_with_special;

/// This does not work yet, but the idea is that we can check if a type has internal borrows or not.
/// IE it's valid for the 'static lifetime
mod maybe_static;
