use crate::setup::config::uis_data_path;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::ErrorKind, path::PathBuf};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortMapping(BTreeMap<String, u16>);

impl PortMapping {
  fn path() -> PathBuf {
    uis_data_path().join(String::from("port_mapping.yml"))
  }

  pub fn get_ui_port_for_app(&self, app_id: &String) -> Option<u16> {
    self.0.get(app_id).map(|p| p.clone())
  }

  pub fn read_port_mapping() -> Result<PortMapping, String> {
    match fs::read_to_string(Self::path()) {
      Err(error) => match error.kind() {
        ErrorKind::NotFound => Ok(PortMapping(BTreeMap::new())),
        _ => Err(format!("Error reading the UIs port mapping {:?}", error)),
      },
      Ok(contents) => {
        let mapping: PortMapping =
          serde_yaml::from_str(contents.as_str()).or(Err("Malformed port mapping file"))?;
        Ok(mapping)
      }
    }
  }

  pub fn set_available_ui_port_for_app(&mut self, app_id: String) -> Result<u16, String> {
    let port = self.get_next_available_port();

    self.0.insert(app_id, port);

    self.write_port_mapping()?;

    Ok(port)
  }

  pub fn remove_app_from_mapping(&mut self, app_id: String) -> Result<(), String> {
    self.0.remove(&app_id);

    self.write_port_mapping()?;

    Ok(())
  }

  fn get_next_available_port(&self) -> u16 {
    portpicker::pick_unused_port().expect("No ports free")
  }

  fn write_port_mapping(&self) -> Result<(), String> {
    let s = serde_yaml::to_string(&self).or(Err("Could not format into yaml"))?;

    fs::write(Self::path(), s).or(Err("Could not write port mapping to file disk".into()))
  }
}

pub fn app_ui_folder_path(app_id: String) -> PathBuf {
  uis_data_path().join(app_id)
}
