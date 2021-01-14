#![feature(trait_alias)]
/// # ref:
/// * http://www-users.math.umn.edu/~brubaker/docs/152/152groups.pdf
/// * https://livedu.in/algebraic-structure/
///

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Eq;
use std::ops::Neg;

/// Groupoid
/// A groupoid is an algebraic structure consisting of a non-empty set G and a binary operation o on G. The pair (G, o) is called groupoid.
/// The set of real numbers with the binary operation of addition is a groupoid.

trait Groupoid = Eq + Sized;

/// Groupoid
/// If (G, o) is a groupoid and if the associative rule (aob)oc = ao(boc) holds for all a, b, c ∈ G, then (G, o) is called a semigroup.
/// An element e of a groupoid (G, o) is called an identity element if eoa = aoe = a for all a ∈ G. If there is an identity element in a groupoid then it is unique.

trait Semigroup: Groupoid + Add {
    fn op(self, rhs: &Self) -> Self;
}

/// Semigroup
/// If (G, o) is a groupoid and if the associative rule (aob)oc = ao(boc) holds for all a, b, c ∈ G, then (G, o) is called a semigroup.
/// An element e of a groupoid (G, o) is called an identity element if eoa = aoe = a for all a ∈ G. If there is an identity element in a groupoid then it is unique.
// A semigroup with identity element is called a monoid.

trait Monoid: Semigroup {
    fn id(self) -> Self;
}


/// Group
/// Let (G, o) be a monoid. An element a’ ∈ G is called an inverse of the element a ∈ G if aoa’ = a’oa = e (the identity element of G). The inverse of the element a ∈ G is denoted by a^{-1}.
/// A monoid in which every element has an inverse is called group.
trait Inverse: Neg {
    fn inv(self) -> Self;
}

trait Group = Monoid + Inverse + Sub;

/// RING is a setRwhich is CLOSED under two operations+and×andsatisfying the following properties:
/// (1) R is an abelian group under+.
/// (2)Associativity of × For every a,b,c∈R,a×(b×c) = (a×b)×c
/// (3)Distributive Properties – For everya,b,c∈Rthe following identities hold:
/// a×(b+c) = (a×b) + (a×c)and(b+c)×a=b×a+c×a
trait Ring: Group + Mul {
    fn sec_op(self, rhs: &Self) -> Self;
}

/// A FIELD is a set F
/// which is closed under two operations + and × s.t.
/// (1) Fis an abelian group under + and
/// (2) F-{0} (the set F without the additive identity 0) is an abelian group under ×.

trait Field: Ring + Div {
    fn sec_id(self) -> Self;
    fn sec_inv(self) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
