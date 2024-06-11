use std::collections::HashSet;

use fraction::Ratio;

type Base = u64;

fn main() {
    let mut complexity: Vec<HashSet<Ratio<Base>>> = Vec::new();

    complexity.push(HashSet::new());

    let mut set_one = HashSet::new();
    set_one.insert(Ratio::ONE);
    complexity.push(set_one);

    print_layer(&complexity);
    
    for _ in 2..31 {
        add_layer(&mut complexity);
        print_layer(&complexity);
    }
}

fn add_layer(complexity: &mut Vec<HashSet<Ratio<Base>>>) {
    let n = complexity.len();
    let mut layer = HashSet::new();
    for i in 1..(n / 2 + 1) {
        let j = n - i;
        let i_set = &complexity[i];
        let j_set = &complexity[j];
        for i in i_set {
            for j in j_set {
                let sum = i + j;
                let product = i * j;
                let quotient = i / j;
                let inverse_quotient = j / i;
                let combinations: [Ratio<Base>; 4] = [sum, product, quotient, inverse_quotient];
                for combination in combinations {
                    if !contains(complexity, combination) {
                        layer.insert(combination);
                    }
                }
            }
        }
    }

    complexity.push(layer);
}

fn contains(complexity: &[HashSet<Ratio<Base>>], value: Ratio<Base>) -> bool {
    for set in complexity {
        if set.contains(&value) {
            return true;
        }
    }
    false
}

fn print_layer(complexity: &[HashSet<Ratio<Base>>]) {
    println!("Layer {}:", complexity.len() - 1);
    let Some(set) = complexity.last() else {
        return;
    };

    let mut set: Vec<Ratio<Base>> = set.iter().copied().collect();
    set.sort_unstable();
    println!("Total: {}", set.len());
    for item in set {
        print!("{}, ", item);
    }
    println!();
    println!();
}
