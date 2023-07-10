trait SurfaceInformation {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl SurfaceInformation for Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn perimeter(&self) -> u32 {
        2 * (self.length + self.width)
    }
}

impl SurfaceInformation for Triangle {
    fn area(&self) -> u32 {
        (0.5 * self.b as f32 * self.h as f32) as u32
    }
    fn perimeter(&self) -> u32 {
        self.a + self.b + self.c
    }
}

impl SurfaceInformation for Square {
    fn area(&self) -> u32 {
        self.length * self.length
    }
    fn perimeter(&self) -> u32 {
        4 * self.length
    }
}

struct Rectangle {
    length: u32,
    width: u32,
}

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
    h: u32,
}

struct Square {
    length: u32,
}

trait DisplayShape{
    fn length(&self)->u32;
}

impl DisplayShape for Square{
    fn length(&self)->u32{
        return self.length;
    }
}

fn main() {
    let rec = Rectangle {
        length: 10,
        width: 5,
    };
    let tri = Triangle {
        a: 4,
        b: 3,
        c: 2,
        h: 2,
    };
    let squ = Square { length: 10 };

    println!("{} {}", rec.area(), rec.perimeter());
    println!("{} {}", tri.area(), tri.perimeter());
    println!("{} {}", squ.area(), squ.perimeter());
    print_square(&squ)
}

fn print_square<T: DisplayShape>(squ: &T) {
    let  x = squ.length();
    let horizontal = "..".repeat(x as usize);
    let vertical = format!(": {} :", "  ".repeat((x - 2) as usize));

    for i in 0..x {
        if i == 0 || i == x - 1 {
            println!("{}", horizontal);
        } else {
            println!("{}", vertical);
        }
    }
}
