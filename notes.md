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