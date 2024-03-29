import numpy as np


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
