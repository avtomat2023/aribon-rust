// 長さnの数列 a_1, a_2, ..., a_n が与えられる。
// この数列の増加部分列のうち、最長のものの長さを求めよ。
//
// 入力
// n
// a_1, a_2, ..., a_n
//
// 出力
// 最長増加部分列の長さ

use atcoder_snippets::*;
use atcoder_snippets::read::*;
use atcoder_snippets::bsearch::*;

use std::cmp::{Ord, Ordering};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Reverse<T: Ord>(pub T);
impl<T: Ord> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Reverse<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Reverse<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
pub trait SortDesc<T> {
    fn sort_desc(&mut self)
    where
        T: Ord;
    fn sort_desc_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering;
    fn sort_desc_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, key: F);
    fn sort_unstable_desc(&mut self)
    where
        T: Ord;
    fn sort_unstable_desc_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering;
    fn sort_unstable_desc_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, key: F);
}
impl<T> SortDesc<T> for [T] {
    fn sort_desc(&mut self)
    where
        T: Ord,
    {
        self.sort_by(|x, y| y.cmp(x));
    }
    fn sort_desc_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.sort_by(|x, y| cmp(y, x));
    }
    fn sort_desc_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, mut key: F) {
        self.sort_by_key(|x| Reverse(key(x)));
    }
    fn sort_unstable_desc(&mut self)
    where
        T: Ord,
    {
        self.sort_unstable_by(|x, y| y.cmp(x));
    }
    fn sort_unstable_desc_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.sort_unstable_by(|x, y| cmp(y, x));
    }
    fn sort_unstable_desc_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, mut key: F) {
        self.sort_unstable_by_key(|x| Reverse(key(x)));
    }
}
#[derive(PartialEq, PartialOrd)]
pub struct Total<T: PartialOrd + PartialEq>(pub T);
impl<T: PartialOrd + PartialEq> Eq for Total<T> {}
impl<T: PartialOrd + PartialEq> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
pub trait WithCmpIdentity<T>: Sized {
    fn new(x: T) -> Self;
    fn inf() -> Self;
    fn as_option(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
    fn into_option(self) -> Option<T>;
    fn is_fin(&self) -> bool {
        self.as_option().is_some()
    }
    fn is_inf(&self) -> bool {
        self.as_option().is_none()
    }
    fn expect_fin(self, msg: &str) -> T {
        self.into_option().expect(msg)
    }
    fn fin(self) -> T {
        self.into_option().unwrap()
    }
    fn fin_or(self, default: T) -> T {
        self.into_option().unwrap_or(default)
    }
    fn fin_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        self.into_option().unwrap_or_else(f)
    }
    fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        self.into_option().map_or(default, f)
    }
    fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        self.into_option().map_or_else(default, f).into()
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MaybeNegInf<T> {
    Inf,
    Fin(T),
}
pub type Max<T> = MaybeNegInf<T>;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MaybeInf<T> {
    Fin(T),
    Inf,
}
pub type Min<T> = MaybeInf<T>;
macro_rules! impl_with_cmp_identity {
    ( $ t : ident ) => {
        impl<T> $t<T> {
            pub fn map<U: Ord, F: FnOnce(T) -> U>(self, f: F) -> $t<U> {
                match self {
                    $t::Fin(x) => $t::Fin(f(x)),
                    $t::Inf => $t::Inf,
                }
            }
        }
        impl<T> WithCmpIdentity<T> for $t<T> {
            fn new(x: T) -> $t<T> {
                $t::Fin(x)
            }
            fn inf() -> $t<T> {
                $t::Inf
            }
            fn as_option(&self) -> Option<&T> {
                match self {
                    $t::Fin(x) => Some(x),
                    $t::Inf => None,
                }
            }
            fn as_option_mut(&mut self) -> Option<&mut T> {
                match self {
                    $t::Fin(x) => Some(x),
                    $t::Inf => None,
                }
            }
            fn into_option(self) -> Option<T> {
                match self {
                    $t::Fin(x) => Some(x),
                    $t::Inf => None,
                }
            }
        }
    };
}
impl_with_cmp_identity!(MaybeNegInf);
impl_with_cmp_identity!(MaybeInf);

fn main() {
    readls!(n = usize, aa = Vec<u64>);
    let mut dp = vec![MaybeInf::inf(); n];
    for a in aa {
        let i = (0..dp.len()).bsearch_right_min(|&i| {
            MaybeInf::new(a) <= dp[i]
        }).unwrap().unwrap();
        dp[i] = MaybeInf::new(a);
    }
    let ans = 1 + (0..dp.len()).bsearch_left_max(|&i| dp[i].is_fin()).unwrap().unwrap();
    println!("{}", ans);
}
