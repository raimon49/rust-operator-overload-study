use std::ops::Add;

pub struct Complex<T> {
    pub re: T,
    pub im: T,

}

impl<T> Add for Complex<T>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re, im: self.im + rhs.im }
    }
}

fn main() {
    // std::ops::Addトレイトをスコープ内でuseすると
    // a + bをa.add(b)と関数呼び出しでも書ける
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}
