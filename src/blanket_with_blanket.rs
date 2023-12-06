use std::marker::PhantomData;

trait NonPromote: Sized {
    fn promote(&self) -> &Self {
        self
    }
}

impl<T> NonPromote for &&T {}

trait SignalPromote: Clone + Copy {
    fn promote(&self) -> Self {
        *self
    }
}
impl<T: Clone + Copy> SignalPromote for T {}

struct Signal<T>(PhantomData<T>);
impl<T> Copy for Signal<T> {}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

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

    assert_a(a);
    assert_b(b);
    assert_c(c);
}

fn assert_a(b: i32) {}
fn assert_b(b: &String) {}
fn assert_c(b: Signal<String>) {}
