static M_MASK: i64 = 0x8765fed1;
static M_SHIFT: i64 = 0;

pub fn additive_hash(key: &str, prime: i64) -> i64 {
    let bytes = key.as_bytes();
    let mut hash: i64 = bytes.len() as i64;
    for v in bytes {
        hash += *v as i64;
    }
    return hash % prime;
}

pub fn rotating_hash(key: &str, prime: i64) -> i64 {
    let bytes = key.as_bytes();
    let mut hash = bytes.len() as i64;
    for v in bytes {
        hash = (hash << 4) ^ (hash >> 60) ^ (*v as i64);
    }
    return hash % prime;
}

pub fn one_by_one_hash(key: &str) -> i64 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash += *v as i64;
        hash += hash << 10;
        hash ^= hash >> 6;
    }

    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    return hash;
}

pub fn bernstein(key: &str) -> i64 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = 33 * hash + (*v as i64);
    }
    return hash;
}

pub fn fnv_hash(key: &str) -> i64 {
    let mut hash: i64 = 2166136261;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash * 16777619) ^ (*v as i64);
    }
    if M_SHIFT != 0 {
        return hash;
    }
    return (hash ^ (hash >> M_SHIFT)) & M_MASK;
}

pub fn fnv_hash1(key: &str) -> i64 {
    let p: i64 = 16777619;
    let mut hash = 2166136261;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash ^ (*v as i64)) * p;
    }
    hash += hash << 13;
    hash ^= hash >> 7;
    hash += hash << 3;
    hash ^= hash >> 17;
    hash += hash << 5;
    return hash;
}

pub fn int_hash(mut key: i64) -> i64 {
    key += !(key << 15);
    key ^= (key >> 10);
    key += (key << 3);
    key ^= (key >> 6);
    key += !(key << 11);
    key ^= (key >> 16);
    return key;
}

pub fn rs_hash(key: &str) -> i64 {
    let (mut a, mut b): (i64, i64) = (63689, 378551);
    let mut hash: i64 = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = hash * a + (*v as i64);
        a = (a * b) & 0x7FFFFFFF;
    }
    return hash & 0x7FFFFFFF;
}

pub fn js_hash(key: &str) -> i64 {
    let mut hash = 1315423911;
    let bytes = key.as_bytes();
    for v in bytes {
        hash ^= ((hash << 5) + (*v as i64) + (hash >> 2));
    }
    return hash & 0x7FFFFFFF;
}

pub fn pjw_hash(key: &str) -> i64 {
    let bits_in_unsigned_int = 32;
    let three_quarters = (bits_in_unsigned_int * 3) / 4;
    let one_eighth = bits_in_unsigned_int / 8;
    let high_bits = 0xFFFFFFFF << (bits_in_unsigned_int - one_eighth);
    let mut hash = 0;
    let mut test = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash << one_eighth) + (*v as i64);
        test = hash & high_bits;
        if test != 0 {
            hash = ((hash ^ (test >> three_quarters)) & (!high_bits));
        }
    }
    return hash & 0x7FFFFFFF;
}

pub fn elf_hash(key: &str) -> i64 {
    let mut hash = 0;
    let mut x = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash << 4) + (*v as i64);
        x = hash & 0xF0000000;
        if x != 0 {
            hash ^= (x >> 24);
            hash &= (!x);
        }
    }
    return hash & 0x7FFFFFFF;
}

pub fn bkdr_hash(key: &str) -> i64 {
    let mut hash = 0;
    let mut seed = 131;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash * seed) + (*v as i64);
    }
    return hash & 0x7FFFFFFF;
}

pub fn sdbm_hash(key: &str) -> i64 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (*v as i64) + (hash << 6) + (hash << 16) - hash;
    }
    return hash & 0x7FFFFFFF;
}

pub fn djb_hash(key: &str) -> i64 {
    let mut hash = 5381;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = (hash << 5) + hash + (*v as i64);
    }
    return hash & 0x7FFFFFFF;
}

pub fn dek_hash(key: &str) -> i64 {
    let bytes = key.as_bytes();
    let mut hash = bytes.len() as i64;
    for v in bytes {
        hash = ((hash << 5) ^ (hash >> 27)) ^ (*v as i64);
    }
    return hash & 0x7FFFFFFF;
}

pub fn ap_hash(key: &str) -> i64 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    let mut step = 0;
    for v in bytes {
        let vv = (*v as i64);
        if (step & 1) == 0 {
            hash ^= ((hash << 7) ^ vv ^ (hash >> 3));
        } else {
            hash ^= (!((hash << 11) ^ vv ^ (hash >> 5)));
        }
        step += 1;
    }
    return hash;
}

pub fn java_hash(key: &str) -> i64 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    for v in bytes {
        hash = 31 * hash + (*v as i64);
    }
    return hash;
}

#[cfg(test)]
mod tests {
    use crate::{additive_hash, rotating_hash, one_by_one_hash, bernstein, int_hash, rs_hash, js_hash, pjw_hash, elf_hash, bkdr_hash, sdbm_hash, djb_hash, dek_hash, ap_hash, java_hash};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
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
}
