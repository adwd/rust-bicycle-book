#[derive(Debug, Clone, Copy)]
struct Parent(usize, Child, Child);

#[derive(Debug, Clone, Copy)]
struct Child(usize);

pub fn copy_semantics() {
    println!("copy semantics");
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 値の所有権をp1からp2にムーブする
    println!("p2: {:?}", p2);
    println!("p1: {:?}", &p1);
}
