use std::io::{Result, Write};
use std::ops::Neg;
use std::ops::{Deref, DerefMut, Drop};
mod first_mod;

fn main() {
    let mut raw = 10;
    raw = -raw;
    println!("{}", raw);
    let dj = first_mod::Dj {
        name: "".to_string(),
        music_genre: vec!["f".to_string()],
    };
    println!("{} : {}", dj.name, dj.music_genre[0]);
    let le = Vec::<i32>::with_capacity(100);
    for el in le {
        println!("{}", el.to_string());
    }

    let is_even = |x: u64| -> bool { x % 2 == 0 };
    assert_eq!(is_even(12), true);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 100,
        health: 100,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);

    let range_num = 1..9;
    let collected_range = collect_into_vector(range_num);
    for num in collected_range {
        println!("{}", num.to_string());
    }
    let a = "aa".to_string();
    let b = &a;
    println!("{}", b);
    let wine = Appellation {
        name: "A".to_string(),
        nicknames: vec!["S".to_string(), "f".to_string()],
    };
    println!("{}", wine.name);

    let tree = Mangrove {};
    tree.branch();
    Mangrove::big_branch();
    let lotto_slot = Selector {
        elements: vec![1, 2, 3, 5, 4, 44, 88],
        current: 0,
    };
    println!("{}", *lotto_slot);
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number.
    re: T,

    /// Imaginary portion of the complex number.
    im: T,
}

fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!();
    }
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

trait Tree {
    fn branch(&self) {
        println!("ss")
    }
}

trait BigTree: Tree {
    fn big_branch() {
        println!("It's big.")
    }
}

struct Mangrove;

impl Tree for Mangrove {}

impl BigTree for Mangrove {}
