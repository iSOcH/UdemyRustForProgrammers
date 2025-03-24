// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner.

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, AddAssign},
};

fn main() {
    let mut map = HashMap::new();

    let old_value1 = map.insert(1, -1);
    match old_value1 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present before!"),
    }

    let old_value2 = map.insert(1, 2);
    match old_value2 {
        Some(val) => println!("Old value: {}", val),
        None => println!("Key was not present before!"),
    }

    map.insert(5, 10);

    println!("{:?}", map);

    println!("Contains 5?: {}", map.contains_key(&5));
    println!("Contains 4?: {}", map.contains_key(&4));

    let removed_value = map.remove(&5);

    match removed_value {
        Some(val) => println!("Removed value: {}", val),
        None => println!("Key was not present before!"),
    }

    for k in map.keys() {
        println!("key: {k}");
    }

    for v in map.values() {
        println!("val: {v}");
    }

    let search_key = 1;
    let got_value = map.get(&search_key).copied().unwrap_or(0);
    println!("got_value: {}", got_value);

    // more efficiently addIfNotPresent
    let existing_entry = map.entry(1);
    existing_entry.or_insert(3); // will not change existing entry
    println!("After entry(1).or_insert(3): {:?}", map);

    // increment counter
    increment_or_add(&mut map, 2, 1);
    println!(r#"After entry(2) "incrementOrAdd": {:?}"#, map);
    increment_or_add(&mut map, 2, 1);
    println!(r#"After entry(2) "incrementOrAdd again": {:?}"#, map);
    increment_or_add(&mut map, 1, 1);
    println!(r#"After entry(2) "incrementOrAdd": {:?}"#, map);
}

fn increment_or_add<K, V>(map: &mut HashMap<K, V>, key: K, step: V)
where
    K: Eq + Hash,
    V: Add<Output = V> + AddAssign + Default,
{
    let entry = map.entry(key);

    // cannot call both methods (or_insert and and_mutate) on `entry` since they consume entry
    match entry {
        std::collections::hash_map::Entry::Vacant(empty) => {
            empty.insert(V::default() + step);
        }
        std::collections::hash_map::Entry::Occupied(mut existing) => {
            *existing.get_mut() += step;
        }
    }
}
