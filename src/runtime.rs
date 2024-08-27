use crate::Error;
#[derive(Clone)]
struct Module {
   name:String,
   childs:Vec<Module>,
   parent:Option<Box<Objects>>,
}
#[derive(Clone)]
pub enum Objects {
    Module(Module),
    Package(Package)
}
trait ModuleTrait {
    fn add_module (&mut self, module:Module);
    fn try_import (&self, path:String)->Option<Module>;
    fn import (&self, path:String)->Result<Module, Error::ModuleNotFound>;
}
#[derive(Clone)]
pub struct Folder {
    childs:Vec<Box<Objects>>,
    name:String,
    parent:Option<Box<Folder>>,
}
impl Folder {
    pub fn new(name:String) -> Self {
        Self { name, childs:[].to_vec() }
    }
}
impl ModuleTrait for Folder {
    
    fn add_module (&mut self, module:Module) {
        self.childs.push(module);
    }
    fn try_import (&self, path:String)->Option<Module> {
        for module in &self.childs {
            if module.name == path {
                return Some(module.clone());
            }
        }
        None
    }
    fn import (&self, path:String)->Result<Module, ModuleNotFoundError> {
        let modul: Option<Module> = self.try_import(path);
        if modul.is_none() {
            ModuleNotFoundSnafu
        }
    }
}