mod buffer;
mod compareString;
mod iterator;

fn main() {
    let mut buffer = buffer::Buffer::new();
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    let sum = buffer.sum();
    println!("Buffer 内元素和: {:?}", sum);

    let x = "create";
    let y = "rust";

    let result = compareString::compareString(x, y);

    if result {
        println!("{} 字典序更大", x);
    } else {
        println!("{} 字典序更大", y);
    }

    let res: Vec<char> = iterator::iterator();
    println!("{:?}", res);
}
