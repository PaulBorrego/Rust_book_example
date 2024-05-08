use std::ops:: {Div,Add};
fn main() {
    println!("Hello, world!");
    let five_feet = Feet(5);
    let six_inches = Inches(6);
    let a = six_inches.clone() + five_feet.clone();
    let b = five_feet.clone()/six_inches.clone();

    println!("5 Feet and 6 inches is {:?} Inches, 5 feet go into 6 inches {:?} times",a, b);

    let a = mult!(4,5,10,12);
    println!("{a}");


    let temp = concat!("this", " is ", "gonna", " be ", "concatonated!");
    let temp_length = add!("this".len(), " is ".len(), "gonna".len(), " be ".len(), "concatonated!".len());

    println!("{temp}\nString has correct capacity of {}. Actual capacity should be {}", temp.capacity(),temp_length);
}

//multiplies all numbers
#[macro_export]
macro_rules! mult {
    ( $( $x:expr ),* ) => {
        {
            let mut a = 1;
            $(
                a *= $x;
            )*
            a
        }
    };
}

//
#[macro_export]
macro_rules! add {
    ( $( $x:expr ),* ) => {
        {
            let mut a = 0;
            $(
                a += $x;
            )*
            a
        }
    };
}

//concatonates multiple strings into one with the correct string capacity 
#[macro_export]
macro_rules! concat {
    ( $( $x:expr ),* ) => {
        {
            let mut a = 0;
            $(
                a += $x.len();
            )*
            let mut s = String::with_capacity(a);
            $(
                s.push_str($x);
            )*

            s
        }
    };
}




#[derive(Debug,Clone)]
struct Feet(u32);

#[derive(Debug,Clone)]
struct Inches(u32);


impl Add<Feet> for Inches {
    type Output = Inches;

    fn add(self, other: Feet) -> Inches {
        Inches(self.0 + (other.0 * 12))
    }
}

impl Div<Inches> for Feet {
    type Output = Feet;

    fn div(self, other: Inches) -> Feet {
        Feet(self.0 * 12 / (other.0))
    }
}
