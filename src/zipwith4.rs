use std::ops::Fn;
use std::marker::PhantomData;
use std::iter::IntoIterator;


#[derive(Clone)]
pub struct ZipWith4<F, S1, S2, S3, S4, T> {
    s1: S1,
    s2: S2,
    s3: S3,
    s4: S4,
    f: F,
    t: PhantomData<T>,
}

impl<F, S1, S2, S3, S4, T> ZipWith4<F, S1, S2, S3, S4, T>
    where S1: IntoIterator,
          S2: IntoIterator,
          S3: IntoIterator,
          S4: IntoIterator,
          F: Fn(S1::Item, S2::Item, S3::Item, S4::Item) -> T
{
    pub fn zip_with(f: F, s1: S1, s2: S2, s3: S3, s4: S4) 
        -> ZipWith4<F, S1::IntoIter, S2::IntoIter, S3::IntoIter, S4::IntoIter, T> {
        
        ZipWith4 { 
            s1: s1.into_iter(), 
            s2: s2.into_iter(), 
            s3: s3.into_iter(),
            s4: s4.into_iter(),
            f: f, 
            t: PhantomData,
        }
    }
}

impl<F, S1, S2, S3, S4, T> Iterator for ZipWith4<F, S1, S2, S3, S4, T>
    where S1: Iterator,
          S2: Iterator,
          S3: Iterator,
          S4: Iterator,
          F: Fn(S1::Item, S2::Item, S3::Item, S4::Item) -> T
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.s1.next().and_then(|v1| {
            self.s2.next().and_then(|v2| {
                self.s3.next().and_then(|v3| {
                    self.s4.next().and_then(|v4| {
                        Some((self.f)(v1,v2,v3,v4))
                    })
                })
            })
        })
    }
}

impl<F, S1, S2, S3, S4, T> ExactSizeIterator for ZipWith4<F, S1, S2, S3, S4, T>
    where S1: ExactSizeIterator,
          S2: ExactSizeIterator,
          S3: ExactSizeIterator,
          S4: ExactSizeIterator,
          F: Fn(S1::Item, S2::Item, S3::Item, S4::Item) -> T
{
    fn len(&self) -> usize {
        let lens = vec![self.s1.len(), self.s2.len(), self.s3.len(), self.s4.len()];

        lens.into_iter().min().unwrap()
    }
}

impl<F, S1, S2, S3, S4, T> DoubleEndedIterator for ZipWith4<F, S1, S2, S3, S4, T>
    where S1: DoubleEndedIterator,
          S2: DoubleEndedIterator,
          S3: DoubleEndedIterator,
          S4: DoubleEndedIterator,
          F: Fn(S1::Item, S2::Item, S3::Item, S4::Item) -> T
{
    fn next_back(&mut self) -> Option<T> {
        self.s1.next_back().and_then(|v1| {
            self.s2.next_back().and_then(|v2| {
                self.s3.next_back().and_then(|v3| {
                    self.s4.next_back().and_then(|v4| { 
                        Some((self.f)(v1,v2,v3,v4))
                    })
                })
            })
        })
    }
}
