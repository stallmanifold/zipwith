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
          C: IntoIterator,
          F: Fn(A::Item, B::Item) -> C::Item
{
    fn zip_with(f: F, this: A, that: B) -> ZipWith<F,A::IntoIter,B::IntoIter,C::IntoIter> {
        ZipWith { 
            a: this.into_iter(), 
            b: that.into_iter(), 
            f: f, 
            c: PhantomData,
        }
    }
}

impl<F,FromA,FromB,ToC> Iterator for ZipWith<F,FromA,FromB,ToC>
    where FromA: Iterator,
          FromB: Iterator,
          ToC:   Iterator,
          F:     Fn(FromA::Item, FromB::Item) -> ToC::Item 
{
    type Item = ToC::Item;

    fn next(&mut self) -> Option<ToC::Item> {
        self.a.next().and_then(|x| {
            self.b.next().and_then(|y| {
                Some((self.f)(x,y))
            })
        })
    }
}

impl<F,FromA,FromB,ToC> ExactSizeIterator for ZipWith<F,FromA,FromB,ToC>
    where FromA: ExactSizeIterator,
          FromB: ExactSizeIterator,
          ToC:   ExactSizeIterator,
          F:     Fn(FromA::Item, FromB::Item) -> ToC::Item
{
    fn len(&self) -> usize {
        cmp::min(self.a.len(), self.b.len())
    }
}

impl<F,FromA,FromB,ToC> DoubleEndedIterator for ZipWith<F,FromA,FromB,ToC>
    where FromA: DoubleEndedIterator,
          FromB: DoubleEndedIterator,
          ToC:   DoubleEndedIterator,
          F:     Fn(FromA::Item, FromB::Item) -> ToC::Item
{
    fn next_back(&mut self) -> Option<ToC::Item> {
        self.a.next_back().and_then(|x| {
            self.b.next_back().and_then(|y| {
                Some((self.f)(x,y))
            })
        })
    }
}