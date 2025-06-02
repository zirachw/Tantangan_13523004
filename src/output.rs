use std::time::Duration;
use crate::style::Style;

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
        
        println!("\n{}", Style::header("[#] Adjacency Matrix:"));
        println!("{}\n", Style::italic("[-] Note: Value at [i][j] represents distance from city i to city j"));
        
        print!("+");
        for _ in 0..=n 
        {
            print!("{:-<width$}+", "", width = max_city_len + 2);
        }
        println!();
        
        print!("| {} |", Style::bold(&format!("{:width$}", "From/To", width = max_city_len)));
        for city in city_names 
        {
            print!(" {} |", Style::city_name(&format!("{:^width$}", city, width = max_city_len)));
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
            print!("| {} |", Style::city_name(&format!("{:width$}", city, width = max_city_len)));
            for j in 0..n 
            {
                let value = adj_matrix[i][j];
                if value == 0 && i != j 
                {
                    print!(" {} |", Style::warning(&format!("{:^width$}", "∞", width = max_city_len)));
                } 
                else 
                {
                    print!(" {} |", Style::number(&format!("{:^width$}", value, width = max_city_len)));
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
            println!("{}", Style::error("[!] No valid tour found!"));
            println!("{}", Style::warning("[-] This might happen if:"));
            println!("{}", Style::warning("[-] 1. The graph is not strongly connected"));
            println!("{}", Style::warning("[-] 2. There's no path that visits all cities and returns to start"));
            return;
        }
        
        println!("{}", Style::header("[#] Results:"));
        println!("{} {}", Style::success("[~] Minimum cost:"), Style::bold(&min_cost.to_string()));
        println!("{} {}", Style::success("[~] Shortest tour(s) with minimum cost found:"), Style::bold(&tours.len().to_string()));
        
        if !tours.is_empty() 
        {
            println!("\n{}", Style::info("[~] Example of shortest tour:"));
            print!("    ");
            
            let first_tour = &tours[0];
            for (i, &city) in first_tour.iter().enumerate() 
            {
                if i > 0 
                {
                    print!(" {} ", Style::arrow());
                }
                print!("{}", Style::city_name(&city_names[city]));
            }
            println!();

            println!("\n{}", Style::info("[~] Step-by-step costs:"));
            let mut total: i32 = 0;
            for i in 0..first_tour.len() - 1 
            {
                let from:usize = first_tour[i];
                let to: usize  = first_tour[i+1];
                let cost: i32  = adj_matrix[from][to];
                total += cost;
                println!("    {} {} {} {} {} {}", 
                    Style::city_name(&city_names[from]), 
                    Style::arrow(), 
                    Style::city_name(&city_names[to]), 
                    Style::italic(&format!("(cost: {})", cost)),
                    Style::bold("(sum:"),
                    Style::bold(&format!("{})", total))
                );
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
        println!("{} {}", Style::success("[~] Execution time ="), Style::highlight(&exec_time));
        
        // Display complexity
        println!("{} {}", Style::info("[~] T(n) ="), Style::italic("T(n² × 2ⁿ)"));
        println!("{} {} {}", Style::info("[~] O(n) ="), Style::italic("O(n² × 2ⁿ)"), Style::italic(&format!("where n = {}", n)));
    }
}