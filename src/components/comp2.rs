use super::*;

#[derive(Debug, Default, Clone)]
pub struct Comp2<ID, T1, T2>(pub Vec<T1>, pub Vec<T2>, PhantomData<ID>);

impl<ID, T1, T2> Insert<Id<ID>, (T1, T2)> for Comp2<ID, T1, T2> {
    fn insert(&mut self, id: Id<ID>, value: (T1, T2)) {
        self.insert_at_index(id.index, value.0, value.1);
    }
}

impl<ID, T1, T2> Insert<&Id<ID>, (T1, T2)> for Comp2<ID, T1, T2> {
    fn insert(&mut self, id: &Id<ID>, value: (T1, T2)) {
        self.insert_at_index(id.index, value.0, value.1);
    }
}

impl<ID, T1, T2> Comp2<ID, T1, T2> {
    fn insert_at_index(&mut self, index: usize, a: T1, b: T2) {
        debug_assert!(self.len() >= index);
        match self.len() {
            len if len > index => {
                if let (Some(va), Some(vb)) = (self.0.get_mut(index), self.1.get_mut(index)) {
                    *va = a;
                    *vb = b;
                }
            }
            len if len == index => {
                self.0.push(a);
                self.1.push(b);
            }
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

}