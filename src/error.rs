use snafu::Snafu;
use toml::de::Error as ParserError;
#[derive(Debug, Snafu)]
pub enum GeneralError {
    #[snafu(display("{kind} '{path} wasnt found'"))]
    NotFound{ kind: String, path: String },
    #[snafu(display("Object '{path}' is not a package"))]
    FSObjectIsntAPackage { path: String },
    #[snafu(display("Object '{path}' is not a folder"))]
    FSObjectIsntAFolder { path: String },
    #[snafu(display("Object '{path}' is not a namespace"))]
    FSObjectIsntANamespace { path: String },
    #[snafu(display("Object '{path}' is not a file"))]
    FSObjectIsntAFile { path: String },
    #[snafu(display("Object '{path}' is not a module"))]
    FSObjectIsntAModule { path: String },
}
#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display(
        "bln.toml config's syntax was incorrect. Heres the error from the parser\n{error}"
    ))]
    ConfigSyntaxError { error: ParserError },
    #[snafu(display("config object '{name}' is required inside '{from}'"))]
    ConfigRequiredObjectNotFound { name: String, from: String },
}
