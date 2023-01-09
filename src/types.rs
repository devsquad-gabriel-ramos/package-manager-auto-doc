use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ComposerPackage {
    pub require: HashMap<String, String>,
    #[serde(rename = "require-dev")]
    pub require_dev: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageRow {
    pub name: String,
    pub description: String,
    pub copyright: String,
    pub license: String,
    pub version: String,
    pub impementation_description: String,
    pub path: String,
    pub reference: String,
    pub language: String,
    pub install: String,
}

#[derive(Deserialize, Debug)]
pub struct PackagistResponse {
    pub package: PackagistInfo,
}

#[derive(Deserialize, Debug)]
pub struct PackagistInfo {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub language: String,
    pub versions: HashMap<String, PackagistVersion>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackagistVersion {
    pub version: String,
    pub license: Vec<String>,
}

#[derive(Debug)]
pub struct VersionNotFound {
    pub message: String,
}