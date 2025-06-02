# Travelling Salesman Problem Solver 
An interactive CLI program in Rust, implements _Dynamic Programming_ approach to find solutions in the Travelling Salesman Problem

---

<!-- CONTRIBUTOR -->
 <div align="center" id="contributor">
   <strong>
     <h3> Contributors </h3>
     <table align="center">
       <tr align="center">
         <td>NIM</td>
         <td>Name</td>
         <td>GitHub</td>
       </tr>
       <tr align="center">
         <td>13523004</td>
         <td>Razi Rachman Widyadhana</td>
         <td align="center" >
           <div style="margin-right: 20px;">
           <a href="https://github.com/zirachw" ><img src="https://avatars.githubusercontent.com/u/148220821?v=4" width="48px;" alt=""/> 
             <br/> <sub><b> @zirachw </b></sub></a><br/>
           </div>
         </td>
       </tr>
     </table>
   </strong>
 </div>

<div align="center">
  <h3 align="center"> Tech Stacks </h3>

  <p align="center">
    
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)][Rust-url]
  
  </p>
</div>

---

<!-- ABOUT -->
### About <a name="about"></a>

This project is the additional **one-night** project for IF2211 Algorithm Strategies year 2024/2025 course at Computer Science major in Bandung Institute of Technology.

The challenge is optional and has delighting bonus score for those who capable of...


--- 

### Description <a name="desc"></a>

The Traveling Salesman Problem (TSP) is a classic algorithmic problem in the field of computer science and operations research. 

<p align="center">
    <img width="584" alt="TSP" src="https://github.com/user-attachments/assets/b3abc63c-457d-45be-a8a0-9a59e0055682" />
</p>

It focuses on optimization: given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?

_**Problem Characteristics:**_

- **Input:**
  
  A weighted graph represented as an adjacency matrix where each edge weight represents the distance between two cities
  
- **Output:**
  
  The minimum cost tour that visits all cities exactly once and returns to the starting city
  
- **Constraint:**
  
  Each city must be visited exactly once (Hamiltonian cycle)

---

<!-- Algorithm -->

### Algorithm: Dynamic Programming Approach <a name="algorithm"></a>

The Dynamic Programming approach to TSP uses bit masking to represent subsets of cities and builds up solutions to larger subproblems from smaller ones.

_**Key Concepts:**_

- **State Representation:**

  `dp[mask][i]` represents the minimum cost to visit all cities in the subset represented by mask, ending at city $i$
  
- **Bit Masking:**

  A binary number where bit $i$ is $1$ if city $i$ is in the subset

- **Recurrence Relation:**

  ```c
  dp[mask][i] = min(dp[mask without i][j] + dist[j][i])
  for all j in mask where j != i
  ```

<br>

_**Algorithm Steps:**_

1. **Initialize base case:** `dp[{start}][start] = 0`

2. **Exploration:** For each subset size from $2$ to $n$:

   - For each subset of that size containing the start city
   - For each city in the subset as the ending city
   - Calculate minimum cost using the recurrence relation

3. **Goal:** minimum cost to complete the tour by returning to start

<br>

_**Complexity Analysis:**_

- **Time Complexity:** $O(n^2 \ \times \ 2^n \ )$ where $n$ is the number of cities
- **Space Complexity:** $O(n \ \times \ 2^n \ )$ for the DP table

---

### Preview <a name="preview"></a>

<p align="center">
    <img width="100%" src="https://github.com/user-attachments/assets/fafa0137-90f0-465c-aace-704eb76fb8b4" />
</p>

---

### Features

_**This project contains:**_

- **Main Program as TSP Solver with _Dynamic Programming_ approach**
- **Load `.txt` file mode**
- **Manually input mode**

_**Take a peek:**_

- **The core logic is located at `~/src/`**
- **Create Pull Request and Collaborate for project improvement**

---

### Installation <a name="install"></a>

> [!NOTE]  
> Before you start, install these dependencies first with links given :D
> - [**Git**](Git-url) - 2.47.0 or later
> - [**Rust**](Rust-url) - 1.87.0

### Initialization

- **Clone the repository**

  ```
  git clone https://github.com/zirachw/Tantangan_13523004
  ```
  
### Command-Line Interface (CLI) Mode

- Run the following command to start the application in `CLI` mode:

  ```bash
  # Build the project
  cargo build --release
  
  # Run the project
  cargo run --release
  ```

---

### Usage <a name="usage"></a>

_**The input format:**_

The program supports two input modes:

1. **File Input**

   Create a `.txt` file with the following format:
   ```txt
   # Example: test/config-1.txt
   3
   Samarinda Balikpapan Kutai
   0 1 2
   1 0 3
   2 3 0
   ```

   Then, choose `1` for file input mode and insert `test/config-1` without the extension

2. **Manual Input**

   Choose `2` for manual input mode and enter the data directly when prompted:
   ```txt
   3
   Samarinda Balikpapan Kutai
   0 1 2
   1 0 3
   2 3 0
   ```

<br>

_**The output format:**_

- A formatted table showing the adjacency matrix
- Graph information (number of cities and roads)
- The shortest tour(s) found with their cost
- An example of the shortest tour path
- Execution time

<br>

_**Screenshot Example:**_

<p align="center">
    <img width="100%" alt="TSP" src="https://github.com/user-attachments/assets/4bfcda61-60ad-499d-8be4-6c690df92333" />
    <img width="100%" alt="TSP" src="https://github.com/user-attachments/assets/f2dcaaab-3b8e-41a7-a634-e9d32b968383" />
</p>

---

### Project Structure

This is the visualization of the project structure:

```txt
📂 Tantangan_13523004/
├── Cargo.toml          # Rust project configuration
├── README.md           # This file
└── 📂 src/
    ├── main.rs         # Main program entry point
    ├── input.rs        # Input handling and validation
    ├── graph.rs        # Graph data structure
    ├── style.rs        # Styling related support
    ├── tsp.rs          # TSP algorithm implementation
    └── output.rs       # Output formatting and display
```

---

### Acknowledgements <a name="acknowledgements"></a>

Special thanks to:

1. Dr. Nur Ulfa Maulidevi, S.T, M.Sc. for the guidance over 13 weeks lecturing the K1 IF2211 Algorithm Strategies course
2. Dr. Ir. Rinaldi Munir, M.T. for this additional one-night challenge to keep the spirit of IF'23 students.
3. Beloved friends who always supporting and keeping my sanity till this day

---

<h3 align="center">
Rzi • 13523004 • © 2025 
</h3>

<!-- MARKDOWN LINKS & IMAGES -->
[Rust-url]: https://www.rust-lang.org/
