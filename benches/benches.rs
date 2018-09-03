extern crate iconic_test_task;
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

extern crate rand;
use iconic_test_task::*;

const LIST_SIZE: u32 = 20;
const MIN_PRICE: Price = 100;
const MAX_PRICE: Price = 1000;
const MIN_SIZE: Size = 10;
const MAX_SIZE: Size = 100;
const MIN_META: Meta = 10;
const MAX_META: Meta = 100;

struct ListWrapper {
    lst: List,
    issues: Vec<(Price, Size)>,
    splitted: Vec<List>,
}

impl ListWrapper {
    fn base(list: List, issues: Vec<(Price, Size)>) -> ListWrapper {
        ListWrapper {
            lst: list,
            issues: issues,
            splitted: vec![],
        }
    }

    fn from(lst: List, issues: Vec<(Price, Size)>, splitted: Vec<List>) -> ListWrapper {
        ListWrapper {
            lst: lst,
            issues: issues,
            splitted: splitted,
        }
    }

    fn separate(self) -> (List, Vec<(Price, Size)>, Vec<List>) {
        (self.lst, self.issues, self.splitted)
    }
}

fn split_routine(inpt: ListWrapper) -> ListWrapper {
    let (mut list, issues, mut splitted) = inpt.separate();
    for i in issues.iter() {
        splitted.push(black_box(list.split(*i)))
    }
    ListWrapper::from(list, issues, splitted)
}

fn split_benchmark(c: &mut Criterion) {
    c.bench_function("split", |b| {
        b.iter_with_setup(
            || ListWrapper::base(gen_list(100), vec![gen_issue()]),
            split_routine,
        )
    });
}

fn split_3_benchmark(c: &mut Criterion) {
    c.bench_function("split_3", |b| {
        b.iter_with_setup(
            || ListWrapper::base(gen_list(100), vec![gen_issue(), gen_issue(), gen_issue()]),
            split_routine,
        )
    });
}

fn split_bigger_benchmark(c: &mut Criterion) {
    c.bench_function("split_bigger", |b| {
        b.iter_with_setup(
            || ListWrapper::base(gen_list(1000), vec![gen_issue()]),
            split_routine,
        )
    });
}

fn add_routine(inpt: (List, Vec<(Price, Size, Meta)>)) -> List {
    let (list, elems) = inpt;
    elems.into_iter().fold(list, |mut lst, i| {
        lst.add(i);
        lst
    })
}

fn add_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter_with_setup(
            || (gen_list(100), (0..100).map(|_| gen_elem()).collect()),
            add_routine,
        )
    });
}

fn add_bigger_benchmark(c: &mut Criterion) {
    c.bench_function("add_bigger", |b| {
        b.iter_with_setup(
            || (gen_list(100), (0..1000).map(|_| gen_elem()).collect()),
            add_routine,
        )
    });
}

criterion_group!(
    benches,
    split_benchmark,
    split_3_benchmark,
    split_bigger_benchmark,
    add_benchmark,
    add_bigger_benchmark
);
criterion_main!(benches);

fn gen_list(n: u64) -> List {
    use self::rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let mut v: Vec<_> = (0..n)
        .map(|_| rng.gen_range(MIN_PRICE, MAX_PRICE))
        .collect();
    v.sort();
    List::from(
        v.iter()
            .map(|v| {
                (
                    *v,
                    (0..LIST_SIZE)
                        .map(|_| {
                            (
                                rng.gen_range(MIN_SIZE, MAX_SIZE),
                                rng.gen_range(MIN_META as u32, MAX_META as u32) as Meta,
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

fn gen_issue() -> (Price, Size) {
    use self::rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    (
        rng.gen_range(MIN_PRICE, MAX_PRICE),
        rng.gen_range(MIN_SIZE, MAX_SIZE),
    )
}

fn gen_elem() -> (Price, Size, Meta) {
    use self::rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    (
        rng.gen_range(MIN_PRICE, MAX_PRICE),
        rng.gen_range(MIN_SIZE, MAX_SIZE),
        rng.gen_range(MIN_META as u32, MAX_META as u32) as Meta,
    )
}
