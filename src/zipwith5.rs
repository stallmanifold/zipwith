use std::ops::Fn;
use std::marker::PhantomData;
use std::iter::IntoIterator;


#[derive(Clone)]
pub struct ZipWith5<F, S1, S2, S3, S4, S5, T> {
    s1: S1,
    s2: S2,
    s3: S3,
    s4: S4,
    s5: S5,
    f: F,
    t: PhantomData<T>,
}

impl<F, S1, S2, S3, S4, S5, T> ZipWith5<F, S1, S2, S3, S4, S5, T>
    where S1: IntoIterator,
          S2: IntoIterator,
          S3: IntoIterator,
          S4: IntoIterator,
          S5: IntoIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, S5::Item) -> T
{
    pub fn zip_with(f: F, s1: S1, s2: S2, s3: S3, s4: S4, s5: S5) 
        -> ZipWith5<F, S1::IntoIter, S2::IntoIter, S3::IntoIter, S4::IntoIter, S5::IntoIter, T> {
        
        ZipWith5 { 
            s1: s1.into_iter(), 
            s2: s2.into_iter(), 
            s3: s3.into_iter(),
            s4: s4.into_iter(),
            s5: s5.into_iter(),
            f: f, 
            t: PhantomData,
        }
    }
}

impl<F, S1, S2, S3, S4, S5, T> Iterator for ZipWith5<F, S1, S2, S3, S4, S5, T>
    where S1: Iterator,
          S2: Iterator,
          S3: Iterator,
          S4: Iterator,
          S5: Iterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, S5::Item) -> T
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.s1.next().and_then(|v1| {
            self.s2.next().and_then(|v2| {
                self.s3.next().and_then(|v3| {
                    self.s4.next().and_then(|v4| {
                        self.s5.next().and_then(|v5| { 
                            Some((self.f)(v1,v2,v3,v4,v5))
                        })
                    })
                })
            })
        })
    }
}

impl<F, S1, S2, S3, S4, S5, T> ExactSizeIterator for ZipWith5<F, S1, S2, S3, S4, S5, T>
    where S1: ExactSizeIterator,
          S2: ExactSizeIterator,
          S3: ExactSizeIterator,
          S4: ExactSizeIterator,
          S5: ExactSizeIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, S5::Item) -> T
{
    fn len(&self) -> usize {
        let lens = vec![self.s1.len(), self.s2.len(), self.s3.len(), self.s4.len(), self.s5.len()];

        lens.into_iter().min().unwrap()
    }
}

impl<F, S1, S2, S3, S4, S5, T> DoubleEndedIterator for ZipWith5<F, S1, S2, S3, S4, S5, T>
    where S1: DoubleEndedIterator,
          S2: DoubleEndedIterator,
          S3: DoubleEndedIterator,
          S4: DoubleEndedIterator,
          S5: DoubleEndedIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, S5::Item) -> T
{
    fn next_back(&mut self) -> Option<T> {
        self.s1.next_back().and_then(|v1| {
            self.s2.next_back().and_then(|v2| {
                self.s3.next_back().and_then(|v3| {
                    self.s4.next_back().and_then(|v4| { 
                        self.s5.next_back().and_then(|v5| { 
                            Some((self.f)(v1,v2,v3,v4,v5))
                        })
                    })
                })
            })
        })
    }
}
