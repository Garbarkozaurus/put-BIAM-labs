## Lab1 - 2024-02-29

Must support at least 10 instances
All QAP instances follow the same format

Pick randomly selected element at the end; then as the penultimate...
Use only static arrays

```
x_1 = rand(N)
x_2 = (rand(N-1)+x_1+1) % N
```
Heuristc, Random Walk, Random Search
Limit time, but also at least 10 executions

HW: implement generating a random permutation, maybe load some instances, try timing something


## Lab2 - 2024-03-07

2-opt neighborhood
xor trick for swapping -
```
x = x xor y
y = x xor y
x = x xor y
```
x=0011
y=0101
1. x=0110
2. y=0011
3. x=0101


number of evaluations, number of neighborhoods, running time, quality of starting solution, final permutation, initial permutation, quality of final permutation - will tell next week

Implement basic greedy or steepest
a[..12].copy_from_slice(&[7, 0, 5, 1, 10, 9, 2, 4, 8, 6, 11, 3]);

## Lab3 - 2024-03-14

Qap - swap products of rows and columns
Greedy - how about not permuting a neighborhood, but just the starting index?

IMPORTANT - give random methods similar amount of time as LS
No need to see specific values
Neighborhood description can be short, ~1 sentence (1 type is enough)

In plots - on x-axis, instance size or instance name
Chose and justify the selected Q - quality measure comparing obtained solutions to the optima
Include whiskers when plotting averages
Efficiency is supposed to include quality and time in a single number
Algorithm steps - jumping from neighbor to neighbor
Evaluated - even delta calculation. "Number of explored solutions"
In point 4 - best result over given number of runs; supposed to be nonincreasing
Can include cumulative average
Permutation similarity in QAP - number of the same numbers at the same position
TSP - number of shared edges
Make sure to normalise
General conclusions: some algorithm is always better/worse then some other
Less general: A is better than B on "large instances"
... -> more and more specific conditions
Difficulties: of any type - implementation, running time, conclusions...
UPLOAD: pdf and code source files (not for plotting, just the computational part)
