trait NonCompare: Sized {
    fn compare(&self, other: &Self) -> bool;
}

impl<T> NonCompare for &&T {
    fn compare(&self, other: &Self) -> bool {
        false
    }
}

trait CanCompare: PartialEq {
    fn compare(&self, other: &Self) -> bool;
}

impl<T: PartialEq> CanCompare for T {
    fn compare(&self, other: &Self) -> bool {
        self == other
    }
}

struct CantCompareThis;

fn it_works(can: (&String, &String), cant: (&CantCompareThis, &CantCompareThis)) {
    let (can0, can1) = can;
    let yes = (&can0).compare(&&can1);

    let (cant0, cant1) = cant;
    let no = (&cant0).compare(&&cant1);

    assert!(yes);
    assert!(!no);
}

#[test]
fn test() {
    let can = ("hello".to_string(), "hello".to_string());
    let cant = (CantCompareThis, CantCompareThis);
    it_works((&can.0, &can.1), (&cant.0, &cant.1));
}
