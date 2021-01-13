use std::ops::Add;
use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,

}

// 算術演算子「+」のオーバーロード
// impl<T> Add for Complex<T>
//     where T: Add<Output=T>
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Complex { re: self.re, im: self.im + rhs.im }
//     }
// }

// 上のコメントアウトされたAdd for Complexを最大限ジェネリックにした実装
// 実際にはLがAdd<R, Output=O>を実装する制約があるため、L, R, Oは同じ型になり、上の単純な実装と大差ない
impl<L, R, O> Add<Complex<R>> for Complex<L>
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

// 単項演算子「-」のオーバーロード
impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex { re: -self.re, im: -self.im }
    }
}

fn main() {
    // std::ops::Addトレイトをスコープ内でuseすると
    // a + bをa.add(b)と関数呼び出しでも書ける
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}
