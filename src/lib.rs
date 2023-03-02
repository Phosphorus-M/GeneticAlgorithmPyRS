extern crate pyo3;

use pyo3::{prelude::*, exceptions::PyValueError};
use rand::{rngs::StdRng, SeedableRng, Rng};

#[pyfunction]
#[pyo3(name = "add_two_numbers")]
fn add_two_numbers(py: Python<'_>, a:PyObject, b: PyObject) -> PyResult<PyObject> {
    if let (Ok(int1), Ok(int2)) = (a.extract::<i64>(py), b.extract::<i64>(py)){
        return Ok((int1 + int2).to_object(py));
    }
    if let (Ok(float1), Ok(float2)) = (a.extract::<f64>(py), b.extract::<f64>(py)){
        return Ok((float1 + float2).to_object(py));
    }
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Not supported"))
}

#[pyclass]
struct GA{
    random_seed: Option<u64>,
    suppress_warnings: bool,
    mutation_by_replacement: bool
}

#[pymethods]
impl GA {
    /*
    def __init__(self, 
                 num_generations, 
                 num_parents_mating, 
                 fitness_func,
                 fitness_batch_size=None,
                 initial_population=None,
                 sol_per_pop=None, 
                 num_genes=None,
                 init_range_low=-4,
                 init_range_high=4,
                 gene_type=float,
                 parent_selection_type="sss",
                 keep_parents=-1,
                 keep_elitism=1,
                 K_tournament=3,
                 crossover_type="single_point",
                 crossover_probability=None,
                 mutation_type="random",
                 mutation_probability=None,
                 mutation_by_replacement=False,
                 mutation_percent_genes='default',
                 mutation_num_genes=None,
                 random_mutation_min_val=-1.0,
                 random_mutation_max_val=1.0,
                 gene_space=None,
                 allow_duplicate_genes=True,
                 on_start=None,
                 on_fitness=None,
                 on_parents=None,
                 on_crossover=None,
                 on_mutation=None,
                 callback_generation=None,
                 on_generation=None,
                 on_stop=None,
                 delay_after_gen=0.0,
                 save_best_solutions=False,
                 save_solutions=False,
                 suppress_warnings=False,
                 stop_criteria=None,
                 parallel_processing=None,
                 random_seed=None):
     */
    #[new]
    fn py_new(num_generations: i64,
           num_parents_mating: i64,
           fitness_func: PyObject,
           fitness_batch_size: Option<i64>,
           initial_population: Option<PyObject>,
           sol_per_pop: Option<i64>,
           num_genes: Option<i64>,
           init_range_low: Option<f64>,
           init_range_high: Option<f64>,
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
           random_mutation_min_val: Option<f64>,
           random_mutation_max_val: Option<f64>,
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
            if let Some(gene_space) = gene_space {
                if let Ok(gene_space) = gene_space.extract::<Vec<PyObject>>(py){
                    if gene_space.len() == 0 {
                        return Err(PyValueError::new_err("'gene_space' cannot be empty (i.e. its length must be >= 0)."));
                        // if let Ok(gene_space) = gene_space[0].extract::<Vec<PyObject>>(py){
                        //     gene_space_nested = true;
                        // }
                    }
                }
            }

        Ok(
            GA{
                random_seed,
                suppress_warnings,
                mutation_by_replacement,
            }
        )
}


#[pymodule]
#[pyo3(name = "_add_two_numbers")]
fn init(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_two_numbers, m)?)?;
    Ok(())
}
