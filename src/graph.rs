pub struct Graph 
{
    n: usize,
    #[allow(dead_code)]
    city_names: Vec<String>,
    adj_matrix: Vec<Vec<i32>>,
}

impl Graph 
{
    pub fn new(n: usize, city_names: Vec<String>, adj_matrix: Vec<Vec<i32>>) -> Self 
    {
        Graph 
        {
            n,
            city_names,
            adj_matrix,
        }
    }
    
    pub fn get_graph_info(&self) -> (usize, usize) 
    {
        let num_roads: usize     = self.count_roads();
        let starting_node: usize = 0;
        (num_roads, starting_node)
    }
    
    fn count_roads(&self) -> usize 
    {
        let mut count: usize = 0;
        for i in 0..self.n 
        {
            for j in 0..self.n 
            {
                if i != j && self.adj_matrix[i][j] > 0 && self.adj_matrix[i][j] != i32::MAX
                {
                    count += 1;
                }
            }
        }
        count
    }
    
    pub fn get_n(&self) -> usize 
    {
        self.n
    }
    
    pub fn get_distance(&self, i: usize, j: usize) -> i32 
    {
        self.adj_matrix[i][j]
    }
    
    #[allow(dead_code)]
    pub fn get_city_name(&self, index: usize) -> &str 
    {
        &self.city_names[index]
    }
    
    pub fn has_edge(&self, i: usize, j: usize) -> bool 
    {
        self.adj_matrix[i][j] > 0
    }
}