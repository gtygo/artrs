use std::sync::atomic::{AtomicU64, Ordering};
use core::arch::x86_64;
use std::thread;


const MAX_PREFIX_LEN: usize = 8;
const MAX_KEY_LEN: usize = 1024;
const MAX_VALUE_LEN: usize = 1024;
const LEAF_TYPE: u8 = 1;
const N4_COUNT: usize = 4;
const N16_COUNT: usize = 16;
const N48_COUNT: usize = 48;
const N256_COUNT: usize = 256;
const OBSOLETE_FLAG: u64 = 1;
// 0 0 0 1
const LOCKED_FLAG: u64 = 2;
// 0 0 1 0
const LOCKED_AND_OBSOLETE_FLAG: u64 = 3;
// 0 0 1 1


const SPIN_COUNT: u64 = 64;


pub enum Type {
    N4 = 1,
    N16 = 2,
    N48 = 3,
    N256 = 4,

}

pub struct Node {
    pub n_type: Type,
    pub n_child: u8,
    pub version: AtomicU64,
    pub prefix_len: u64,
    pub prefix: [u8; MAX_PREFIX_LEN],
    pub prefix_leaf: Option<Box<Leaf>>,
}

struct Node4 {
    pub n: Node,
    pub keys: [u8; N4_COUNT],
    pub children: [Box<Node>; N4_COUNT],
}

struct Node16 {
    pub n: Node,
    pub keys: [u8; N16_COUNT],
    pub children: [Box<Node>; N16_COUNT],
}

struct Node48 {
    pub n: Node,
    pub keys: [u8; N48_COUNT],
    pub children: [Box<Node>; N48_COUNT],
}

struct Node256 {
    pub n: Node,
    pub children: [Box<Node>; N256_COUNT],
}

pub struct Leaf {
    pub n_type: u8,
    pub key: Box<[u8]>,
    pub value: Box<[u8]>,
}

impl Node {
    pub fn new(typ: Type) -> Self {
        let pfx: [u8; MAX_PREFIX_LEN] = [0; MAX_PREFIX_LEN];
        Node {
            n_type: typ,
            n_child: 0,
            version: AtomicU64::new(0),
            prefix_len: 0,
            prefix: pfx,
            prefix_leaf: None,
        }
    }

    //sync
    pub fn write_lock(&self) -> bool {
        let v: u64 = 0;
        while true {
            if !self.read_lock(v) {
                return false;
            }
            if self.upgrade_to_write_lock(v) {
                break;
            }
        }
        return true;
    }

    pub fn write_unlock(&self) {
        self.version.fetch_add(LOCKED_FLAG, Ordering::SeqCst);
    }

    pub fn write_unlock_obsolete(&self) {
        self.version.fetch_add((LOCKED_FLAG | OBSOLETE_FLAG), Ordering::SeqCst);
    }

    // spin lock
    pub fn await_unlock(&self) -> u64 {
        let mut v = self.version.load(Ordering::SeqCst);
        let mut c = SPIN_COUNT;
        while (v & LOCKED_FLAG) == LOCKED_FLAG {
            if c == 0 {
                //todo: Need yield() ?
                // thread::yield_now();
                unsafe {
                    core::arch::x86_64::_mm_pause()
                }
                c = SPIN_COUNT;
            }
            v = self.version.load(Ordering::SeqCst);
        }
        return v;
        // self.version.compare_exchange(11, 12, Ordering::SeqCst, Ordering::SeqCst);
    }

    pub fn is_obsolete(&self, version: u64) -> bool {
        return (version & OBSOLETE_FLAG) == OBSOLETE_FLAG;
    }

    pub fn read_lock(&self, &version: u64) -> bool {
        let v = self.wait_unlock();
        if self.is_obsolete(v) {
            version = 0;
            return false;
        }
        version = v;
        return true;
    }

    pub fn read_unlock(&self, version: u64) -> bool {
        return version == self.version.load(Ordering::SeqCst);
    }

    pub fn r_unlock_with_node() {}

    pub fn upgrade_to_write_lock(&self, version: u64) {
        // todo : 'SeqCst' -> 'Relaxed' ?
        return self.version.compare_exchange(version, version + LOCKED_FLAG, Ordering::SeqCst, Ordering::SeqCst);
    }

    pub fn upgrade_to_write_lock_with_node() {}

    pub fn check() {}
}

impl Leaf {
    pub fn new(key: &[u8], value: &[u8]) -> Leaf {
        Leaf {
            n_type: LEAF_TYPE,
            key: Box::from(key),
            value: Box::from(value),
        }
    }
}
