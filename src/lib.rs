extern crate pyo3;
mod models;
use std::{collections::HashMap};

use models::intfloats::IntFloats;
use pyo3::{prelude::*, exceptions::{PyValueError, PyWarning}};
use rand::{rngs::StdRng, SeedableRng};

#[pyclass]
struct GA{
    random_seed: Option<u64>,
    suppress_warnings: bool,
    mutation_by_replacement: bool,
    gene_space_nested: bool,
    gene_space: Option<PyObject>,
    init_range_low: IntFloats,
    init_range_high: IntFloats
}

#[pymethods]
impl GA {
   
    // https://github.com/ahmedfgad/GeneticAlgorithmPython/blob/master/pygad.py#L16
    #[new]
    #[pyo3(signature = (num_generations,num_parents_mating,
        fitness_func, fitness_batch_size=None, initial_population=None, sol_per_pop=None, num_genes=None,
        init_range_low=None, init_range_high=None, gene_type=None, parent_selection_type=None, keep_parents=None,
        keep_elitism=None, K_tournament=None, crossover_type=None, crossover_probability=None, mutation_type=None,
        mutation_probability=None, mutation_by_replacement=None, mutation_percent_genes=None, mutation_num_genes=None,
        random_mutation_min_val=None, random_mutation_max_val=None, gene_space=None, allow_duplicate_genes=None,
        on_start=None, on_fitness=None, on_parents=None, on_crossover=None, on_mutation=None, callback_generation=None,
        on_generation=None, on_stop=None, delay_after_gen=None, save_best_solutions=None, save_solutions=None,
        suppress_warnings=None, stop_criteria=None, parallel_processing=None, random_seed=None
    ))]
    fn py_new(py: Python<'_>,
           num_generations: i64,
           num_parents_mating: i64,
           fitness_func: PyObject,
           fitness_batch_size: Option<i64>,
           initial_population: Option<PyObject>,
           sol_per_pop: Option<i64>,
           num_genes: Option<i64>,
           init_range_low: Option<IntFloats>,
           init_range_high: Option<IntFloats>,
           gene_type: Option<PyObject>,
           parent_selection_type: Option<String>,
           keep_parents: Option<i64>,
           keep_elitism: Option<i64>,
           K_tournament: Option<i64>,
           crossover_type: Option<String>,
           crossover_probability: Option<f64>,
           mutation_type: Option<String>,
           mutation_probability: Option<f64>,
           mutation_by_replacement: Option<bool>,
           mutation_percent_genes: Option<String>,
           mutation_num_genes: Option<i64>,
           random_mutation_min_val: Option<IntFloats>,
           random_mutation_max_val: Option<IntFloats>,
           gene_space: Option<PyObject>,
           allow_duplicate_genes: Option<bool>,
           on_start: Option<PyObject>,
           on_fitness: Option<PyObject>,
           on_parents: Option<PyObject>,
           on_crossover: Option<PyObject>,
           on_mutation: Option<PyObject>,
           callback_generation: Option<PyObject>,
           on_generation: Option<PyObject>,
           on_stop: Option<PyObject>,
           delay_after_gen: Option<f64>,
           save_best_solutions: Option<bool>,
           save_solutions: Option<bool>,
           suppress_warnings: Option<bool>,
           stop_criteria: Option<PyObject>,
           parallel_processing: Option<PyObject>,
           random_seed: Option<u64>) -> PyResult<Self>{
            println!("Executing...");

            let seed = if let Some(seed) = random_seed {
                let mut rng = StdRng::seed_from_u64(seed);
                Some(rng)
            }else{
                None
            };

            let Some(suppress_warnings) = suppress_warnings else {
                return Err(PyValueError::new_err("The expected type of the 'suppress_warnings' parameter is not bool"));
            };

            let Some(mutation_by_replacement) = mutation_by_replacement else {
                return Err(PyValueError::new_err("The expected type of the 'suppress_warnings' parameter is not bool"));
            };

            let mut gene_space_nested = false;
            // Validate gene_space
            if let Some(gene_space) = gene_space.clone() {
                if let Ok(gene_space) = gene_space.extract::<Vec<PyObject>>(py){
                    if gene_space.len() == 0 {
                        return Err(PyValueError::new_err("'gene_space' cannot be empty (i.e. its length must be >= 0)."));
                    }else{
                        // for iter
                        for (index, gene) in gene_space.iter().enumerate(){
                            // https://github.com/ahmedfgad/GeneticAlgorithmPython/blob/master/pygad.py#L154
                            let element = gene.extract::<Option<Vec<PyObject>>>(py);
                            if let Ok(None) = element {
                                continue;
                                // self.gene_space_nested = True
                            }
                            if let Ok(Some(element)) = element{
                                if element.len() == 0 {
                                    return Err(PyValueError::new_err(format!("The element indexed {index} of 'gene_space' cannot be empty (i.e. its length must be >= 0")));
                                }
                                //check if all values in gene_space are numbers
                                for val in element{
                                    if let Err(_) = val.extract::<Option<IntFloats>>(py){
                                        return Err(PyValueError::new_err("All values in the sublists inside the 'gene_space' attribute must be numeric of type int/float/None"));
                                    }
                                    gene_space_nested = true;
                                }
                            }
                            if let Ok(Some(element)) =  gene.extract::<Option<HashMap<String, PyObject>>>(py){
                                if element.values().len() == 2{
                                    // verify if low and high are in element
                                    if element.contains_key("low") && element.contains_key("high"){
                                        continue;
                                    }else{
                                        // get keys as string
                                        let keys = element.keys().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");

                                        return Err(PyValueError::new_err(format!("When an element in the 'gene_space' parameter is of type dict, then it can have the keys 'low', 'high', and 'step' (optional) but the following keys found: {keys}")));
                                    }
                                }else if element.values().len() == 3 {
                                    // verify if low and high are in element
                                    if element.contains_key("low") && element.contains_key("high") && element.contains_key("step"){
                                        continue;
                                    }else{
                                        // get keys as string
                                        let keys = element.keys().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");

                                        return Err(PyValueError::new_err(format!("When an element in the 'gene_space' parameter is of type dict, then it can have the keys 'low', 'high', and 'step' (optional) but the following keys found: {keys}")));
                                    }
                                }else{
                                    return Err(PyValueError::new_err(format!("When an element in the 'gene_space' parameter is of type dict, then it must have only 2 items")));
                                }
                            };
                            if let Err(_) = gene.extract::<IntFloats>(py){
                                return Err(PyValueError::new_err("Unexpected type for the element indexed {index} of 'gene_space'. The accepted types are list/tuple/range/numpy.ndarray of numbers, a single number (int/float), or None."));
                            }
                        }
                        
                    }
                }

                // https://github.com/ahmedfgad/GeneticAlgorithmPython/blob/master/pygad.py#L187
                // TODO: wrap in a function?
                if let Ok(gene_space) = gene_space.extract::<HashMap<String, PyObject>>(py){
                    if gene_space.values().len() == 2{
                        // verify if low and high are in element
                        if gene_space.contains_key("low") && gene_space.contains_key("high"){
                        }else{
                            // get keys as string
                            let keys = gene_space.keys().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");

                            return Err(PyValueError::new_err(format!("When the 'gene_space' parameter is of type dict, then it can have only the keys 'low', 'high', and 'step' (optional) but the following keys found: {keys}")));
                        }
                    }
                    else if gene_space.values().len() == 3 {
                        // verify if low and high are in element
                        if gene_space.contains_key("low") && gene_space.contains_key("high") && gene_space.contains_key("step"){
                        }else{
                            // get keys as string
                            let keys = gene_space.keys().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");

                            return Err(PyValueError::new_err(format!("When the 'gene_space' parameter is of type dict, then it can have only the keys 'low', 'high', and 'step' (optional) but the following keys found: {keys}")));
                        }
                    }else{
                        return Err(PyValueError::new_err(format!("When the 'gene_space' parameter is of type dict, then it must have only 2 items")));
                    }

                }
                else {
                    return Err(PyValueError::new_err(format!("When the 'gene_space' parameter is of type dict, then it must have only 2 items found.")));
                }
                // https://github.com/ahmedfgad/GeneticAlgorithmPython/blob/master/pygad.py#L203
            }else{
                return Err(PyValueError::new_err("The expected type of 'gene_space' is list, tuple, range, or numpy.ndarray"));
            }

            // Validate init_range_low and init_range_high
            let Some(init_range_low) = init_range_low else {
                return Err(PyValueError::new_err("The value passed to the 'init_range_low' parameter must be either integer or floating-point number."));
            };
            let Some(init_range_high) = init_range_high else {
                return Err(PyValueError::new_err("The value passed to the 'init_range_high' parameter must be either integer or floating-point number."));
            };
            // Validate random_mutation_min_val and random_mutation_max_val
            let Some(random_mutation_min_val) = random_mutation_min_val else {
                return Err(PyValueError::new_err("The expected type of the 'random_mutation_min_val' parameter is numeric."));
            };
            let Some(random_mutation_max_val) = random_mutation_max_val else {
                return Err(PyValueError::new_err("The expected type of the 'random_mutation_max_val' parameter is numeric."));
            };
            if random_mutation_min_val == random_mutation_max_val && suppress_warnings {
                // TODO: Show it as a Warning
                println!("The values of the 2 parameters 'random_mutation_min_val' and 'random_mutation_max_val' are equal and this causes a fixed change to all genes.")
            }

            println!("Finishing execution...");

        Ok(
            GA{
                random_seed,
                suppress_warnings,
                mutation_by_replacement,
                gene_space_nested,
                gene_space,
                init_range_low,
                init_range_high
            }
        )
}
}


#[pymodule]
#[pyo3(name = "_genetic_algorithm")]
fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<GA>()?;
    Ok(())
}
