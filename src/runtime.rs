
use std::error::Error;

use snafu::{AsErrorSource, ErrorCompat};

use crate::error::*;
static SEP:&str = "/";
static SCOPESEP:&str = ":";
#[derive(Clone)]
pub enum Parent {
    Namespace(Namespace), Folder(Folder)
}
#[derive(Clone)]
pub struct Module {
    name: String,
    content: BlnCode,
    parent: Option<Box<Parent>>,
}
// i never expected this, but linkage is a great idea for this
#[derive(Clone)]
pub struct File {
    name: String,
    content: Vec<u8>,
    parent: Option<Box<Parent>>,
}
#[derive(Clone)]
pub enum Objects {
    Module(Module),
    Namespace(Namespace),
    Folder(Folder),
    File(File),
    Package(Package),
}
pub trait ModuleTrait {
    fn set_parent(&mut self, parent: Box<Parent>);
    fn add_object(&mut self, module: Objects);
    fn try_import(&self, path: Vec<String>) -> Result<Module, ModuleError>;
    fn import(&self, path: Vec<String>) -> Module {
        match self.try_import(path.clone()) {
            Ok(module) => module,
            Err(e) => panic!("{}", e),
        }
    }
}
impl File {
    pub fn new(name: String, content: Vec<u8>) -> Self {
        Self {
            name,
            content,
            parent: None,
        }
    }
}
impl ModuleTrait for File {
    fn set_parent(&mut self, parent: Box<Parent>) {
        self.parent = Some(parent);
    }

    fn add_object(&mut self, _module: Objects) {
        unimplemented!()
    }
    fn try_import(&self, _path: Vec<String>) -> Result<Module, ModuleError> {
        unimplemented!()
    }
}
#[derive(Clone)]
pub struct Namespace {
    scope: Vec<String>,
    children:Vec<Box<Objects>>,
    name: String,
}
impl Namespace {
    pub fn new(name:String, scope:Vec<String>) -> Self {
        Self {
            name,
            scope,
            children:vec![]
        }
    }
}
impl ModuleTrait for Namespace {
    fn set_parent(&mut self, _parent: Box<Parent>) {
        unimplemented!()
    }
    fn add_object(&mut self, obj: Objects) {
        if Objects::Package != obj {
           panic!("{}", GeneralError::FSObjectIsntAPackage{})
        }
        self.children.push(Box::new(obj));
    }
    
    fn try_import(&self, _path: Vec<String>) -> Result<Module, ModuleError> {
        unimplemented!()
    }
}
#[derive(Clone)]
pub struct Folder {
    children: Vec<Box<Objects>>,
    name: String,
    parent: Option<Box<Parent>>,
}
impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
            parent: None,
        }
    }
}
impl ModuleTrait for Folder {
    fn set_parent(&mut self, parent: Box<Parent>) {
        self.parent = Some(parent);
    }
    fn add_object(&mut self, obj: Objects) {
        self.children.push(Box::new(obj));
    }
    fn try_import(&self, path: Vec<String>) -> Result<Module, ModuleError> {
        if path.len() == 0 {
            return Err(GeneralError::ObjectIsntAPackage { path: path.join(SEP) });
        }
        for child in &self.children {
            match &**child {
                Objects::Module(module) => {
                    if module.name == path[0] {
                        return Ok(module.clone());
                    }
                }
                Objects::Folder(folder) => {
                    if folder.name == path[0] {
                        return folder.try_import(path[1..].to_vec());
                    }
                }
                Objects::Package(package) => {
                    if package.name == path[0] {
                        return package.try_import();
                    }
                }
                _ => {}
            }
        }
        Err(ModuleError::ModuleNotFound { path:path.join(SEP) })
    }
}