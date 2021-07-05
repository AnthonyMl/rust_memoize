use std::collections::{HashMap};
use std::hash::{Hash};


fn memoize<'a, I: Hash + Eq + PartialEq + Clone + 'a, O: Clone + 'a, F: Fn(I) -> O + 'a>
    (f: F) -> Box<dyn FnMut(I) -> O + 'a>
{
    let mut map: HashMap<I, O> = HashMap::new();

    Box::new(move|i: I| {
        map.entry(i.clone()).or_insert_with(||f(i)).clone()
    })
}

fn double(a: i32) -> i32 {
    println!("calling double({})", a);
    a * 2
}

fn main() {
    let mut memo_double = memoize(double);

    println!("values: {} {} {} {} {}",
        memo_double(4),
        memo_double(5),
        memo_double(5),
        memo_double(4),
        memo_double(6)
    );
}
