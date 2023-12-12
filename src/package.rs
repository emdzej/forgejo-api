use std::fmt::Write;

use super::*;

impl Forgejo {
    pub async fn get_user_packages(
        &self,
        owner: &str,
        query: PackagesQuery,
    ) -> Result<Vec<Package>, ForgejoError> {
        self.get(&query.path(owner)).await
    }

    pub async fn get_package(
        &self,
        owner: &str,
        _type: PackageType,
        name: &str,
        version: &str,
    ) -> Result<Option<Package>, ForgejoError> {
        self.get_opt(&format!(
            "packages/{owner}/{}/{name}/{version}",
            _type.as_str()
        ))
        .await
    }

    pub async fn delete_package(
        &self,
        owner: &str,
        _type: PackageType,
        name: &str,
        version: &str,
    ) -> Result<(), ForgejoError> {
        self.delete(&format!(
            "packages/{owner}/{}/{name}/{version}",
            _type.as_str()
        ))
        .await
    }

    pub async fn get_package_files(
        &self,
        owner: &str,
        _type: PackageType,
        name: &str,
        version: &str,
    ) -> Result<Vec<PackageFile>, ForgejoError> {
        self.get(&format!(
            "packages/{owner}/{}/{name}/{version}",
            _type.as_str()
        ))
        .await
    }
}

#[derive(Default, Debug)]
pub struct PackagesQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub kind: Option<PackageType>,
    pub query: String,
}

impl PackagesQuery {
    fn path(&self, owner: &str) -> String {
        let mut s = String::from("packages/");
        s.push_str(owner);
        s.push('?');
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}"))
                .expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}"))
                .expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(kind) = self.kind {
            s.push_str("type=");
            s.push_str(kind.as_str());
            s.push('&');
        }
        if !self.query.is_empty() {
            s.push_str("q=");
            s.push_str(&self.query);
            s.push('&');
        }
        s
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PackageType {
    Alpine,
    Cargo,
    Chef,
    Composer,
    Conan,
    Conda,
    Container,
    Cran,
    Debian,
    Generic,
    Go,
    Helm,
    Maven,
    Npm,
    Nuget,
    Pub,
    Pypi,
    Rpm,
    RubyGems,
    Swift,
    Vagrant,
}

impl PackageType {
    fn as_str(&self) -> &'static str {
        match self {
            PackageType::Alpine => "alpine",
            PackageType::Cargo => "cargo",
            PackageType::Chef => "chef",
            PackageType::Composer => "composer",
            PackageType::Conan => "conan",
            PackageType::Conda => "conda",
            PackageType::Container => "container",
            PackageType::Cran => "cran",
            PackageType::Debian => "debian",
            PackageType::Generic => "generic",
            PackageType::Go => "go",
            PackageType::Helm => "helm",
            PackageType::Maven => "maven",
            PackageType::Npm => "npm",
            PackageType::Nuget => "nuget",
            PackageType::Pub => "pub",
            PackageType::Pypi => "pypi",
            PackageType::Rpm => "rpm",
            PackageType::RubyGems => "rubygems",
            PackageType::Swift => "swift",
            PackageType::Vagrant => "vagrant",
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Package {
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub creator: User,
    pub id: u64,
    pub name: String,
    pub owner: User,
    pub repository: Option<Repository>,
    pub _type: PackageType,
    pub version: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PackageFile {
    #[serde(rename = "Size")]
    pub size: u64,
    pub id: u64,
    pub md5: String,
    pub name: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}
