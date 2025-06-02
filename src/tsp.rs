use crate::graph::Graph;

pub struct TSPSolver<'a> 
{
    graph: &'a Graph,
    n: usize,
    dp: Vec<Vec<i32>>,
    parent: Vec<Vec<usize>>,
}

impl<'a> TSPSolver<'a> 
{
    pub fn new(graph: &'a Graph) -> Self 
    {
        let n: usize = graph.get_n();
        let size: usize = 1 << n;
        
        TSPSolver 
        {
            graph,
            n,
            dp: vec![vec![i32::MAX; n]; size],
            parent: vec![vec![usize::MAX; n]; size],
        }
    }
    
    pub fn solve(&mut self, start: usize) -> (i32, Vec<Vec<usize>>) 
    {
        // Initialize base case
        self.dp[1 << start][start] = 0;
        
        // Fill DP table
        for mask in 0..(1 << self.n) 
        {
            for u in 0..self.n 
            {
                if (mask & (1 << u)) == 0 || self.dp[mask][u] == i32::MAX 
                {
                    continue;
                }
                
                for v in 0..self.n 
                {
                    if (mask & (1 << v)) != 0 || !self.graph.has_edge(u, v) 
                    {
                        continue;
                    }
                    
                    let new_mask = mask | (1 << v);
                    let new_cost = self.dp[mask][u] + self.graph.get_distance(u, v);
                    
                    if new_cost < self.dp[new_mask][v] 
                    {
                        self.dp[new_mask][v] = new_cost;
                        self.parent[new_mask][v] = u;
                    }
                }
            }
        }
        
        // Find minimum cost and reconstruct tours
        let full_mask: usize = (1 << self.n) - 1;
        let mut min_cost: i32 = i32::MAX;
        let mut end_cities: Vec<usize> = Vec::new();
        
        for u in 0..self.n 
        {
            if u != start && self.graph.has_edge(u, start) && self.dp[full_mask][u] != i32::MAX 
            {
                let total_cost = self.dp[full_mask][u] + self.graph.get_distance(u, start);
                if total_cost < min_cost 
                {
                    min_cost = total_cost;
                    end_cities.clear();
                    end_cities.push(u);
                } 
                else if total_cost == min_cost 
                {
                    end_cities.push(u);
                }
            }
        }
        
        // Reconstruct all optimal tours
        let mut tours: Vec<Vec<usize>> = Vec::new();
        for &end_city in &end_cities 
        {
            let tour: Vec<usize> = self.reconstruct_tour(start, end_city, full_mask);
            tours.push(tour);
        }
        
        (min_cost, tours)
    }
    
    fn reconstruct_tour(&self, start: usize, end: usize, full_mask: usize) -> Vec<usize>
    {
        let mut tour: Vec<usize> = Vec::new();
        let mut current: usize   = end;
        let mut mask: usize      = full_mask;
        
        tour.push(current);
        
        while self.parent[mask][current] != usize::MAX
         {
            let prev = self.parent[mask][current];
            tour.push(prev);
            mask ^= 1 << current;
            current = prev;
        }
        
        tour.reverse();
        tour.push(start);
        tour
    }
}