use std::time::Duration;

pub struct Output;

impl Output 
{
    pub fn display_table(city_names: &[String], adj_matrix: &[Vec<i32>]) 
    {
        let n = city_names.len();
        let max_city_len = city_names.iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0)
            .max(7);
        
        println!("\n[#] Adjacency Matrix:");
        println!("[-] Note: Value at [i][j] represents distance from city i to city j\n");
        
        print!("+");
        for _ in 0..=n 
        {
            print!("{:-<width$}+", "", width = max_city_len + 2);
        }
        println!();
        
        print!("| {:width$} |", "From/To", width = max_city_len);
        for city in city_names 
        {
            print!(" {:^width$} |", city, width = max_city_len);
        }
        println!();
        
        print!("+");
        for _ in 0..=n 
        {
            print!("{:-<width$}+", "", width = max_city_len + 2);
        }
        println!();
        
        for (i, city) in city_names.iter().enumerate() 
        {
            print!("| {:width$} |", city, width = max_city_len);
            for j in 0..n 
            {
                let value = adj_matrix[i][j];
                if value == 0 && i != j 
                {
                    print!(" {:^width$} |", "∞", width = max_city_len);
                } 
                else 
                {
                    print!(" {:^width$} |", value, width = max_city_len);
                }
            }
            println!();
            
            print!("+");
            for _ in 0..=n 
            {
                print!("{:-<width$}+", "", width = max_city_len + 2);
            }
            println!();
        }
    }
    
    pub fn display_results(city_names: &[String],
                           _starting_node: usize,
                           min_cost: i32,
                           tours: &[Vec<usize>],
                           duration: Duration,
                           n: usize,
                           adj_matrix: &[Vec<i32>],
    ) 
    {
        if min_cost == i32::MAX 
        {
            println!("[!] No valid tour found!");
            println!("[-] This might happen if:");
            println!("[-] 1. The graph is not strongly connected");
            println!("[-] 2. There's no path that visits all cities and returns to start");
            return;
        }
        
        println!("[#] Results:");
        println!("[~] Minimum cost: {}", min_cost);
        println!("[~] Shortest tour(s) with minimum cost found: {}", tours.len());
        
        if !tours.is_empty() 
        {
            println!("\n[~] Example of shortest tour:");
            print!("    ");
            
            let first_tour = &tours[0];
            for (i, &city) in first_tour.iter().enumerate() 
            {
                if i > 0 
                {
                    print!(" → ");
                }
                print!("{}", city_names[city]);
            }
            println!();

            println!("\n[~] Step-by-step costs:");
            let mut total: i32 = 0;
            for i in 0..first_tour.len() - 1 
            {
                let from:usize = first_tour[i];
                let to: usize  = first_tour[i+1];
                let cost: i32  = adj_matrix[from][to];
                total += cost;
                println!("    {} → {} (cost: {}) (sum: {})", city_names[from], city_names[to], cost, total);
            }
        }
        
        println!();
        
        // Display execution time
        let exec_time = if duration.as_micros() < 1000 
        {
            format!("{:.2}µs", duration.as_micros() as f64)
        } 
        else if duration.as_millis() < 1000 
        {
            format!("{:.2}ms", duration.as_millis() as f64)
        } 
        else 
        {
            format!("{:.2}s", duration.as_secs_f64())
        };
        println!("[~] Execution time = {}", exec_time);
        
        // Display complexity
        println!("[~] T(n) = T(n² × 2ⁿ)");
        println!("[~] O(n) = O(n² × 2ⁿ) where n = {}", n);
    }
}