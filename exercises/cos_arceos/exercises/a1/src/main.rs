#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::collections::BTreeMap;
use std::vec::Vec;

// I AM NOT DONE

fn test_vec() {
    const N: usize = 3;
    let mut v = Vec::with_capacity(N);
    for i in 0..N {
        v.push(i);
    }
    for i in 0..N {
        assert!(v[i] == i);
    }
    println!("test_vec() OK!");
}

fn test_btree_map() {
    const N: usize = 5;
    let mut m = BTreeMap::new();
    for i in 0..N {
        let key = format!("key_{i}");
        m.insert(key, i as u32);
    }
    for (k, v) in m.iter() {
        if let Some(k) = k.strip_prefix("key_") {
            assert_eq!(k.parse::<u32>().unwrap(), *v);
        }
    }
    println!("test_btree_map() OK!");
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Running memory tests...");

    test_vec();
    test_btree_map();

    println!("Memory tests run OK!");
    println!("\n[ArceOS Tutorial]: A1 okay!");
}
