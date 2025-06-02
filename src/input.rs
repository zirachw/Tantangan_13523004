use std::io::{self, Write};
use std::fs;
use crate::style::Style;

pub struct Input 
{
    n: usize,
    city_names: Vec<String>,
    adj_matrix: Vec<Vec<i32>>,
}

impl Input {
    pub fn new() -> Self 
    {
        Input 
        {
            n: 0,
            city_names: Vec::new(),
            adj_matrix: Vec::new(),
        }
    }
    
    pub fn get_input(&mut self) -> Result<(), String> 
    {
        println!("{}", Style::header("[#] Input mode:"));
        println!("{}", Style::info("[-] 1. Read .txt file (relative path)"));
        println!("{}", Style::info("[-] 2. Manual Input"));
        print!("\n{} ", Style::prompt("[?] Choose option (1 or 2):"));
        io::stdout().flush().unwrap();
        
        let mut choice: String = String::new();
        loop
        {
            io::stdin().read_line(&mut choice).map_err(|_| Style::error("[!] Failed to read choice"))?;
            match choice.trim() {
                "1" => return self.read_from_file(),
                "2" => return self.read_manual(),
                _ => {
                    println!("{}", Style::warning("[!] Invalid choice, please enter 1 or 2"));
                    choice.clear();
                    print!("\n{} ", Style::prompt("[?] Choose option (1 or 2):"));
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
    
    fn read_from_file(&mut self) -> Result<(), String> 
    {
        println!("\n{}", Style::header("[#] Insert .txt file relative path (without .txt extension)"));
        println!("{}", Style::info("[-] Example: test/config-1"));
        print!("{} ", Style::prompt("[?] Your input:"));
        io::stdout().flush().unwrap();
        
        let mut path = String::new();
        loop {
            path.clear();
            io::stdin().read_line(&mut path).map_err(|_| Style::error("[!] Failed to read file path"))?;
            let trimmed_path = path.trim();
            if trimmed_path.is_empty() 
            {
                println!("{}", Style::warning("[!] Path cannot be empty, please try again."));
                continue;
            }

            let file_path: String    = format!("{}.txt", trimmed_path);
            let file_content: String = fs::read_to_string(&file_path)
                .map_err(|_| Style::error("[!] Failed to read the file"))?;

            return self.parse_content(&file_content);
        }
    }
    
    fn read_manual(&mut self) -> Result<(), String> 
    {
        println!("\n{}", Style::header("[#] Enter input in the following format:"));
        println!("{}", Style::info("[-] <N city>"));
        println!("{}", Style::info("[-] <City Names>"));
        println!("{}", Style::info("[-] <Adjacency Matrix>"));
        println!("\n{}", Style::header("[#] Example:"));
        println!("{}", Style::number("4"));
        println!("{}", Style::highlight("Medan Jakarta Samarinda Depok"));
        println!("{}", Style::number("0 10 15 20"));
        println!("{}", Style::number("10 0 35 25"));
        println!("{}", Style::number("15 35 0 30"));
        println!("{}", Style::number("20 25 30 0"));
        println!("\n{}", Style::italic("[#] Note: Matrix can be asymmetric (directed graph)"));
        println!("\n{}", Style::prompt("[?] Your input:"));
        
        let mut content = String::new();
        
        // Read N
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|_| Style::error("[!] Failed to read N"))?;
        content.push_str(&line);
        
        // Read city names
        line.clear();
        io::stdin().read_line(&mut line).map_err(|_| Style::error("[!] Failed to read city names"))?;
        content.push_str(&line);
        
        // Read matrix lines
        let n: usize = content.lines().next().unwrap().trim().parse().map_err(|_| Style::error("[!] Invalid number of cities"))?;
        
        for i in 0..n 
        {
            line.clear();
            io::stdin().read_line(&mut line).map_err(|_| Style::error(&format!("[!] Failed to read matrix row {}", i)))?;
            content.push_str(&line);
        }
        
        self.parse_content(&content)
    }
    
    fn parse_content(&mut self, content: &str) -> Result<(), String> 
    {
        let lines: Vec<&str> = content.lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        
        if lines.is_empty() 
        {
            return Err(Style::error("[!] Empty input"));
        }
        
        // Parse N
        self.n = lines[0].trim().parse().map_err(|_| Style::error("[!] Invalid number of cities"))?;
        
        if self.n < 2 
        {
            return Err(Style::error("[!] Number of cities must be at least 2"));
        }
        
        // Parse city names
        self.city_names = lines[1].split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if self.city_names.len() != self.n 
        {
            return Err(Style::error(&format!("[!] Expected {} city names, got {}", 
                self.n, self.city_names.len())));
        }
        
        // Parse adjacency matrix
        if lines.len() < self.n + 2 
        {
            return Err(Style::error("[!] Incomplete adjacency matrix"));
        }
        
        self.adj_matrix = Vec::new();
        for i in 0..self.n 
        {
            let row: Result<Vec<i32>, _> = lines[i + 2]
                .split_whitespace()
                .map(|s| s.parse())
                .collect();
            
            let row = row.map_err(|_| 
                Style::error(&format!("[!] Invalid number in matrix row {}", i)))?;
            
            if row.len() != self.n 
            {
                return Err(Style::error(&format!("[!] Row {} has {} elements, expected {}", 
                    i, row.len(), self.n)));
            }
            
            self.adj_matrix.push(row);
        }
        
        self.validate_matrix()?;
        Ok(())
    }
    
    fn validate_matrix(&self) -> Result<(), String> 
    {
        for i in 0..self.n 
        {
            if self.adj_matrix[i][i] != 0 
            {
                return Err(Style::error(&format!("[!] Diagonal element at ({}, {}) should be zero (distance from city to itself)", i, i)));
            }
        }
        
        for i in 0..self.n 
        {
            for j in 0..self.n 
            {
                if i != j && self.adj_matrix[i][j] < 0 
                {
                    return Err(Style::error(&format!("[!] Negative distance found at ({}, {}): {}", i, j, self.adj_matrix[i][j])));
                }
            }
        }
    
        Ok(())
    }
    
    pub fn get_n(&self) -> usize 
    {
        self.n
    }
    
    pub fn get_city_names(&self) -> Vec<String> 
    {
        self.city_names.clone()
    }
    
    pub fn get_adj_matrix(&self) -> Vec<Vec<i32>> 
    {
        self.adj_matrix.clone()
    }
}