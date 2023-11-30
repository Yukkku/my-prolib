trait AddEdgeSimple<const DIRECTED: bool> {
    fn add_edge(&mut self, from: usize, to: usize);
}
trait AddEdgeWeight<T, const DIRECTED: bool> {
    fn add_edge(&mut self, from: usize, to: usize, weight: T);
}

pub mod dijkstra;
pub use dijkstra::Dijkstra;
