pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

#[derive(Default)]
pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager { plugins: vec![] }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.iter().any(|p| p.name() == plugin.name()) {
            panic!("Plugin with name '{}' already exists", plugin.name())
        }
        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| plugin.name() == name)?;

        Some(self.plugins.remove(index))
    }

    pub fn execute_all(&self) {
        self.plugins.iter().for_each(|plugin| plugin.execute())
    }
}

pub fn main() {
    let _manager = PluginManager::new();
}
