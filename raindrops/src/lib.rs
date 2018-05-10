
pub struct Factors {
    a: bool,
    b: bool,
    c: bool,
}

pub fn raindrops(n: usize) -> String {
    let factors = Factors { a: n % 3 == 0, b: n % 5 == 0 , c: n % 7 == 0};
    match factors {
        Factors {a: true, b: false, c: false}  => format!("{}", "Pling"),
        Factors {a: false, b: true, c: false}  => format!("{}", "Plang"),
        Factors {a: false, b: false, c: true}  => format!("{}", "Plong"),
        Factors {a: true, b: true, c: false}   => format!("{}", "PlingPlang"),
        Factors {a: true, b: true, c: true}    => format!("{}", "PlingPlangPlong"),
        Factors {a: false, b: true, c: true}   => format!("{}", "PlangPlong"),
        Factors {a: true, b: false, c: true}   => format!("{}", "PlingPlong"),
        Factors {a: false, b: false, c: false} => format!("{}", n)
    }
}

