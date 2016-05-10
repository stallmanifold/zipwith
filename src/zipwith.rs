use std::ops::Fn;
use std::marker::PhantomData;
use std::cmp;
use std::iter::IntoIterator;


#[derive(Clone)]
pub struct ZipWith<F,A,B,C> {
    a: A,
    b: B,
    f: F,
    c: PhantomData<C>,
}

impl<F,A,B,C> ZipWith<F,A,B,C>
    where A: IntoIterator,
          B: IntoIterator,
          F: Fn(A::Item, B::Item) -> C
{
    pub fn zip_with(f: F, this: A, that: B) -> ZipWith<F, A::IntoIter, B::IntoIter, C> {
        ZipWith { 
            a: this.into_iter(), 
            b: that.into_iter(), 
            f: f, 
            c: PhantomData,
        }
    }
}

impl<F, FromA, FromB, C> Iterator for ZipWith<F, FromA, FromB, C>
    where FromA: Iterator,
          FromB: Iterator,
          F:     Fn(FromA::Item, FromB::Item) -> C
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        self.a.next().and_then(|x| {
            self.b.next().and_then(|y| {
                Some((self.f)(x,y))
            })
        })
    }
}

impl<F, FromA, FromB, C> ExactSizeIterator for ZipWith<F, FromA, FromB, C>
    where FromA: ExactSizeIterator,
          FromB: ExactSizeIterator,
          F:     Fn(FromA::Item, FromB::Item) -> C
{
    fn len(&self) -> usize {
        cmp::min(self.a.len(), self.b.len())
    }
}

impl<F, FromA, FromB, C> DoubleEndedIterator for ZipWith<F, FromA, FromB, C>
    where FromA: DoubleEndedIterator,
          FromB: DoubleEndedIterator,
          F:     Fn(FromA::Item, FromB::Item) -> C
{
    fn next_back(&mut self) -> Option<C> {
        self.a.next_back().and_then(|x| {
            self.b.next_back().and_then(|y| {
                Some((self.f)(x,y))
            })
        })
    }
}
