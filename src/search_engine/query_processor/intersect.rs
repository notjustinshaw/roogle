use super::query_result::QueryResult;

pub trait Intersect {
    /// Returns a vector that is the intersection of
    /// this and the other query results vector.
    fn intersect(&mut self, other: &Vec<QueryResult>);
}

impl Intersect for Vec<QueryResult> {
    fn intersect(&mut self, other: &Vec<QueryResult>) {
        self.retain(|a: &QueryResult| other.iter().any(|b: &QueryResult| a.doc_id == b.doc_id));
    }
}
