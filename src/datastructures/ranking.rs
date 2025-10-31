#[derive(Debug, Clone, Default)]
pub struct Ranking<T>(pub Vec<T>);

impl<T> Ranking<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn get_ranks<F, K>(mut self, mut sort_by: F) -> Vec<(usize, T)>
    where
        F: FnMut(&T) -> K + Clone,
        K: Ord + Eq,
    {
        self.0.sort_unstable_by_key(sort_by.clone());

        let mut i = 0;
        let mut out = Vec::new();
        let mut last_k_opt = None;
        for element in self.0 {
            let Some(last_k) = &last_k_opt else {
                last_k_opt = Some((sort_by)(&element));
                out.push((i, element));
                continue;
            };

            let new_k = (sort_by)(&element);

            if &new_k != last_k {
                i += 1;
                last_k_opt = Some(new_k);
            }

            out.push((i, element));
        }

        out
    }
}

impl<T> From<Vec<T>> for Ranking<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use crate::datastructures::ranking::Ranking;

    #[test]
    fn ranking_test() {
        let mut ranks = Ranking::new();
        ranks.push(1);
        ranks.push(2);
        ranks.push(2);
        ranks.push(3);
        ranks.push(4);
        ranks.push(4);

        let rankings = ranks.get_ranks(|element| Reverse(*element));
        let mut iter = rankings.into_iter();

        assert_eq!(iter.next(), Some((0, 4)));
        assert_eq!(iter.next(), Some((0, 4)));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), None);
    }
}
