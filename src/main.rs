use std::ops::Add;
use std::ops::Neg;
use std::ops::AddAssign;
use std::cmp::PartialEq;

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

// 複合代入演算子「+=」のオーバーロード
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// std::cmp::PartialEqのeqメソッドとneメソッドのうち、neはデフォルト実装を持つ
// よってeqメソッドのみ実装することでComplex型は「==」で比較可能になる
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

fn main() {
    // std::ops::Addトレイトをスコープ内でuseすると
    // a + bをa.add(b)と関数呼び出しでも書ける
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);

    // std::cmp::PartialEqトレイトを実装している型は比較がassert_eq/assert_neで比較が可能
    // #[derive(PartialEq)]でderive属性に追加しても、ほぼ同じコードが生成される
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_ne!(x, y);

    let s = "d\x6fv\x65t\x61i\x6c".to_string();
    let t = "\x64o\x76e\x74a\x69l".to_string();
    assert!(s == t); // 非Copy値であってもPartialEqは参照で借用されるだけ。所有権は移動しない
}
