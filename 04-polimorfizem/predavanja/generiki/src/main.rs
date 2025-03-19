use std::cmp::PartialOrd;

fn prestej_i32(v: &Vec<i32>) -> usize {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn prestej_f64(v: &Vec<f64>) -> usize {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

fn prestej<T>(v: &Vec<T>) -> usize {
    let mut stevilo = 0;
    for _ in v {
        stevilo += 1;
    }
    stevilo
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(PartialEq)]
struct Tocka<T, O> {
    x: T,
    y: T,
    oznaka: O,
}

impl<O, T> Tocka<T, O> {
    fn abscisa(&self) -> &T {
        &self.x
    }

    fn naredi_novo_tocko_z_oznako_drugega_tipa<P>(self, oznaka: P) -> Tocka<T, P> {
        Tocka {
            x: self.x,
            y: self.y,
            oznaka: oznaka,
        }
    }
}

impl<O> Tocka<f64, O> {
    fn absolutna_vrednost(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // fn abscisa(self) -> f64 {
    //     self.x
    // }
}

impl<O> Tocka<i32, O> {
    fn absolutna_vrednost(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl Tocka<f64, String> {
    fn to_string(&self) -> String {
        String::from("točka")
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let n1 = prestej(&v1);
    let n10 = prestej(&v1);
    println!("Dolžina prvega vektorja je {n1}");
    let v2 = vec![1., 2., 3., 4., 5., 6.];
    let n2 = prestej(&v2);
    println!("Dolžina drugega vektorja je {n2}");
    let p1 = Tocka {
        x: 3.0,
        y: 4.0,
        oznaka: String::from("A"),
    };
    let p2 = Tocka {
        x: 3.0,
        y: 4.0,
        oznaka: String::from("A"),
    };
    println!(
        "Absolutna vrednost točke {} je {}",
        p1.to_string(),
        p2.absolutna_vrednost()
    );
}


trait Sequence<T> {
    fn name(&self) -> String;
    fn k_th(&self,k:usize) -> Option<T>;
    fn contains(&self,item:T) -> bool;
    fn start(&self) -> T; 
}

struct Constant<T> {
    c: T
}

impl<T> Constant<T> {
    fn new(c: T) -> Constant<T>{
        Constant {c}
    }
}

struct ConstantInteger {
    c: i32
}

impl ConstantInteger {
    fn new(c: i32) -> ConstantInteger{
        ConstantInteger  {c}
    }
}

// impl Constant<i32> for ConstantInteger {
//     fn new(c:i32) -> Constant<i32> {
//         Constant{c}
//     }
// }

// impl<T> Sequence<T> for Constant<T> {
//     fn name(&self) -> String{
//         format!("Constant")
//     }
//     fn k_th(&self, k:usize) -> Option<T>{
//         return Some(self.c);
//     }
//     fn contains(&self,item:T) -> bool {
//         return self.c == item;
//     }
//     fn start(&self) -> T {
//         return self.c
//     }
// }


impl Sequence<i32> for Constant<i32> {
    fn name(&self) -> String{
        format!("Constant")
    }
    fn k_th(&self, k:usize) -> Option<i32>{
        return Some(self.c);
    }
    fn contains(&self,item:i32) -> bool {
        return self.c == item;
    }
    fn start(&self) -> i32 {
        return self.c
    }
}


fn test() {}