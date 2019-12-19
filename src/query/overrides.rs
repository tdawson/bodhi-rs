//! query for buildroot overrides (or *one* override by NVR)
//!
//! The contents of this module can be used to query a bodhi instance about
//! existing buildroot overrides.
//!
//! The `OverrideNVRQuery` returns exactly one Override, if and only if a
//! Override for the build with this NVR exists - otherwise, it will return an
//! error.
//!
//! The `OverrideQuery` can be used to execute more complex queries, for example
//! filtering overrides by status, sets of overrides for certain packages, or
//! overrides filed by a given list of users.

use std::collections::HashMap;

use serde::Deserialize;

use crate::data::{FedoraRelease, Override};
use crate::error::QueryError;
use crate::query::{Query, SinglePageQuery};
use crate::service::{BodhiService, ServiceError, DEFAULT_PAGE, DEFAULT_ROWS};

/// Use this for querying bodhi for a specific override, by its NVR
/// (Name-Version-Release) string. It will return either an `Ok(Some(Override))`
/// matching the specified NVR, return `Ok(None)` if it doesn't exist, or return
/// an `Err(String)` if another error occurred.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let over_ride = bodhi.query(
///     &bodhi::query::OverrideNVRQuery::new(String::from("wingpanel-2.2.1-1.fc28"))
/// ).unwrap();
/// ```
#[derive(Debug)]
pub struct OverrideNVRQuery {
    nvr: String,
}

#[derive(Debug, Deserialize)]
struct OverridePage {
    r#override: Override,
}

impl OverrideNVRQuery {
    /// This method is the only way to create a new `OverrideNVRQuery` instance.
    pub fn new(nvr: String) -> Self {
        OverrideNVRQuery { nvr }
    }
}

impl SinglePageQuery<Option<Override>> for OverrideNVRQuery {
    fn path(&self) -> String {
        format!("/overrides/{}", self.nvr)
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        None
    }

    fn parse(string: String) -> Result<Option<Override>, QueryError> {
        let override_page: OverridePage = serde_json::from_str(&string)?;
        Ok(Some(override_page.r#override))
    }

    fn missing() -> Result<Option<Override>, QueryError> {
        Ok(None)
    }
}

impl Query<Option<Override>> for OverrideNVRQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Option<Override>, QueryError> {
        <Self as SinglePageQuery<Option<Override>>>::query(self, bodhi)
    }
}

/// Use this for querying bodhi about a set of overrides with the given properties,
/// which can be specified with the builder pattern. Note that some options can be
/// specified multiple times, and overrides will be returned if any criteria match.
/// This is consistent with both the web interface and REST API behavior.
///
/// ```
/// let bodhi = bodhi::BodhiServiceBuilder::default().build().unwrap();
///
/// let overrides = bodhi.query(
///     &bodhi::query::OverrideQuery::new()
///     .releases(bodhi::data::FedoraRelease::F29)
///     .users(String::from("decathorpe"))
/// ).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct OverrideQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    search: Option<String>,
    users: Option<Vec<String>>,
}

impl OverrideQuery {
    /// This method returns a new `OverrideQuery` with *no* filters set.
    pub fn new() -> Self {
        OverrideQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            users: None,
        }
    }

    /// Restrict the returned results to overrides for the given build(s).
    /// Can be specified multiple times.
    pub fn builds(mut self, build: String) -> Self {
        match &mut self.builds {
            Some(builds) => builds.push(build),
            None => self.builds = Some(vec![build]),
        }

        self
    }

    /// Restrict the returned results to (not) expired overrides.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
        self
    }

    /// Restrict search to overrides *like* the given argument (in the SQL sense).
    pub fn like(mut self, like: String) -> Self {
        self.like = Some(like);
        self
    }

    /// Restrict the returned results to overrides for the given package(s).
    /// Can be specified multiple times.
    pub fn packages(mut self, package: String) -> Self {
        match &mut self.packages {
            Some(packages) => packages.push(package),
            None => self.packages = Some(vec![package]),
        }

        self
    }

    /// Restrict the returned results to overrides for the given release(s).
    /// Can be specified multiple times.
    pub fn releases(mut self, release: FedoraRelease) -> Self {
        match &mut self.releases {
            Some(releases) => releases.push(release.into()),
            None => self.releases = Some(vec![release.into()]),
        }

        self
    }

    /// Restrict search to overrides containing the given argument.
    pub fn search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    /// Restrict the returned results to overrides created by the given user(s).
    /// Can be specified multiple times.
    pub fn users(mut self, user: String) -> Self {
        match &mut self.users {
            Some(users) => users.push(user),
            None => self.users = Some(vec![user]),
        }

        self
    }

    /// Query the remote bodhi instance with the given parameters.
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Override>, QueryError> {
        let mut overrides: Vec<Override> = Vec::new();
        let mut page = 1;

        loop {
            let mut query = OverridePageQuery::new();
            query.page = page;

            query.builds = self.builds.clone();
            query.expired = self.expired;
            query.like = self.like.clone();
            query.packages = self.packages.clone();
            query.releases = self.releases.clone();
            query.search = self.search.clone();
            query.users = self.users.clone();

            let result = query.query(bodhi)?;
            overrides.extend(result.r#overrides);

            page += 1;

            if page > result.pages {
                break;
            }
        }

        Ok(overrides)
    }
}

impl Query<Vec<Override>> for OverrideQuery {
    fn query(&self, bodhi: &BodhiService) -> Result<Vec<Override>, QueryError> {
        OverrideQuery::query(self, bodhi)
    }
}

#[derive(Debug, Deserialize)]
struct OverrideListPage {
    overrides: Vec<Override>,
    page: u32,
    pages: u32,
    rows_per_page: u32,
    total: u32,
}

#[derive(Debug)]
struct OverridePageQuery {
    builds: Option<Vec<String>>,
    expired: Option<bool>,
    like: Option<String>,
    packages: Option<Vec<String>>,
    releases: Option<Vec<String>>,
    search: Option<String>,
    users: Option<Vec<String>>,

    page: u32,
    rows_per_page: u32,
}

impl OverridePageQuery {
    fn new() -> Self {
        OverridePageQuery {
            builds: None,
            expired: None,
            like: None,
            packages: None,
            releases: None,
            search: None,
            users: None,
            page: DEFAULT_PAGE,
            rows_per_page: DEFAULT_ROWS,
        }
    }
}

impl SinglePageQuery<OverrideListPage> for OverridePageQuery {
    fn path(&self) -> String {
        String::from("/overrides/")
    }

    fn args(&self) -> Option<HashMap<&str, String>> {
        let mut args: HashMap<&str, String> = HashMap::new();

        if let Some(builds) = &self.builds {
            args.insert("builds", builds.join(","));
        }

        if let Some(expired) = self.expired {
            args.insert("expired", expired.to_string());
        }

        if let Some(like) = &self.like {
            args.insert("like", like.to_owned());
        }

        if let Some(packages) = &self.packages {
            args.insert("packages", packages.join(","));
        }

        if let Some(releases) = &self.releases {
            args.insert("releases", releases.join(","));
        }

        if let Some(search) = &self.search {
            args.insert("search", search.to_owned());
        }

        if let Some(users) = &self.users {
            args.insert("user", users.join(","));
        }

        args.insert("page", format!("{}", self.page));
        args.insert("rows_per_page", format!("{}", self.rows_per_page));

        Some(args)
    }

    fn parse(string: String) -> Result<OverrideListPage, QueryError> {
        let override_page: OverrideListPage = serde_json::from_str(&string)?;
        Ok(override_page)
    }

    fn missing() -> Result<OverrideListPage, QueryError> {
        Err(QueryError::ServiceError {
            error: ServiceError::EmptyResponseError,
        })
    }
}
