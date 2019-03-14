//! Models mapping the API.

use std::fmt;

/// Indicator of how to perform a search.
#[derive(Copy, Clone, Debug)]
pub enum SearchBy {
    /// Indicator to search by name, ascending.
    Name,
    /// Indicator to search by description.
    NameDesc,
    /// Indicator to search by maintainer.
    Maintainer,
    /// Indicator to search by depended packages.
    Depends,
    /// Indicator to search by make-depended packages.
    MakeDepends,
    /// Indicator to search by check-depended packages.
    CheckDepends,
    /// Indicator to search by optionally-depended packages.
    OptDepends,
}

impl fmt::Display for SearchBy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            SearchBy::Name => "name",
            SearchBy::NameDesc => "name-desc",
            SearchBy::Maintainer => "maintainer",
            SearchBy::Depends => "depends",
            SearchBy::MakeDepends => "makedepends",
            SearchBy::OptDepends => "optdepends",
            SearchBy::CheckDepends => "checkdepends",
        })?;
        Ok(())
    }
}

/// Result data for a search.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Search<T: Send + Sync> {
    /// A list of relevant results.
    pub results: Vec<T>,
    /// The number of results in the [`results`] field.
    ///
    /// [`results`]: #structfield.resultcount
    #[serde(rename = "resultcount")]
    pub result_count: u64,
    /// The type of search that was performed.
    #[serde(rename = "type")]
    pub type_: String,
    /// The version of the API in use.
    pub version: u64,
}

/// A result for a search without additional information metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchResult {
    /// A longer description of the package.
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    /// When the package was first submitted.
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: u64,
    /// The ID of the package.
    #[serde(rename = "ID")]
    pub id: u64,
    /// When the package was last modified.
    #[serde(rename = "LastModified")]
    pub last_modified: u64,
    /// The name of the package's maintainer.
    #[serde(default, rename = "Maintainer")]
    pub maintainer: Option<String>,
    /// The name of the package.
    #[serde(rename = "Name")]
    pub name: String,
    /// The number of votes that the package has.
    #[serde(rename = "NumVotes")]
    pub num_votes: u64,
    /// When the package was marked as out-of-date.
    #[serde(default, rename = "OutOfDate")]
    pub out_of_date: Option<u64>,
    /// The name of the base package.
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    /// The ID of the base package.
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: u64,
    /// The relative popularity of the package.
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    /// URL to the package's project home.
    #[serde(default, rename = "URL")]
    pub url: Option<String>,
    /// Path to the package's snapshot tar.
    #[serde(rename = "URLPath")]
    pub url_path: String,
    /// The version of the package.
    #[serde(rename = "Version")]
    pub version: String,
}

/// A result for a search _with_ additional information metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InfoResult {
    /// A list of package names that conflicts with this package.
    #[serde(default, rename = "Conflicts")]
    pub conflicts: Vec<String>,
    /// The packages that this package depends upon.
    #[serde(default, rename = "Depends")]
    pub dependencies: Vec<String>,
    /// A longer description of the package.
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    /// When the package was first submitted.
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: u64,
    /// The ID of the package.
    #[serde(rename = "ID")]
    pub id: u64,
    /// The keywords that the package has been marked with for queryability.
    #[serde(default, rename = "Keywords")]
    pub keywords: Vec<String>,
    /// When the package was last modified.
    #[serde(rename = "LastModified")]
    pub last_modified: u64,
    /// The license(s) that the project is licensed under.
    #[serde(default, rename = "License")]
    pub license: Vec<String>,
    /// The name of the package's maintainer.
    #[serde(default, rename = "Maintainer")]
    pub maintainer: Option<String>,
    /// The list of dependencies to make the package.
    #[serde(default, rename = "MakeDepends")]
    pub make_depends: Vec<String>,
    /// The name of the package.
    #[serde(rename = "Name")]
    pub name: String,
    /// The number of votes that the package has.
    #[serde(rename = "NumVotes")]
    pub num_votes: u64,
    /// The packages that this package optionally depends upon.
    #[serde(default, rename = "OptDepends")]
    pub optional_dependencies: Vec<String>,
    /// When the package was marked as out-of-date.
    #[serde(default, rename = "OutOfDate")]
    pub out_of_date: Option<u64>,
    /// The name of the base package.
    #[serde(rename = "PackageBase")]
    pub package_base: String,
    /// The ID of the base package.
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: u64,
    /// The relative popularity of the package.
    #[serde(rename = "Popularity")]
    pub popularity: f64,
    /// A list of packages this provides for.
    #[serde(default, rename = "Provides")]
    pub provides: Vec<String>,
    /// URL to the package's project home.
    #[serde(default, rename = "URL")]
    pub url: Option<String>,
    /// Path to the package's snapshot tar.
    #[serde(rename = "URLPath")]
    pub url_path: String,
    /// The version of the package.
    #[serde(rename = "Version")]
    pub version: String,
}
