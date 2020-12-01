fn fat (n: u64) -> u64 {
    if (n == 0) {
        return 1;
    }
    return n*fat(n-1);
}

fn main() {

    let n = std::env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(10);

    let f;

    f = fat(n);

    println!("Fatorial de {} é {}", n, f);

}