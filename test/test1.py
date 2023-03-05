from genetic_algorithm_py_rs import GA

num_generations = 300
num_parents_mating = 20
sol_per_pop = 220
parent_selection_type = "rank"
keep_parents = 9
crossover_type = "two_points"
mutation_type = "random"
mutation_probability = 0.11
num_genes = 15
init_range_low = 1

def calculate_fitness(genes):
    return sum(genes)

GA(num_generations=num_generations,
  num_parents_mating=num_parents_mating,
  sol_per_pop=sol_per_pop,
  num_genes=num_genes,
  init_range_low=init_range_low,
  init_range_high=4,
  parent_selection_type=parent_selection_type,
  keep_parents=keep_parents,
  crossover_type=crossover_type,
  mutation_type=mutation_type,
  fitness_func=calculate_fitness,
  #mutation_num_genes=mutation_num_genes,
  mutation_probability=mutation_probability,
  allow_duplicate_genes=False,
  gene_space=range(1,23),
  #parallel_processing=["thread", 5],
  mutation_by_replacement=True,
  initial_population=[],
  suppress_warnings=False,
  gene_type=int)