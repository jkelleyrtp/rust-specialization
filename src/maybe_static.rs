//! does not work yet, sorry :(

trait NotStatic: Sized {
    fn is_static(self) -> (Self, bool);
}

impl<T> NotStatic for &&T
where
    T: 'static,
{
    fn is_static(self) -> (Self, bool) {
        (self, true)
    }
}

trait IsStatic: Sized {
    fn is_static(self) -> (Self, bool);
}

impl<T> IsStatic for T {
    fn is_static(self) -> (Self, bool) {
        (self, false)
    }
}

struct NotBorrowed<'a>(&'a String);

fn it_works<'a>(is: String, isnt: NotBorrowed<'a>) {
    let a = (is).is_static();
    let b = (isnt).is_static();

    dbg!(a.1);
    dbg!(b.1);
}

#[test]
fn test() {
    let s = String::from("hello");
    let s2 = s.clone();

    it_works(s, NotBorrowed(&s2));
}
