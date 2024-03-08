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
