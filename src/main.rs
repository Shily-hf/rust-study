#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
fn main() {
    
    let s = Rectangle::square(20);

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("{:?}",s);

    // println!("{}", area(&rect));
    // println!("{:?}", rect);
    // println!("{:#?}", rect);
}
// 第一种写法
// fn area(width: u32,length: u32)->u32{
//     width*length
// }
// 第二种写法
// fn area(dim: (u32,u32))-> u32{
//     dim.0*dim.1
// }
// 第三种写法
// fn area(rect: &Rectangle) -> u32{
//     rect.width * rect.length
// }
