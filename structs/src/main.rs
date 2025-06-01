#[derive(Debug)]
struct Racktangle {
    width: u32,
    height: u32,
}

impl Racktangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Racktangle {
    fn square(size: u32) -> Racktangle {
        Racktangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Racktangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    dbg!(rect1);
    println!("Square = {:?}", Racktangle::square(20));
}
