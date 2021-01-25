use std::ops::Add;
use std::ops::Neg;
use std::ops::AddAssign;
use std::cmp::PartialEq;
use std::cmp::{Ordering, PartialOrd};

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

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, //inclusive(含まれる)
    upper: T // exclusive(含まれない)
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other { Some(Ordering::Equal) }
        else if self.lower >= other.upper { Some(Ordering::Greater) }
        else if self.upper <= other.lower { Some(Ordering::Less) }
        else { None }
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
    assert_eq!(format!("{} {}", s, t), "dovetail dovetail");

    // IEEE標準では個々のNaN値が他のNaN値と等価でないことを要請する。
    // PartialEq（partial equivalence relation = 部分同値関係）は以下の3つのうち2つのみを満たす
    // x == yならばy ==xも成立
    // x == y && y == zならばx == zも成立
    // x == xは真である
    assert!(f64::is_nan(0.0/0.0));
    assert_eq!(0.0/0.0 == 0.0/0.0, false);
    assert_eq!(0.0/0.0 != 0.0/0.0, true);
    // NaN値との比較は常にfalse
    assert_eq!(0.0/0.0 < 0.0/0.0, false);
    assert_eq!(0.0/0.0 > 0.0/0.0, false);
    assert_eq!(0.0/0.0 <= 0.0/0.0, false);
    assert_eq!(0.0/0.0 >= 0.0/0.0, false);

    // EqとPartialEqのトレイトは実質的に同じ（EqがPartialEqの拡張実装）
    // 実装されたeq()/ne()が部分同値関係か完全同値関係かコンパイラからは分からないため
    // それぞれの動作はトレイトの宣言や#derive属性を書く実装者に委ねられている
    // 言語コアのi32はEqを実装するがf32はPartialEqしか実装していない

    // PartialOrdトレイトを実装した型では「<」などで順序比較が可能
    assert!(Interval { lower: 10, upper: 20 } < Interval { lower: 20, upper: 40});
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1});
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8});
    // Interval同士が重なっている場合、どちらかが大きいという事にはならない実装
    let left  = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));
}
