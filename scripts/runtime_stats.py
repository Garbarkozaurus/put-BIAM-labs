import sys
import numpy as np


def runtime_stats_from_file(file_path: str) -> tuple[float, float]:
    with open(file_path, "r") as fp:
        lines = fp.readlines()
        run_times = [int(line.split(';')[-2]) for line in lines]
    return (np.min(run_times), np.mean(run_times),
            np.max(run_times), np.std(run_times))


if __name__ == "__main__":
    steepest_file_path: str = f"./saved_results/steepest/{sys.argv[1]}.txt"
    greedy_file_path: str = f"./saved_results/greedy/{sys.argv[1]}.txt"
    s_min, s_mean, s_max, s_sd = runtime_stats_from_file(steepest_file_path)
    g_min, g_mean, g_max, g_sd = runtime_stats_from_file(greedy_file_path)
    print(f"-- {sys.argv[1]} --")
    print(f"Steepest: {s_mean}; {s_sd} ({s_min}-{s_max})")
    print(f"Greedy: {g_mean}; {g_sd} ({g_min}-{g_max})")
