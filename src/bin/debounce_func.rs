use std::cell::Cell;
use std::thread::sleep;
use std::time::Duration;

fn debounce<F>(mut func: F, delay: Duration) -> impl FnMut()
where
    F: FnMut(),
{
    let mut timer: Option<std::time::Instant> = None;
    move || match timer {
        Some(ref mut t) => {
            if t.elapsed() >= delay {
                func();
                *t = std::time::Instant::now();
            }
        }
        None => {
            func();
            timer = Some(std::time::Instant::now());
        }
    }
}

fn main() {
    let count = Cell::new(0);
    let mut debounced = debounce(|| count.set(count.get() + 1), Duration::from_millis(100));

    debounced(); // will run immediately
    debounced(); // will be ignored
    sleep(Duration::from_millis(200));
    debounced(); // will run again
    println!("{}", count.get()); // prints "2"
}
