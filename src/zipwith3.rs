use std::ops::Fn;
use std::marker::PhantomData;
use std::cmp;
use std::iter::IntoIterator;


#[derive(Clone)]
pub struct ZipWith3<F, A, B, C, D> {
    a: A,
    b: B,
    c: C,
    f: F,
    d: PhantomData<D>,
}

impl<F, A, B, C, D> ZipWith3<F, A, B, C, D>
    where A: IntoIterator,
          B: IntoIterator,
          C: IntoIterator,
          F: Fn(A::Item, B::Item, C::Item) -> D
{
    pub fn zip_with(f: F, a: A, b: B, c: C) -> ZipWith3<F, A::IntoIter, B::IntoIter, C::IntoIter, D> {
        ZipWith3 { 
            a: a.into_iter(), 
            b: b.into_iter(), 
            c: c.into_iter(),
            f: f, 
            d: PhantomData,
        }
    }
}

impl<F, A, B, C, D> Iterator for ZipWith3<F, A, B, C, D>
    where A: Iterator,
          B: Iterator,
          C: Iterator,
          F: Fn(A::Item, B::Item, C::Item) -> D
{
    type Item = D;

    fn next(&mut self) -> Option<D> {
        self.a.next().and_then(|x| {
            self.b.next().and_then(|y| {
                self.c.next().and_then(|z| {
                    Some((self.f)(x,y,z))
                })
            })
        })
    }
}

impl<F, A, B, C, D> ExactSizeIterator for ZipWith3<F, A, B, C, D>
    where A: ExactSizeIterator,
          B: ExactSizeIterator,
          C: ExactSizeIterator,
          F: Fn(A::Item, B::Item, C::Item) -> D
{
    fn len(&self) -> usize {
        cmp::min(self.a.len(), cmp::min(self.b.len(), self.c.len()))
    }
}

impl<F, A, B, C, D> DoubleEndedIterator for ZipWith3<F, A, B, C, D>
    where A: DoubleEndedIterator,
          B: DoubleEndedIterator,
          C: DoubleEndedIterator,
          F: Fn(A::Item, B::Item, C::Item) -> D
{
    fn next_back(&mut self) -> Option<D> {
        self.a.next_back().and_then(|x| {
            self.b.next_back().and_then(|y| {
                self.c.next_back().and_then(|z| {
                    Some((self.f)(x,y,z))
                })
            })
        })
    }
}
