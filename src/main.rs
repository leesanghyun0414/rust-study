use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

const NUM: i128 = 4564564564654865;

fn main() {
    let mv = vec![1];
    let _mv2 = mv.clone();
    let from = 1;
    let _on = from;
    println!("{}", from);
    let fruit = FruitProduct {
        name: "Banana".to_string(),
        price: 1000,
        quality: ProductQuality::High,
    };

    // println!("{} {} {}", fruit.name, fruit.price, fruit.quality);
    print!("{}", fruit.name);
    println!("test");
    let oo = triangle(NUM);
    let oo2 = built_in_triangle(NUM);
}

#[derive(Debug)]
enum ProductQuality {
    High,
}

impl fmt::Display for ProductQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Fruit")
    }
}

struct FruitProduct {
    name: String,
    price: usize,
    quality: ProductQuality,
}

trait FruitQualityManager {
    fn count_selected_products<F>(fruits: &Vec<FruitProduct>, test_fn: F) -> usize
    where
        F: Fn(&FruitProduct) -> bool;
}

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }
}

fn triangle(n: i128) -> i128 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

fn built_in_triangle(n: i128) -> i128 {
    (1..n + 1).fold(0, |sum, item| sum + item)
}
