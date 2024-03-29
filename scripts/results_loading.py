import numpy as np
import matplotlib.pyplot as plt


def load_optimum(instance_name: str) -> tuple[int, np.ndarray[np.int32]]:
    file_name = f"../qap_data/{instance_name}.sln"
    with open(file_name) as fp:
        lines: list[str] = fp.readlines()
        size, cost = lines[0].strip().split()
        size = int(size)
        cost = int(cost)
        all_numbers = []
        for line in lines[1:]:
            if len(line) == 0:
                break
            for x in line.strip().split():
                all_numbers.append(int(x))
    return cost, np.array(all_numbers, dtype=np.int32)


def load_history(
        instance_name: str, search_type: str
        ) -> list[list[tuple[int, int]]]:
    file_name = f"../saved_results/{search_type}/histories/{instance_name}.txt"
    ret_list = []
    with open(file_name) as fp:
        lines = fp.readlines()
        for line in lines:
            run, stats = line.split(';')
            run_list = []
            for pair in stats.split(','):
                t, cost = pair.split(':')
                t = int(t)
                cost = int(cost)
                run_list.append((t, cost))
            ret_list.append(np.array(run_list))
    return ret_list


class search_stats():
    def __init__(self, line: str) -> None:
        run_number, num_visited, num_evals, running_time, best_sol \
                = line.split(";")
        run_number = int(run_number)
        num_visited = int(num_visited)
        num_evals = int(num_evals)
        running_time = int(running_time)
        best_sol = [int(x) for x in best_sol.split(',')]
        self.run_number = run_number
        self.num_visited = num_visited
        self.num_evals = num_evals
        self.running_time = running_time
        self.best_sol = np.array(best_sol, dtype=np.int32)


if __name__ == "__main__":
    hist = load_history("tai10a", "steepest")
    plt.plot(hist[0][:, 0], hist[0][:, 1], "o")
    plt.show()
