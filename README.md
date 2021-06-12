# hash-algorithms

#### Description
Hash, generally translated as hash, hash, or transliterated as hash, is to transform any length of input (also known as pre image) into fixed length output through hash algorithm, and the output is the hash value. This transformation is a kind of compression mapping, that is, the space of hash value is usually much smaller than the space of input, different inputs may hash into the same output, so it is impossible to determine the unique input value from hash value. In short, it is a function that compresses messages of any length to a message digest of a fixed length.

#### How To Use

```rust
fn main() {
    let hello = "hello";
    println!("additive_hash {}", additive_hash(hello, 131));
    println!("rotating_hash {}", rotating_hash(hello, 131));
    println!("one_by_one_hash {}", one_by_one_hash(hello));
    println!("bernstein {}", bernstein(hello));
    println!("int_hash {}", int_hash(73));
    // println!("{}", rs_hash(hello));
    println!("js_hash {}", js_hash(hello));
    println!("pjw_hash {}", pjw_hash(hello));
    println!("elf_hash {}", elf_hash(hello));
    println!("sdbm_hash {}", sdbm_hash(hello));
    println!("djb_hash {}", djb_hash(hello));
    println!("djb_hash {}", dek_hash(hello));
    println!("ap_hash: {}", ap_hash(hello));
    println!("java_hash {}", java_hash(hello));
}
```

#### License
This library is licensed under MIT license. See LICENSE for details.