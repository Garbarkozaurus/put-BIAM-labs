import numpy as np
import matplotlib.pyplot as plt


SEARCH_TYPES = ["random_search", "random_walk", "greedy", "steepest"]

LABELS_SEARCH_TYPES = {"random_search": "R", "random_walk": "RW",
                       "greedy": "G", "steepest": "S"}

INSTANCE_NAMES = ["tai10a", "tai20a", "tai30b", "tai40b", "tai50b", "tai60a",
                  "tai80a", "tai100b"]

COLOR_DICT = {
    "random_search": "orange",
    "random_walk": "olive",
    "greedy": "blue",
    "steepest": "red"
}


def extract_monitored_stat_by_column(file_path: str, column_number: int) -> np.ndarray[np.int32]:
    with open(file_path) as fp:
        lines = fp.readlines()
        values = [int(line.split(';')[column_number]) for line in lines]
    return np.array(values)


def worst_avg_best_cost_instance_search(instance_name: str, search_type: str
                                        ) -> tuple[int, float, float]:
    file_name = f"../saved_results/{search_type}/histories/{instance_name}.txt"
    costs = []
    with open(file_name) as fp:
        lines = fp.readlines()
        for line in lines:
            last_cost = line.split(';')[1].split(",")[-1].split(":")[1]
            costs.append(int(last_cost))
    return np.min(costs), np.mean(costs), np.max(costs)


def extract_best_costs(instance_name: str, search_type: str) -> np.ndarray[np.int32]:
    file_name = f"../saved_results/{search_type}/histories/{instance_name}.txt"
    costs = []
    with open(file_name) as fp:
        lines = fp.readlines()
        for line in lines:
            last_cost = line.split(';')[1].split(",")[-1].split(":")[1]
            costs.append(int(last_cost))
    return np.array(costs, dtype=np.int32)


def extract_initial_costs(instance_name: str, search_type: str) -> np.ndarray[np.int32]:
    file_name = f"../saved_results/{search_type}/histories/{instance_name}.txt"
    costs = []
    with open(file_name) as fp:
        lines = fp.readlines()
        for line in lines:
            last_cost = line.split(';')[1].split(",")[0].split(":")[1]
            costs.append(int(last_cost))
    return np.array(costs, dtype=np.int32)


def efficiency_values(optimal_cost: int, instance_name: str, search_type: str,
                      mode: str = "evaluated") -> np.ndarray[np.float64]:
    """Efficiency is measured as: (neighborhood_size * best_cost)/(optimal_cost * num_evaluated_solutions)
    Therefore, the maximum efficiency of 1.0 is obtained by an algorithm which
    immediately identifies the global optimum. Note that num_evaluated_solutions
    is taken from the entire running time of the algorithm, not how many it took
    to find the best solution.
    Alternatively, the number of visited solutions can be used, giving very
    different, but still interesting results"""
    if instance_name[3:6].isnumeric():
        instance_size = float(instance_name[3:6])
    else:
        instance_size = float(instance_name[3:5])
    norm_factor = 0.5 * instance_size * (instance_size-1)
    monitored_stats_file_name = f"../saved_results/{search_type}/{instance_name}.txt"
    # column 2 stores information about the number of evaluated solutions
    col_number = {"visited": 1, "evaluated": 2, "running_time": 3}[mode]
    num_evals = extract_monitored_stat_by_column(monitored_stats_file_name, col_number)
    costs = extract_best_costs(instance_name, search_type)
    costs = costs.astype(np.float64)
    # the first two methods are numerically unstable
    # efficiencies = np.array([cost / (evals * optimal_cost) for cost, evals in zip(costs, num_evals)])
    # efficiencies = costs / (num_evals * optimal_cost)
    efficiencies = qualities_from_costs(costs, optimal_cost)
    efficiencies = efficiencies / num_evals
    efficiencies *= norm_factor
    return efficiencies


def solution_similarity(sol: np.ndarray[np.int32], opt_sol: np.ndarray[np.int32]) -> float:
    """IMPORTANT: ADDS 1 TO EVERY ELEMENT OF `SOL` (without modifying it) to
    take into account the optima being given in 1-indexed notation"""
    local_sol = sol.copy()
    local_sol += 1
    return len(np.where(local_sol == opt_sol)[0]) / len(opt_sol)


def qualities_from_costs(costs: np.ndarray[np.int32], optimal_cost: int) -> np.ndarray[np.float32]:
    vectorized_divide = np.vectorize(lambda x: optimal_cost/x)
    qualities = vectorized_divide(costs)
    return qualities


def extract_best_sols(instance_name: str, search_type: str) -> np.ndarray[np.int32, np.int32]:
    file_path = f"../saved_results/{search_type}/{instance_name}.txt"
    with open(file_path) as fp:
        lines = fp.readlines()
        sol_strings = [line.split(';')[-1] for line in lines]
        sol_indices = [list(map(int, sol_str.split(','))) for sol_str in sol_strings]
    return np.array(sol_indices)


def color_explaining_plot() -> None:
    for i, (search, color) in enumerate(COLOR_DICT.items()):
        plt.bar(i*4, 4, 4, color=color, label=search, align="center")
        plt.text(i*4, 2, search, ha="center", va="center", color="white", weight="bold")

    plt.gcf().set_size_inches(10, 2.5)
    plt.axis("off")
    plt.show()


if __name__ == "__main__":
    color_explaining_plot()
