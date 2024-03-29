Experiments with random search algorithm R, its much faster variant – random walk RW, simple (if possible non-deterministic) heuristic H, and greedy G and steepest S in the "multi-random start" mode. Statistical evaluation on at least 8 instances, in each case 10 runs of the algorithm in order to obtain reliable performance information.

One run of the R and RW algorithms should be given (more or less) the same time as one run of the G or S algorithm on a given instance.

Report on experiments: concise, consistent notation, appropriate number of significant digits, meaningful and legible plots and tables (prefer plots to large tables with lots of numbers) included in the text. You may use this template.

---

Mandatory parts in the report:

0. Short description of the problem, its applications and interpretations, complexity – up to 20 lines \
Justification for the choice of instances; names of chosen instances
Two or three sentences characterizing your implementation \
1. Description of the neighborhood operators used (at least one); neighborhood size
2. Comparison of the performance of 5 algorithms and implemented types of neighborhoods on all problem instances – plots:
    - Quality = distance from the optimum (according to what measure?), the average and the best case (optionally: also the worst case).
    - Running time (average)
    - Efficiency of algorithms (average) – i.e., quality over time (suggest a good measure and justify your choice)
    - G,S: average number of algorithm steps (step = changing the current solution)
    - G,S,R,RW: average number of evaluated (i.e., visited – full or partial evaluation) solutions

For the averages, we assess the stability of the results (standard deviations should always be shown along with the averages).

3. G,S – plot: quality of the initial solution vs. quality of the final solution (at least 200 repetitions, use small points) for several interesting instances; interesting instances are the ones that demonstrate some heterogeneity. For the charts shown, provide and interpret the correlation

4. G,S – plot: the number of restarts (up to at least 300, horizontal axis) in multi-random start vs. average and best of solutions found so far, for two (or a few) selected instances. Is it worth repeating the algorithm? If so, how many times?

5. Objective assessment of the similarity of locally optimal solutions found for two selected instances, and the assessment of their similarity to the global optimum (if, for ATSP, we don't know the global one, use the best local one). For example: a plot of at least 100 points: x=quality, y=similarity

6. Conclusions (from general to specific) from the experiments

7. Difficulties encountered

8. Justification of the introduced improvements, suggestions for improvements and their expected effects

---

Optional parts:

- More neighborhood operators (2-3), in particular 3-OPT
- The impact of using a non-random initial solution on the performance of the algorithm (for example H or anti-H)
- Checking the correlation between the local optima found – scatter plot: solution quality vs. its average similarity to not-worse solutions
- Comparison of the effectiveness of 2-OPT: swapping a pair of elements vs. reversing the order of a substring
- How can the performance of optimization algorithms be assessed? What could be the criteria other than these already used?

---

Extra parts – not needed, but...:

- Problem instance generator for QAP/ATSP/STSP
- Exact (full) search (for small instances), if the optima are not known

---
