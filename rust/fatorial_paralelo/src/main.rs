use std::thread;

fn fat (n: i64) -> u64 {
    if (n > 0) {
        return (n as u64)*fat(n-2);
    }
    return 1;
}

fn main() {

    let n = std::env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(10);

    let mut f;

    let child = thread::spawn( move|| {
        fat(n-1)
    });
    f = fat(n);

    let res = child.join().expect("Deu ruim");

    f = f*res;

    println!("Fatorial de {} é {}", n, f);

}