use std::ops::Drop;
mod copy;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    println!("Hello, world!");

    let p1 = Parent(1, Child(11), Child(12));

    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }

    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);

    move_semantics();

    copy::copy_semantics();
}

fn move_semantics() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 値の所有権をp1からp2にムーブする
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", &p1); // p1は値の所有権を失ったためアクセス不可
    // → error[E0382]: borrow of moved value: `p1`

    p1 = Parent(2, Child(21), Child(22)); // p1を別の値に束縛する
    println!("p1: {:?}", p1); // p1は別の値の所有権を持つためアクセスできる
}
