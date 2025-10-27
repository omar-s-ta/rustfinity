pub fn countdown(n: u32) -> Vec<u32> {
    (0..=n).rev().collect()
}

pub fn _countdown(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut ns = Vec::new();
    while n != 0 {
        ns.push(n);
        n -= 1;
    }
    ns.push(n);
    ns
}
