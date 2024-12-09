pub(crate) struct SafeChunksIter<'s, const B: usize> {
    remaining: &'s str
}
impl<'s, const B: usize> SafeChunksIter<'s, B> {
    pub(crate) const fn new(remaining: &'s str) -> Self {
        Self { remaining }
    }
}
impl<'s, const B: usize> Iterator for SafeChunksIter<'s, B> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let split_point;

        match self.remaining.find("%") {
            Some(0) => {
                let closing_marker = self.remaining[1..]
                    .find("%")
                    .map(|i| i + 2)
                    .unwrap_or(self.remaining.len());
                split_point = closing_marker;
                debug_assert!(split_point < B);
            }
            Some(next_marker) => {
                split_point = next_marker.min(B);
            }
            None => {
                split_point = self.remaining.len().min(B);
            }
        }
        let (section, the_rest) = self.remaining.split_at(split_point);
        self.remaining = the_rest;
        Some(section)
    }
}
