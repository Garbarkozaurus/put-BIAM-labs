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

## Lab4 - 2024-03-21

Random walk should always visit more solutions than random search

Plot - think about the goal of the visualization, consider the reader
Chart title can/should be removed if there is a caption
Font size in plots - not smaller than subscript/superscript in regular text;
not bigger than regular text
Colors are often overused; if multiple colors on one plot - make sure they are meaningful
Use tight layout

Instance,timeout for rand (microseconds, as used by the implementation),known_optimum
(No instance of size 10 has a solution provided - I have reason to believe that the optimum for tai10a is 135028, after performing 10000 runs of steepest and greedy each)
tai10a,  100,     135028
tai15a,  150,     388214
tai20a,  200,     703482
tai25b,  700,     344355646
tai30b,  1600,    637117113
tai35b,  2600,    283315445
tai40b,  5200,    637250948
tai50b,  12500,   458821517
tai60a,  13000,   7208572
tai80a,  35000,   13557864
tai100b, 240000,  1185996137
tai256c, 1500000, 44759294

## Lab 5 - 2024-03-28
No notes

## Lab 6 - 2024-04-04

QAP heuristic default: fix one element at a time
"Consider all pairs that could be fixed in one shot" - consider n^2

## Lab 7 - 11.04.2024

Stopping condition - up to you, check eKursy
Tabu list - based on lecture info, store just the identifiers

How to understand "no improvement"? No local improvement, or no local improvement?
Dealing with 95% acceptance chance
Is my 1% acceptance chance in the end correct?
