use snafu::Snafu;
use toml::de::Error as ParserError;

#[derive(Debug, Snafu)]
pub enum Module {
    #[snafu(display("Module '{path}' not found"))]
    ModuleNotFound {path:String},
    #[snafu(display("Expected module '{path}' is a folder"))]
    ModuleIsAFolder {path:String}
}

#[derive(Debug, Snafu)]
pub enum Config {
    #[snafu(display("bln.toml config wanst found on the root project"))]
    ConfigNotFound {},
    #[snafu(display("bln.toml config's syntax was incorrect. Heres the error from the parser\n{error}"))]
    ConfigSyntaxError {error:ParserError},
    #[snafu(display("config object '{name}' is required inside '{from}'"))]
    ConfigRequiredObjectNotFound {name:String, from:String},
}