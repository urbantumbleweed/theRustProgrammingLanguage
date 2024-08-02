use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let intensity = 90;
    let mut cache = Cacher::new(|intensity| {
        expensive_function(intensity)
    });

    println!(
        "Today, do {} pushups!",
        cache.value(intensity)
    );

    println!(
        "Today, do {} pushups!",
        cache.value(intensity)
    );

    println!("Take a break today! Remember to stay hydrated!");

    let intensity = 29;

    println!(
        "Today, run for {} minutes!",
        cache.value(intensity)
    );
}

fn expensive_function(intensity: i32) -> i32 {
    println!("calculating slowing....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T, U, V> 
where 
    T: Fn(U) -> V,
    U: Eq + std::hash::Hash + Copy,
    V: Copy,
{
    calculate: T,
    value: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
where 
    T: Fn(U) -> V,
    U: Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calculator: T) -> Cacher<T, U, V> {
        Cacher {
            calculate: calculator,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        if let Some(&v) = self.value.get(&arg) {
            v
        } else {
            let v = (self.calculate)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

#[test]

fn persist_single_value() {
    let v = 20;
    let mut cache = Cacher::new(|x| x);

    assert_eq!(v, cache.value(20))
}
#[test]
fn cache_multiple_items() {
    let v1 = 20;
    let v2: i32 = 55;
    let mut cache = Cacher::new(|x| x);

    assert_eq!(v1, cache.value(v1));
    assert_eq!(v2, cache.value(v2));
    assert_ne!(cache.value(v1), cache.value(v2));
}