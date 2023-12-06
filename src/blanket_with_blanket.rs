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

struct SomeProps {
    a: i32,
    b: String,
}

fn it_works(props: &SomeProps) {
    let SomeProps { a, b } = props;
    let a = a.promote();
    let b = b.promote();
}
