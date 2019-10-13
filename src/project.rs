use crate::config::Config;
use crate::instruction::Instruction;
use crate::project_graph::ProjectGraph;
use crate::*;
use log::*;
use openapiv3::OpenAPI;
use std::path::PathBuf;

pub struct Project {
    config: Config,
    spec: OpenAPI,
    // graphs: Vec<ProjectGraph>,
}

impl Project {
    pub fn read(path: &PathBuf) -> CliResult<Self> {
        let config = Config::read(path.join("config.yml"))?;
        let spec = load_openapi_spec()?;
        // let graphs = vec![];

        Ok(Project {
            config,
            spec,
            // graphs,
        })
    }

    pub fn copy_templates(&self) -> CliResult<()> {
        info!("Checking project directories");
        for (name, service) in self.config.services.clone() {
            let project_path = service.project_path.clone();
            // let project_path = PathBuf::from(".");
            if !util::file_exists(project_path.clone()) {
                warn!(
                    "Could not find local project for {} at {} - copying fresh template from {}",
                    name.clone(),
                    project_path,
                    service.template_path,
                );

                let template_path = util::expand_home_path(service.template_path.clone())?;
                util::copy_dir(template_path, ".")?;
            } else {
                info!("Found: {}", name);
            }
        }
        Ok(())
    }

    pub fn generate_instruction_tree(&self) -> CliResult<Vec<Instruction>> {
        info!("Generating project graphs");
        let spec_graph = project_graph::ProjectGraph::from(self.spec.clone());
        let mut instruction_tree = Vec::new();

        for model in spec_graph.models {
            instruction_tree.push(Instruction::AddModel(model.clone()));
        }

        Ok(instruction_tree)
    }

    pub fn generate_project_graphs(&self) -> CliResult<Vec<ProjectGraph>> {
        info!("Generating project graphs");

        for (name, service) in self.config.services.clone() {
            let spec_graph = service.extract_models()?;
        }

        Ok(vec![])
    }
}

// fn load_config_file() -> CliResult<config::Config> {
//     let config_path = PathBuf::from("./config.yml");

//     if !config_path.clone().exists() {
//         return Err(failure::format_err!(
//             "Could not find a config.yml. Try running the init command first if this is a new project."
//         ));
//     };

//     let config = config::Config::read(config_path)?;
//     info!("Read config file from ./config.yml");

//     Ok(config)
// }

fn load_openapi_spec() -> CliResult<OpenAPI> {
    let spec_path: PathBuf = PathBuf::from("openapi.yml");

    if !spec_path.exists() {
        return Err(failure::format_err!("Could not find openapi.yml"));
    };

    let spec = std::fs::read_to_string(spec_path).unwrap();
    let openapi: OpenAPI = serde_yaml::from_str(&spec).expect("Could not deserialize input");

    Ok(openapi)
}