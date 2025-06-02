mod input;
mod graph;
mod output;
mod tsp;

use std::time::{Duration, Instant};
use input::Input;
use graph::Graph;
use tsp::TSPSolver;
use output::Output;

fn main() 
{
    println!("\nWelcome to TSP Solver using Dynamic Programming, made with love by Rzi\n");
    
    // Get input from user
    let mut input_handler = Input::new();
    match input_handler.get_input() 
    {
        Ok(_) => 
        {
            // Retrieve values from input
            let n: usize                  = input_handler.get_n();
            let city_names: Vec<String>   = input_handler.get_city_names();
            let adj_matrix: Vec<Vec<i32>> = input_handler.get_adj_matrix();
            
            // Display initial table
            Output::display_table(&city_names, &adj_matrix);
            
            // Construct graph
            let graph: Graph = Graph::new(n, city_names.clone(), adj_matrix.clone());
            let (num_roads, starting_node) = graph.get_graph_info();
            
            println!("\n[#] Graph Information:");
            println!("Number of cities: {}", n);
            println!("Total of roads: {}", num_roads);
            println!("Starting City: {} (Node {})", city_names[starting_node], starting_node);
            println!();
            
            // Solve TSP
            let start_time: Instant                     = Instant::now();
            let mut tsp_solver: TSPSolver<'_>           = TSPSolver::new(&graph);
            let (min_cost, tours) = tsp_solver.solve(starting_node);
            let duration: Duration                      = start_time.elapsed();
            
            // Display results
            Output::display_results(&city_names,
                                    starting_node,
                                    min_cost,
                                    &tours,
                                    duration,
                                    n,
                                    &adj_matrix
            );
        }

        Err(e) => 
        {
            eprintln!("Error: {}", e);
        }
    }
}