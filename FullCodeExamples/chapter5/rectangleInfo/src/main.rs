#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rect1 is {:?}", rect1); //With :? we are stating that we want rust to output the Debug info from the rectangle rect1, this is because structs doesnt have doesnt implement the fromatting known as Display:
    area(rect1);
}

fn area(r: Rectangle) -> u32 {
    println!(
        "This rectangle is {:?} and it has an area of {}",
        r,
        r.width * r.height
    );
    r.width * r.height
}
