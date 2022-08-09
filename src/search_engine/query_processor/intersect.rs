use super::query_result::QueryResult;

pub trait Intersect {
    /// Returns a vector that is the intersection of this and the other query results vector.
    fn intersect(&mut self, other: &Vec<QueryResult>);
}

impl Intersect for Vec<QueryResult> {
    fn intersect(&mut self, other: &Vec<QueryResult>) {
        self.retain_mut(|a: &mut QueryResult| {
            other.iter().any(|b: &QueryResult| {
                if a.doc_id == b.doc_id {
                    a.rank += b.rank; // increment rank
                    true
                } else {
                    false
                }
            })
        });
    }
}
