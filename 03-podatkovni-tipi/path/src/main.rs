// Ponovitev

// struct Par {
//     first: u8,
//     second: u8,
// }

// use Par as P;


// impl P {
//     fn nov_par(f: u8, s: u8) -> P {
//         P {
//             first: f,
//             second: s,
//         }
//     }

//     fn fst(&self) -> u8 {
//         self.first
//     }
// }

// Real deal

struct AritmeticnoZaporedje {
    prvi_clen: i32,
    korak: i32,
    trenutni_clen : i32,
}

use AritmeticnoZaporedje as AZ;

impl AZ {
    fn new(p: i32, k: i32,) -> AZ {
        return AZ {
            prvi_clen: p,
            korak: k,
            trenutni_clen: p,
        };
    }

    fn next(&mut self) -> i32 {
        let tiho_bodi = self.trenutni_clen;
        self.trenutni_clen = self.trenutni_clen + self.korak;
        return tiho_bodi
    }

    fn n_th(&self, n: i32) -> i32 {
        return self.prvi_clen + (n-1) * self.korak
    }

    fn reset(&mut self) {
        self.trenutni_clen = self.prvi_clen
    }
    
    fn current(&self) -> i32 {
        return self.trenutni_clen
    }

    fn sum(&self, n: u32) -> i32 {
        let mut vsota = 0;
        for i in 0..n {
            vsota += self.n_th(i.try_into().unwrap())
        }
        return vsota;
    }

    fn vsota(&self, other : &Self) -> AZ {
        return AZ {
            prvi_clen: self.prvi_clen + other.prvi_clen,
            korak: self.korak + other.korak,
            trenutni_clen: self.trenutni_clen + other.trenutni_clen
        }
    }
    
}





fn main() {
    let mut a = AZ::new(1, 1);
    println!("{},{}", a.next(), a.next());
    println!("{}", a.n_th(1000000000));
    println!("{}",a.current());
    a.reset();
    println!("{}",a.current());
    let cognito_ergo = AZ::new(2, 3);
    println!("{}",cognito_ergo.sum(100));
    println!("{}",a.sum(100));
    println!("{}",a.vsota(&a).current());
    // println!("{}",)
    // println!("{}",)
    // println!("{}",)
    // println!("{}",)
    // println!("{}",)
    // println!("{}",)
    // let izraz = Izraz 
    //     Operacija (Izraz{Konstanta(1)}, BinOperacija {Plus}, Izraz{Operacija(Izraz{Konstanta(2), BinOperacija {Krat}, Izraz{Konstanta(3)} }) })
    let izraz = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija
            (Box::new(Izraz::Konstanta(2)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(3)),)
        ),
    );

    println!("{}",izraz.eval());
    println!("{}",izraz.eval());
    println!("{}",izraz.collect());
    println!("{}",izraz.izpis());

}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

impl Izraz {
    fn eval(&self) -> u32 {
        match self {
            Izraz::Konstanta(n) => *n,
            Izraz::Operacija (izraz1, operacija, izraz2) =>
                match operacija {
                    BinOperacija::Plus => izraz1.eval() + izraz2.eval(),
                    BinOperacija::Minus => izraz1.eval() - izraz2.eval(),
                    BinOperacija::Times => izraz1.eval() * izraz2.eval(),
                }
        }
    }

    fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Operacija (izraz1, _ , izraz2) =>
            izraz1.collect() + izraz2.collect(),
        }
    }

    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(n) => format!("{}",*n),
            Izraz::Operacija (izraz1, operacija, izraz2) =>
                match operacija {
                    BinOperacija::Plus => format!("({} + {})", izraz1.izpis(), izraz2.izpis()),
                    BinOperacija::Minus => format!("({} - {})", izraz1.izpis(), izraz2.izpis()),
                    BinOperacija::Times => format!("({} * {})", izraz1.izpis(), izraz2.izpis()),
                }
        }
    }

}

// const IZRAZ: Izraz = Izraz {
//     Operacija (Izraz{Konstanta(1)}, BinOperacija {Plus}, Izraz{Operacija(Izraz{Konstanta(2), BinOperacija {Krat}, Izraz{Konstanta(3)} }) })
// }
