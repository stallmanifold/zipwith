use std::ops::Fn;
use std::marker::PhantomData;
use std::iter::IntoIterator;


#[derive(Clone)]
pub struct ZipWith8<F, S1, S2, S3, S4, S5, S6, S7, S8, T> {
    s1: S1,
    s2: S2,
    s3: S3,
    s4: S4,
    s5: S5,
    s6: S6,
    s7: S7,
    s8: S8,
    f: F,
    t: PhantomData<T>,
}

impl<F, S1, S2, S3, S4, S5, S6, S7, S8, T> ZipWith8<F, S1, S2, S3, S4, S5, S6, S7, S8, T>
    where S1: IntoIterator,
          S2: IntoIterator,
          S3: IntoIterator,
          S4: IntoIterator,
          S5: IntoIterator,
          S6: IntoIterator,
          S7: IntoIterator,
          S8: IntoIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, 
                 S5::Item, S6::Item, S7::Item, S8::Item) -> T
{
    pub fn zip_with(f: F, s1: S1, s2: S2, s3: S3, s4: S4, s5: S5, s6: S6, s7: S7, s8: S8) 
        -> ZipWith8<F, S1::IntoIter, S2::IntoIter, S3::IntoIter, S4::IntoIter, 
                       S5::IntoIter, S6::IntoIter, S7::IntoIter, S8::IntoIter, T> {
        
        ZipWith8 { 
            s1: s1.into_iter(), 
            s2: s2.into_iter(), 
            s3: s3.into_iter(),
            s4: s4.into_iter(),
            s5: s5.into_iter(),
            s6: s6.into_iter(),
            s7: s7.into_iter(),
            s8: s8.into_iter(),
            f: f, 
            t: PhantomData,
        }
    }
}

impl<F, S1, S2, S3, S4, S5, S6, S7, S8, T> Iterator for ZipWith8<F, S1, S2, S3, S4, S5, S6, S7, S8, T>
    where S1: Iterator,
          S2: Iterator,
          S3: Iterator,
          S4: Iterator,
          S5: Iterator,
          S6: Iterator,
          S7: Iterator,
          S8: Iterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, 
                 S5::Item, S6::Item, S7::Item, S8::Item) -> T
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.s1.next().and_then(|v1| {
            self.s2.next().and_then(|v2| {
                self.s3.next().and_then(|v3| {
                    self.s4.next().and_then(|v4| {
                        self.s5.next().and_then(|v5| { 
                            self.s6.next().and_then(|v6| {
                                self.s7.next().and_then(|v7| {
                                    self.s8.next().and_then(|v8| {
                                        Some((self.f)(v1,v2,v3,v4,v5,v6,v7,v8))
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    }
}

impl<F, S1, S2, S3, S4, S5, S6, S7, S8, T> ExactSizeIterator 
    for ZipWith8<F, S1, S2, S3, S4, S5, S6, S7, S8, T>
    
    where S1: ExactSizeIterator,
          S2: ExactSizeIterator,
          S3: ExactSizeIterator,
          S4: ExactSizeIterator,
          S5: ExactSizeIterator,
          S6: ExactSizeIterator,
          S7: ExactSizeIterator,
          S8: ExactSizeIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, 
                 S5::Item, S6::Item, S7::Item, S8::Item) -> T
{
    fn len(&self) -> usize {
        let lens = vec![self.s1.len(), self.s2.len(), self.s3.len(), 
                        self.s4.len(), self.s5.len(), self.s6.len(), 
                        self.s7.len(), self.s8.len()];

        lens.into_iter().min().unwrap()
    }
}

impl<F, S1, S2, S3, S4, S5, S6, S7, S8, T> DoubleEndedIterator 
    for ZipWith8<F, S1, S2, S3, S4, S5, S6, S7, S8, T>
    
    where S1: DoubleEndedIterator,
          S2: DoubleEndedIterator,
          S3: DoubleEndedIterator,
          S4: DoubleEndedIterator,
          S5: DoubleEndedIterator,
          S6: DoubleEndedIterator,
          S7: DoubleEndedIterator,
          S8: DoubleEndedIterator,
          F:  Fn(S1::Item, S2::Item, S3::Item, S4::Item, S5::Item, S6::Item, S7::Item, S8::Item) -> T
{
    fn next_back(&mut self) -> Option<T> {
        self.s1.next_back().and_then(|v1| {
            self.s2.next_back().and_then(|v2| {
                self.s3.next_back().and_then(|v3| {
                    self.s4.next_back().and_then(|v4| { 
                        self.s5.next_back().and_then(|v5| { 
                            self.s6.next_back().and_then(|v6| {
                                self.s7.next_back().and_then(|v7| {
                                    self.s8.next_back().and_then(|v8| {
                                        Some((self.f)(v1,v2,v3,v4,v5,v6,v7,v8))
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    }
}
