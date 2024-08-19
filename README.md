## Overview
This Rust program finds and lists the generators of the additive group $\mathbb{Z}_n$, where $\mathbb{Z}_n$ represents the set of integers $\\{0,1,2,\cdots,n-1\\}$ under addition modulo $n$. In this regard, a generator is an element that can be used to generate all of the group's elements by repeated addition modulo $n$.
## How It Works
- find_generators $(n: u32) -> Vec<u32>$: This function iterates through all numbers from $1$ to $n−1$ and checks if each number is a generator using the is_generator function. It returns a vector of all generators found.
- is_generator $(g: u32, n: u32) -> bool$: This function checks if the given number g is a generator for the group $\mathbb{Z}_n$. It repeatedly adds $g$ to itself modulo $n$ and checks if it produces all possible elements of $\mathbb{Z}_n$.
- Main Function: The main function sets the value of $n$, calls the find_generators function, and prints the generators.
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- You can change the values of $n$ in the main function to test different cases.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
git clone https://github.com/cypriansakwa/Generators_of_an_Additive_Group_Z_n.git
cd Generators_of_an_Additive_Group_Z_n
