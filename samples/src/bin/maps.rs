use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![1, 2, 5, 3, 5, 10, 3, 100, 5, 4, 4, 4, 0];
    let mut freq: HashMap<i32, usize> = HashMap::new();

    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    assert_eq!(freq.insert(3, 10), Some(2));
    assert_eq!(freq.insert(256, 10), None);

    for (x, fx) in &mut freq {
        *fx *= 2;
        println!("{}\t{}", *fx, *x);
    }

    println!("size: {} empty: {}", freq.len(), freq.is_empty());

    assert_eq!(freq.get(&3), Some(&20));
    assert_eq!(freq.get(&200), None);

    assert_eq!(freq.remove(&3), Some(20));
    assert_eq!(freq.remove(&3), None);

    for _ in 0..3 {
        let x: i32 = 256;
        print!("freq[{}] = {}, ", x, *freq.get(&x).unwrap_or(&0));
        if let Entry::Occupied(mut e) = freq.entry(256) {
            if *e.get() % 2 == 0 {
                println!("increment");
                *e.get_mut() += 1
            } else {
                println!("remove");
                e.remove();
            }
        } else {
            println!("not found");
        }
    }
}
