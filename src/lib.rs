use pyo3::prelude::*;

use nalgebra::{Matrix3, Vector3, RowVector3};

use inertia_rs::{System, Inertia};

#[pyclass(unsendable)]
struct PySystem(System);

impl Inertia for PySystem {
    fn get_position(&self) -> Vector3<f32> {
        self.0.get_position()
    }

    fn get_mass(&self) -> f32 {
        self.0.get_mass()
    }

    fn center_of_mass(&self) -> Vector3<f32> {
        self.0.center_of_mass()
    }

    fn total_inertia(&self) -> Matrix3<f32> {
        self.0.total_inertia()
    }

    fn total_mass(&self) -> f32 {
        self.0.total_mass()
    }
}

impl<'source> FromPyObject<'source> for PySystem {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let mass = ob.getattr("mass")?.extract()?;
        let position_py: Vec<f32> = ob.getattr("position")?.extract()?;
        let mut moment_of_inertia_py: Vec<Vec<f32>> = ob.getattr("moment_of_inertia")?.extract()?;

        let mut subsystems: Vec<Box<dyn Inertia>> = Vec::new();
        for subsys in ob.getattr("subsystems")?.extract::<Vec<PySystem>>()? {
            subsystems.push(Box::new(subsys));
        }

        let description = ob.getattr("description")?.extract()?;
        
        let position = Vector3::from_vec(position_py);
        let moment_of_inertia = Matrix3::from_rows(&[
                RowVector3::from_vec(moment_of_inertia_py.remove(0)),
                RowVector3::from_vec(moment_of_inertia_py.remove(1)),
                RowVector3::from_vec(moment_of_inertia_py.remove(2)),
                ]);
        Ok(PySystem(
            System::new(mass, position, moment_of_inertia, subsystems, description)
            )
          )
    }
}

#[pymethods]
impl PySystem {
    #[new]
    fn new_py(
        mass: f32,
        position_py: Vec<f32>,
        mut moment_of_inertia_py: Vec<Vec<f32>>,
        subsystems_py: Vec<PySystem>,
        description: String,
    ) -> PySystem {
        let position = Vector3::from_vec(position_py);
        
        let moment_of_inertia = Matrix3::from_rows(&[
                RowVector3::from_vec(moment_of_inertia_py.remove(0)),
                RowVector3::from_vec(moment_of_inertia_py.remove(0)),
                RowVector3::from_vec(moment_of_inertia_py.remove(0)),
                ]);
        
        let mut subsystems: Vec<Box<dyn Inertia>> = Vec::new();
        for subs in subsystems_py.into_iter() {
            subsystems.push(Box::new(subs));
        }

        PySystem(System::new(mass, position, moment_of_inertia, subsystems, description))
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn py_inertia_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySystem>()?;

    Ok(())
}
