fn main() {
    let n = 19; // Example value for n
    let generators = find_generators(n);
    
    println!("Generators of the additive group Z_{}: {:?}", n, generators);
}

/// Finds the generators of the additive group Z_n
fn find_generators(n: u32) -> Vec<u32> {
    let mut generators = Vec::new();
    
    for candidate in 1..n {
        if is_generator(candidate, n) {
            generators.push(candidate);
        }
    }
    
    generators
}

/// Checks if a given number is a generator of the additive group Z_n
fn is_generator(g: u32, n: u32) -> bool {
    let mut current = g;
    let mut count = 1;

    while current != 0 {
        current = (current + g) % n;
        count += 1;
        if count == n {
            break;
        }
    }

    count == n
}

