#[derive(Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vector{{{}, {}}}", self.x, self.y)
    }
}

fn main() {
    let v1 = Vector{x: 10, y: 20};
    let v2 = v1;
    println!("v1 = {}, v2 = {}", v1, v2);
}
