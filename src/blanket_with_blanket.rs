use std::marker::PhantomData;

trait NonPromote: Sized {
    fn promote(&self) -> &Self;
}

impl<T> NonPromote for &&T {
    fn promote(&self) -> &Self {
        self
    }
}

trait SignalPromote: Clone + Copy {
    fn promote(&self) -> Self;
}

impl<T: Clone + Copy> SignalPromote for T {
    fn promote(&self) -> Self {
        self.clone()
    }
}

struct Signal<T> {
    inner: PhantomData<T>,
}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal { inner: PhantomData }
    }
}
impl<T> Copy for Signal<T> {}

struct SomeProps {
    a: i32,
    b: String,
    c: Signal<String>,
}

fn it_works(props: &SomeProps) {
    let SomeProps { a, b, c } = props;
    let a = a.promote();
    let b = b.promote();
    let c = c.promote();
}
