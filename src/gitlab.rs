#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///API_Entities_BasicProjectDetails model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_BasicProjectDetails model",
    ///  "type": "object",
    ///  "properties": {
    ///    "avatar_url": {
    ///      "examples": [
    ///        "http://example.com/uploads/project/avatar/3/uploads/avatar.png"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2020-05-07T04:27:17.016Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "custom_attributes": {
    ///      "$ref": "#/components/schemas/API_Entities_CustomAttribute"
    ///    },
    ///    "default_branch": {
    ///      "examples": [
    ///        "main"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "examples": [
    ///        "desc"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "forks_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "http_url_to_repo": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "last_activity_at": {
    ///      "examples": [
    ///        "2013-09-30T13:46:02Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "license": {
    ///      "$ref": "#/components/schemas/API_Entities_LicenseBasic"
    ///    },
    ///    "license_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/LICENCE"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name_with_namespace": {
    ///      "examples": [
    ///        "John Doe / project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "namespace": {
    ///      "$ref": "#/components/schemas/API_Entities_NamespaceBasic"
    ///    },
    ///    "path": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "path_with_namespace": {
    ///      "examples": [
    ///        "namespace1/project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "readme_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/README.md"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repository_storage": {
    ///      "examples": [
    ///        "default"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ssh_url_to_repo": {
    ///      "examples": [
    ///        "git@gitlab.example.com:gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "star_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "tag_list": {
    ///      "examples": [
    ///        "tag"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "topics": {
    ///      "examples": [
    ///        "topic"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesBasicProjectDetails {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub custom_attributes: ::std::option::Option<ApiEntitiesCustomAttribute>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forks_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub http_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_activity_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license: ::std::option::Option<ApiEntitiesLicenseBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub namespace: ::std::option::Option<ApiEntitiesNamespaceBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub readme_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_storage: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssh_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub star_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tag_list: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub topics: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesBasicProjectDetails> for ApiEntitiesBasicProjectDetails {
        fn from(value: &ApiEntitiesBasicProjectDetails) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesBasicProjectDetails {
        fn default() -> Self {
            Self {
                avatar_url: Default::default(),
                created_at: Default::default(),
                custom_attributes: Default::default(),
                default_branch: Default::default(),
                description: Default::default(),
                forks_count: Default::default(),
                http_url_to_repo: Default::default(),
                id: Default::default(),
                last_activity_at: Default::default(),
                license: Default::default(),
                license_url: Default::default(),
                name: Default::default(),
                name_with_namespace: Default::default(),
                namespace: Default::default(),
                path: Default::default(),
                path_with_namespace: Default::default(),
                readme_url: Default::default(),
                repository_storage: Default::default(),
                ssh_url_to_repo: Default::default(),
                star_count: Default::default(),
                tag_list: Default::default(),
                topics: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesBasicProjectDetails {
        pub fn builder() -> builder::ApiEntitiesBasicProjectDetails {
            Default::default()
        }
    }
    ///API_Entities_Ci_PipelineBasic model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_Ci_PipelineBasic model",
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "examples": [
    ///        "2022-10-21T16:49:48+02:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "iid": {
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "project_id": {
    ///      "examples": [
    ///        3
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ref": {
    ///      "examples": [
    ///        "feature-branch"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "sha": {
    ///      "examples": [
    ///        "0ec9e58fdfca6cdd6652c083c9edb53abc0bad52"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "examples": [
    ///        "push"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "examples": [
    ///        "success"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "examples": [
    ///        "2022-10-21T16:49:48+02:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab-org/gitlab-foss/-/pipelines/61"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesCiPipelineBasic {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub iid: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<i64>,
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesCiPipelineBasic> for ApiEntitiesCiPipelineBasic {
        fn from(value: &ApiEntitiesCiPipelineBasic) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesCiPipelineBasic {
        fn default() -> Self {
            Self {
                created_at: Default::default(),
                id: Default::default(),
                iid: Default::default(),
                project_id: Default::default(),
                ref_: Default::default(),
                sha: Default::default(),
                source: Default::default(),
                status: Default::default(),
                updated_at: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesCiPipelineBasic {
        pub fn builder() -> builder::ApiEntitiesCiPipelineBasic {
            Default::default()
        }
    }
    ///API_Entities_Commit model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_Commit model",
    ///  "type": "object",
    ///  "properties": {
    ///    "author_email": {
    ///      "examples": [
    ///        "john@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "author_name": {
    ///      "examples": [
    ///        "John Smith"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "authored_date": {
    ///      "examples": [
    ///        "2012-05-28T04:42:42-07:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "committed_date": {
    ///      "examples": [
    ///        "2012-05-28T04:42:42-07:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "committer_email": {
    ///      "examples": [
    ///        "jack@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "committer_name": {
    ///      "examples": [
    ///        "Jack Smith"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2017-07-26T11:08:53+02:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "extended_trailers": {
    ///      "examples": [
    ///        "{ \"Signed-off-by\": [\"John Doe <johndoe@gitlab.com>\", \"Jane Doe <janedoe@gitlab.com>\"] }"
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "2695effb5807a22ff3d138d593fd856244e155e7"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "examples": [
    ///        "Initial commit"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "parent_ids": {
    ///      "examples": [
    ///        "2a4b78934375d7f53875269ffd4f45fd83a84ebe"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "short_id": {
    ///      "examples": [
    ///        "2695effb"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "examples": [
    ///        "Initial commit"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "trailers": {
    ///      "examples": [
    ///        "{ \"Merged-By\": \"Jane Doe janedoe@gitlab.com\" }"
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/janedoe/gitlab-foss/-/commit/ed899a2f4b50b4370feeea94676502b42383c746"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesCommit {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authored_date: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committed_date: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committer_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committer_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub extended_trailers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub parent_ids: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub short_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub trailers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesCommit> for ApiEntitiesCommit {
        fn from(value: &ApiEntitiesCommit) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesCommit {
        fn default() -> Self {
            Self {
                author_email: Default::default(),
                author_name: Default::default(),
                authored_date: Default::default(),
                committed_date: Default::default(),
                committer_email: Default::default(),
                committer_name: Default::default(),
                created_at: Default::default(),
                extended_trailers: Default::default(),
                id: Default::default(),
                message: Default::default(),
                parent_ids: Default::default(),
                short_id: Default::default(),
                title: Default::default(),
                trailers: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesCommit {
        pub fn builder() -> builder::ApiEntitiesCommit {
            Default::default()
        }
    }
    ///API_Entities_CommitDetail model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_CommitDetail model",
    ///  "type": "object",
    ///  "properties": {
    ///    "author_email": {
    ///      "examples": [
    ///        "john@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "author_name": {
    ///      "examples": [
    ///        "John Smith"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "authored_date": {
    ///      "examples": [
    ///        "2012-05-28T04:42:42-07:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "committed_date": {
    ///      "examples": [
    ///        "2012-05-28T04:42:42-07:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "committer_email": {
    ///      "examples": [
    ///        "jack@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "committer_name": {
    ///      "examples": [
    ///        "Jack Smith"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2017-07-26T11:08:53+02:00"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "extended_trailers": {
    ///      "examples": [
    ///        "{ \"Signed-off-by\": [\"John Doe <johndoe@gitlab.com>\", \"Jane Doe <janedoe@gitlab.com>\"] }"
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "2695effb5807a22ff3d138d593fd856244e155e7"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "last_pipeline": {
    ///      "$ref": "#/components/schemas/API_Entities_Ci_PipelineBasic"
    ///    },
    ///    "message": {
    ///      "examples": [
    ///        "Initial commit"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "parent_ids": {
    ///      "examples": [
    ///        "2a4b78934375d7f53875269ffd4f45fd83a84ebe"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "project_id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "short_id": {
    ///      "examples": [
    ///        "2695effb"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "stats": {
    ///      "$ref": "#/components/schemas/API_Entities_CommitStats"
    ///    },
    ///    "status": {
    ///      "examples": [
    ///        "success"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "examples": [
    ///        "Initial commit"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "trailers": {
    ///      "examples": [
    ///        "{ \"Merged-By\": \"Jane Doe janedoe@gitlab.com\" }"
    ///      ],
    ///      "type": "object"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/janedoe/gitlab-foss/-/commit/ed899a2f4b50b4370feeea94676502b42383c746"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesCommitDetail {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authored_date: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committed_date: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committer_email: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committer_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub extended_trailers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_pipeline: ::std::option::Option<ApiEntitiesCiPipelineBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub parent_ids: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub short_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub stats: ::std::option::Option<ApiEntitiesCommitStats>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub trailers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesCommitDetail> for ApiEntitiesCommitDetail {
        fn from(value: &ApiEntitiesCommitDetail) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesCommitDetail {
        fn default() -> Self {
            Self {
                author_email: Default::default(),
                author_name: Default::default(),
                authored_date: Default::default(),
                committed_date: Default::default(),
                committer_email: Default::default(),
                committer_name: Default::default(),
                created_at: Default::default(),
                extended_trailers: Default::default(),
                id: Default::default(),
                last_pipeline: Default::default(),
                message: Default::default(),
                parent_ids: Default::default(),
                project_id: Default::default(),
                short_id: Default::default(),
                stats: Default::default(),
                status: Default::default(),
                title: Default::default(),
                trailers: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesCommitDetail {
        pub fn builder() -> builder::ApiEntitiesCommitDetail {
            Default::default()
        }
    }
    ///ApiEntitiesCommitStats
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "additions": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "deletions": {
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "total": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesCommitStats {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub additions: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deletions: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&ApiEntitiesCommitStats> for ApiEntitiesCommitStats {
        fn from(value: &ApiEntitiesCommitStats) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesCommitStats {
        fn default() -> Self {
            Self {
                additions: Default::default(),
                deletions: Default::default(),
                total: Default::default(),
            }
        }
    }
    impl ApiEntitiesCommitStats {
        pub fn builder() -> builder::ApiEntitiesCommitStats {
            Default::default()
        }
    }
    ///ApiEntitiesContainerExpirationPolicy
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cadence": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "keep_n": {
    ///      "type": "integer"
    ///    },
    ///    "name_regex": {
    ///      "type": "string"
    ///    },
    ///    "name_regex_keep": {
    ///      "type": "string"
    ///    },
    ///    "next_run_at": {
    ///      "type": "string"
    ///    },
    ///    "older_than": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesContainerExpirationPolicy {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cadence: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keep_n: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_regex: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_regex_keep: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_run_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub older_than: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesContainerExpirationPolicy>
        for ApiEntitiesContainerExpirationPolicy
    {
        fn from(value: &ApiEntitiesContainerExpirationPolicy) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesContainerExpirationPolicy {
        fn default() -> Self {
            Self {
                cadence: Default::default(),
                enabled: Default::default(),
                keep_n: Default::default(),
                name_regex: Default::default(),
                name_regex_keep: Default::default(),
                next_run_at: Default::default(),
                older_than: Default::default(),
            }
        }
    }
    impl ApiEntitiesContainerExpirationPolicy {
        pub fn builder() -> builder::ApiEntitiesContainerExpirationPolicy {
            Default::default()
        }
    }
    ///API_Entities_CustomAttribute model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_CustomAttribute model",
    ///  "type": "object",
    ///  "properties": {
    ///    "key": {
    ///      "examples": [
    ///        "foo"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "examples": [
    ///        "bar"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesCustomAttribute {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesCustomAttribute> for ApiEntitiesCustomAttribute {
        fn from(value: &ApiEntitiesCustomAttribute) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesCustomAttribute {
        fn default() -> Self {
            Self {
                key: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl ApiEntitiesCustomAttribute {
        pub fn builder() -> builder::ApiEntitiesCustomAttribute {
            Default::default()
        }
    }
    ///API_Entities_Event model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_Event model",
    ///  "type": "object",
    ///  "properties": {
    ///    "action_name": {
    ///      "examples": [
    ///        "closed"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "author": {
    ///      "$ref": "#/components/schemas/API_Entities_UserBasic"
    ///    },
    ///    "author_id": {
    ///      "examples": [
    ///        25
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "author_username": {
    ///      "examples": [
    ///        "root"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2017-02-09T10:43:19.667Z"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "imported": {
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "imported_from": {
    ///      "examples": [
    ///        "none"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "note": {
    ///      "$ref": "#/components/schemas/API_Entities_Note"
    ///    },
    ///    "project_id": {
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "push_data": {
    ///      "$ref": "#/components/schemas/API_Entities_PushEventPayload"
    ///    },
    ///    "target_id": {
    ///      "examples": [
    ///        160
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "target_iid": {
    ///      "examples": [
    ///        157
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "target_title": {
    ///      "examples": [
    ///        "Public project search field"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "target_type": {
    ///      "examples": [
    ///        "Issue"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "wiki_page": {
    ///      "$ref": "#/components/schemas/API_Entities_WikiPageBasic"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesEvent {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub action_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<ApiEntitiesUserBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_username: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub imported: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub imported_from: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub note: ::std::option::Option<ApiEntitiesNote>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub push_data: ::std::option::Option<ApiEntitiesPushEventPayload>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_iid: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub target_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_page: ::std::option::Option<ApiEntitiesWikiPageBasic>,
    }
    impl ::std::convert::From<&ApiEntitiesEvent> for ApiEntitiesEvent {
        fn from(value: &ApiEntitiesEvent) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesEvent {
        fn default() -> Self {
            Self {
                action_name: Default::default(),
                author: Default::default(),
                author_id: Default::default(),
                author_username: Default::default(),
                created_at: Default::default(),
                id: Default::default(),
                imported: Default::default(),
                imported_from: Default::default(),
                note: Default::default(),
                project_id: Default::default(),
                push_data: Default::default(),
                target_id: Default::default(),
                target_iid: Default::default(),
                target_title: Default::default(),
                target_type: Default::default(),
                wiki_page: Default::default(),
            }
        }
    }
    impl ApiEntitiesEvent {
        pub fn builder() -> builder::ApiEntitiesEvent {
            Default::default()
        }
    }
    ///ApiEntitiesGroupAccess
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_level": {
    ///      "type": "integer"
    ///    },
    ///    "notification_level": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesGroupAccess {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_level: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub notification_level: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&ApiEntitiesGroupAccess> for ApiEntitiesGroupAccess {
        fn from(value: &ApiEntitiesGroupAccess) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesGroupAccess {
        fn default() -> Self {
            Self {
                access_level: Default::default(),
                notification_level: Default::default(),
            }
        }
    }
    impl ApiEntitiesGroupAccess {
        pub fn builder() -> builder::ApiEntitiesGroupAccess {
            Default::default()
        }
    }
    ///ApiEntitiesLicenseBasic
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "html_url": {
    ///      "examples": [
    ///        "http://choosealicense.com/licenses/gpl-3.0"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "key": {
    ///      "examples": [
    ///        "gpl-3.0"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "GNU General Public License v3.0"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "nickname": {
    ///      "examples": [
    ///        "GNU GPLv3"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "source_url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesLicenseBasic {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub html_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub nickname: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesLicenseBasic> for ApiEntitiesLicenseBasic {
        fn from(value: &ApiEntitiesLicenseBasic) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesLicenseBasic {
        fn default() -> Self {
            Self {
                html_url: Default::default(),
                key: Default::default(),
                name: Default::default(),
                nickname: Default::default(),
                source_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesLicenseBasic {
        pub fn builder() -> builder::ApiEntitiesLicenseBasic {
            Default::default()
        }
    }
    ///ApiEntitiesNamespaceBasic
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avatar_url": {
    ///      "examples": [
    ///        "https://example.com/avatar/12345"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "full_path": {
    ///      "examples": [
    ///        "group/my_project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "kind": {
    ///      "examples": [
    ///        "project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "parent_id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "path": {
    ///      "examples": [
    ///        "my_project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://example.com/group/my_project"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesNamespaceBasic {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub full_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesNamespaceBasic> for ApiEntitiesNamespaceBasic {
        fn from(value: &ApiEntitiesNamespaceBasic) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesNamespaceBasic {
        fn default() -> Self {
            Self {
                avatar_url: Default::default(),
                full_path: Default::default(),
                id: Default::default(),
                kind: Default::default(),
                name: Default::default(),
                parent_id: Default::default(),
                path: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesNamespaceBasic {
        pub fn builder() -> builder::ApiEntitiesNamespaceBasic {
            Default::default()
        }
    }
    ///ApiEntitiesNote
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attachment": {
    ///      "type": "string"
    ///    },
    ///    "author": {
    ///      "$ref": "#/components/schemas/API_Entities_UserBasic"
    ///    },
    ///    "body": {
    ///      "type": "string"
    ///    },
    ///    "commands_changes": {
    ///      "type": "object"
    ///    },
    ///    "commit_id": {
    ///      "type": "string"
    ///    },
    ///    "confidential": {
    ///      "type": "boolean"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "integer"
    ///    },
    ///    "imported": {
    ///      "type": "boolean"
    ///    },
    ///    "imported_from": {
    ///      "examples": [
    ///        "github"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "internal": {
    ///      "type": "boolean"
    ///    },
    ///    "noteable_id": {
    ///      "type": "integer"
    ///    },
    ///    "noteable_iid": {
    ///      "type": "integer"
    ///    },
    ///    "noteable_type": {
    ///      "type": "string"
    ///    },
    ///    "position": {
    ///      "type": "object"
    ///    },
    ///    "project_id": {
    ///      "type": "integer"
    ///    },
    ///    "resolvable": {
    ///      "type": "boolean"
    ///    },
    ///    "resolved": {
    ///      "type": "boolean"
    ///    },
    ///    "resolved_at": {
    ///      "type": "string"
    ///    },
    ///    "resolved_by": {
    ///      "$ref": "#/components/schemas/API_Entities_UserBasic"
    ///    },
    ///    "system": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesNote {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attachment: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<ApiEntitiesUserBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub body: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub commands_changes: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub confidential: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub imported: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub imported_from: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub internal: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub noteable_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub noteable_iid: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub noteable_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub position: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolvable: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolved: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolved_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolved_by: ::std::option::Option<ApiEntitiesUserBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub system: ::std::option::Option<bool>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesNote> for ApiEntitiesNote {
        fn from(value: &ApiEntitiesNote) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesNote {
        fn default() -> Self {
            Self {
                attachment: Default::default(),
                author: Default::default(),
                body: Default::default(),
                commands_changes: Default::default(),
                commit_id: Default::default(),
                confidential: Default::default(),
                created_at: Default::default(),
                id: Default::default(),
                imported: Default::default(),
                imported_from: Default::default(),
                internal: Default::default(),
                noteable_id: Default::default(),
                noteable_iid: Default::default(),
                noteable_type: Default::default(),
                position: Default::default(),
                project_id: Default::default(),
                resolvable: Default::default(),
                resolved: Default::default(),
                resolved_at: Default::default(),
                resolved_by: Default::default(),
                system: Default::default(),
                type_: Default::default(),
                updated_at: Default::default(),
            }
        }
    }
    impl ApiEntitiesNote {
        pub fn builder() -> builder::ApiEntitiesNote {
            Default::default()
        }
    }
    ///API_Entities_Project model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_Project model",
    ///  "type": "object",
    ///  "properties": {
    ///    "_links": {
    ///      "type": "object",
    ///      "properties": {
    ///        "cluster_agents": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/cluster_agents"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "events": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/events"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "issues": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/issues"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "labels": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/labels"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "members": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/members"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "merge_requests": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/merge_requests"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "repo_branches": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/repository/branches"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "self": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4"
    ///          ],
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "allow_merge_on_skipped_pipeline": {
    ///      "type": "boolean"
    ///    },
    ///    "allow_pipeline_trigger_approve_deployment": {
    ///      "type": "boolean"
    ///    },
    ///    "analytics_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "approvals_before_merge": {
    ///      "type": "integer"
    ///    },
    ///    "archived": {
    ///      "type": "boolean"
    ///    },
    ///    "auto_cancel_pending_pipelines": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "auto_devops_deploy_strategy": {
    ///      "examples": [
    ///        "continuous"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "auto_devops_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "autoclose_referenced_issues": {
    ///      "type": "boolean"
    ///    },
    ///    "avatar_url": {
    ///      "examples": [
    ///        "http://example.com/uploads/project/avatar/3/uploads/avatar.png"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "build_git_strategy": {
    ///      "examples": [
    ///        "fetch"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "build_timeout": {
    ///      "examples": [
    ///        3600
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "builds_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "can_create_merge_request_in": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_allow_fork_pipelines_to_run_in_parent_project": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_config_path": {
    ///      "examples": [
    ///        ""
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ci_default_git_depth": {
    ///      "examples": [
    ///        20
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ci_delete_pipelines_in_seconds": {
    ///      "examples": [
    ///        86400
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ci_forward_deployment_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_forward_deployment_rollback_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_id_token_sub_claim_components": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ci_job_token_scope_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_pipeline_variables_minimum_override_role": {
    ///      "type": "string"
    ///    },
    ///    "ci_push_repository_for_job_token_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_restrict_pipeline_cancellation_role": {
    ///      "type": "string"
    ///    },
    ///    "ci_separated_caches": {
    ///      "type": "boolean"
    ///    },
    ///    "compliance_frameworks": {
    ///      "type": "array"
    ///    },
    ///    "container_expiration_policy": {
    ///      "$ref": "#/components/schemas/API_Entities_ContainerExpirationPolicy"
    ///    },
    ///    "container_registry_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "container_registry_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "container_registry_image_prefix": {
    ///      "examples": [
    ///        "registry.gitlab.example.com/gitlab/gitlab-client"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2020-05-07T04:27:17.016Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "creator_id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "custom_attributes": {
    ///      "$ref": "#/components/schemas/API_Entities_CustomAttribute"
    ///    },
    ///    "default_branch": {
    ///      "examples": [
    ///        "main"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "examples": [
    ///        "desc"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description_html": {
    ///      "type": "string"
    ///    },
    ///    "emails_disabled": {
    ///      "type": "boolean"
    ///    },
    ///    "emails_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "empty_repo": {
    ///      "type": "boolean"
    ///    },
    ///    "enforce_auth_checks_on_uploads": {
    ///      "type": "boolean"
    ///    },
    ///    "environments_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "external_authorization_classification_label": {
    ///      "type": "string"
    ///    },
    ///    "feature_flags_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "forked_from_project": {
    ///      "$ref": "#/components/schemas/API_Entities_BasicProjectDetails"
    ///    },
    ///    "forking_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "forks_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "group_runners_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "http_url_to_repo": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "import_error": {
    ///      "examples": [
    ///        "Import error"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_status": {
    ///      "examples": [
    ///        "none"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_type": {
    ///      "examples": [
    ///        "git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_url": {
    ///      "examples": [
    ///        "https://gitlab.com/gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "infrastructure_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issue_branch_template": {
    ///      "examples": [
    ///        "%(title)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "issues_template": {
    ///      "type": "string"
    ///    },
    ///    "jobs_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "keep_latest_artifact": {
    ///      "type": "boolean"
    ///    },
    ///    "last_activity_at": {
    ///      "examples": [
    ///        "2013-09-30T13:46:02Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "lfs_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "license": {
    ///      "$ref": "#/components/schemas/API_Entities_LicenseBasic"
    ///    },
    ///    "license_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/LICENCE"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "marked_for_deletion_at": {
    ///      "type": "string"
    ///    },
    ///    "marked_for_deletion_on": {
    ///      "type": "string"
    ///    },
    ///    "max_artifacts_size": {
    ///      "type": "integer"
    ///    },
    ///    "merge_commit_template": {
    ///      "examples": [
    ///        "%(title)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_method": {
    ///      "examples": [
    ///        "merge"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_pipelines_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_requests_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_template": {
    ///      "type": "string"
    ///    },
    ///    "merge_trains_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_trains_skip_train_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "mirror": {
    ///      "type": "boolean"
    ///    },
    ///    "mirror_overwrites_diverged_branches": {
    ///      "type": "string"
    ///    },
    ///    "mirror_trigger_builds": {
    ///      "type": "string"
    ///    },
    ///    "mirror_user_id": {
    ///      "type": "string"
    ///    },
    ///    "model_experiments_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "model_registry_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "monitor_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "mr_default_target_self": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name_with_namespace": {
    ///      "examples": [
    ///        "John Doe / project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "namespace": {
    ///      "$ref": "#/components/schemas/API_Entities_NamespaceBasic"
    ///    },
    ///    "only_allow_merge_if_all_discussions_are_resolved": {
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_all_status_checks_passed": {
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_pipeline_succeeds": {
    ///      "type": "boolean"
    ///    },
    ///    "only_mirror_protected_branches": {
    ///      "type": "string"
    ///    },
    ///    "open_issues_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "owner": {
    ///      "$ref": "#/components/schemas/API_Entities_UserBasic"
    ///    },
    ///    "packages_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "pages_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "path_with_namespace": {
    ///      "examples": [
    ///        "namespace1/project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "pre_receive_secret_detection_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "prevent_merge_without_jira_issue": {
    ///      "type": "boolean"
    ///    },
    ///    "printing_merge_request_link_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "public_jobs": {
    ///      "type": "boolean"
    ///    },
    ///    "readme_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/README.md"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "releases_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "remove_source_branch_after_merge": {
    ///      "type": "boolean"
    ///    },
    ///    "repository_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repository_object_format": {
    ///      "examples": [
    ///        "sha1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repository_storage": {
    ///      "examples": [
    ///        "default"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "request_access_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "requirements_access_level": {
    ///      "type": "string"
    ///    },
    ///    "requirements_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "resolve_outdated_diff_discussions": {
    ///      "type": "boolean"
    ///    },
    ///    "restrict_user_defined_variables": {
    ///      "type": "boolean"
    ///    },
    ///    "runner_token_expiration_interval": {
    ///      "examples": [
    ///        3600
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "runners_token": {
    ///      "examples": [
    ///        "b8547b1dc37721d05889db52fa2f02"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "secret_push_protection_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "security_and_compliance_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "security_and_compliance_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "service_desk_address": {
    ///      "examples": [
    ///        "address@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "service_desk_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "shared_runners_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "shared_with_groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "snippets_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "snippets_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "squash_commit_template": {
    ///      "examples": [
    ///        "%(source_branch)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "squash_option": {
    ///      "examples": [
    ///        "default_off"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ssh_url_to_repo": {
    ///      "examples": [
    ///        "git@gitlab.example.com:gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "star_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "statistics": {
    ///      "$ref": "#/components/schemas/API_Entities_ProjectStatistics"
    ///    },
    ///    "suggestion_commit_message": {
    ///      "examples": [
    ///        "Suggestion message"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tag_list": {
    ///      "examples": [
    ///        "tag"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "topics": {
    ///      "examples": [
    ///        "topic"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "examples": [
    ///        "2020-05-07T04:27:17.016Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "visibility": {
    ///      "examples": [
    ///        "public"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "warn_about_potentially_unwanted_characters": {
    ///      "type": "boolean"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "wiki_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "wiki_enabled": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProject {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_merge_on_skipped_pipeline: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_pipeline_trigger_approve_deployment: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub analytics_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub approvals_before_merge: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub archived: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_cancel_pending_pipelines: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_deploy_strategy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub autoclose_referenced_issues: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_git_strategy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_timeout: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub builds_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub can_create_merge_request_in: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_allow_fork_pipelines_to_run_in_parent_project: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_config_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_default_git_depth: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_delete_pipelines_in_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_rollback_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ci_id_token_sub_claim_components: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_job_token_scope_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_pipeline_variables_minimum_override_role:
            ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_push_repository_for_job_token_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_restrict_pipeline_cancellation_role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_separated_caches: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub compliance_frameworks: ::std::vec::Vec<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_expiration_policy:
            ::std::option::Option<ApiEntitiesContainerExpirationPolicy>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_image_prefix: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub creator_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub custom_attributes: ::std::option::Option<ApiEntitiesCustomAttribute>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description_html: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_disabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub empty_repo: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enforce_auth_checks_on_uploads: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub environments_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub external_authorization_classification_label:
            ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub feature_flags_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forked_from_project: ::std::option::Option<ApiEntitiesBasicProjectDetails>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forking_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forks_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_runners_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub http_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub infrastructure_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issue_branch_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobs_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keep_latest_artifact: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_activity_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lfs_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license: ::std::option::Option<ApiEntitiesLicenseBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "_links",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub links: ::std::option::Option<ApiEntitiesProjectLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub marked_for_deletion_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub marked_for_deletion_on: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_artifacts_size: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_commit_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_pipelines_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_skip_train_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_overwrites_diverged_branches: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_trigger_builds: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_user_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_experiments_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_registry_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub monitor_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mr_default_target_self: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub namespace: ::std::option::Option<ApiEntitiesNamespaceBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_discussions_are_resolved: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_status_checks_passed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_pipeline_succeeds: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_mirror_protected_branches: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub open_issues_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<ApiEntitiesUserBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub packages_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pages_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pre_receive_secret_detection_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prevent_merge_without_jira_issue: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub printing_merge_request_link_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_jobs: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub readme_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub releases_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub remove_source_branch_after_merge: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_object_format: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_storage: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub request_access_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requirements_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requirements_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolve_outdated_diff_discussions: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub restrict_user_defined_variables: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runner_token_expiration_interval: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runners_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub secret_push_protection_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub security_and_compliance_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub security_and_compliance_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub service_desk_address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub service_desk_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shared_runners_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub shared_with_groups: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_commit_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_option: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssh_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub star_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub statistics: ::std::option::Option<ApiEntitiesProjectStatistics>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suggestion_commit_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tag_list: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub topics: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visibility: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub warn_about_potentially_unwanted_characters: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_enabled: ::std::option::Option<bool>,
    }
    impl ::std::convert::From<&ApiEntitiesProject> for ApiEntitiesProject {
        fn from(value: &ApiEntitiesProject) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProject {
        fn default() -> Self {
            Self {
                allow_merge_on_skipped_pipeline: Default::default(),
                allow_pipeline_trigger_approve_deployment: Default::default(),
                analytics_access_level: Default::default(),
                approvals_before_merge: Default::default(),
                archived: Default::default(),
                auto_cancel_pending_pipelines: Default::default(),
                auto_devops_deploy_strategy: Default::default(),
                auto_devops_enabled: Default::default(),
                autoclose_referenced_issues: Default::default(),
                avatar_url: Default::default(),
                build_git_strategy: Default::default(),
                build_timeout: Default::default(),
                builds_access_level: Default::default(),
                can_create_merge_request_in: Default::default(),
                ci_allow_fork_pipelines_to_run_in_parent_project: Default::default(),
                ci_config_path: Default::default(),
                ci_default_git_depth: Default::default(),
                ci_delete_pipelines_in_seconds: Default::default(),
                ci_forward_deployment_enabled: Default::default(),
                ci_forward_deployment_rollback_allowed: Default::default(),
                ci_id_token_sub_claim_components: Default::default(),
                ci_job_token_scope_enabled: Default::default(),
                ci_pipeline_variables_minimum_override_role: Default::default(),
                ci_push_repository_for_job_token_allowed: Default::default(),
                ci_restrict_pipeline_cancellation_role: Default::default(),
                ci_separated_caches: Default::default(),
                compliance_frameworks: Default::default(),
                container_expiration_policy: Default::default(),
                container_registry_access_level: Default::default(),
                container_registry_enabled: Default::default(),
                container_registry_image_prefix: Default::default(),
                created_at: Default::default(),
                creator_id: Default::default(),
                custom_attributes: Default::default(),
                default_branch: Default::default(),
                description: Default::default(),
                description_html: Default::default(),
                emails_disabled: Default::default(),
                emails_enabled: Default::default(),
                empty_repo: Default::default(),
                enforce_auth_checks_on_uploads: Default::default(),
                environments_access_level: Default::default(),
                external_authorization_classification_label: Default::default(),
                feature_flags_access_level: Default::default(),
                forked_from_project: Default::default(),
                forking_access_level: Default::default(),
                forks_count: Default::default(),
                group_runners_enabled: Default::default(),
                http_url_to_repo: Default::default(),
                id: Default::default(),
                import_error: Default::default(),
                import_status: Default::default(),
                import_type: Default::default(),
                import_url: Default::default(),
                infrastructure_access_level: Default::default(),
                issue_branch_template: Default::default(),
                issues_access_level: Default::default(),
                issues_enabled: Default::default(),
                issues_template: Default::default(),
                jobs_enabled: Default::default(),
                keep_latest_artifact: Default::default(),
                last_activity_at: Default::default(),
                lfs_enabled: Default::default(),
                license: Default::default(),
                license_url: Default::default(),
                links: Default::default(),
                marked_for_deletion_at: Default::default(),
                marked_for_deletion_on: Default::default(),
                max_artifacts_size: Default::default(),
                merge_commit_template: Default::default(),
                merge_method: Default::default(),
                merge_pipelines_enabled: Default::default(),
                merge_requests_access_level: Default::default(),
                merge_requests_enabled: Default::default(),
                merge_requests_template: Default::default(),
                merge_trains_enabled: Default::default(),
                merge_trains_skip_train_allowed: Default::default(),
                mirror: Default::default(),
                mirror_overwrites_diverged_branches: Default::default(),
                mirror_trigger_builds: Default::default(),
                mirror_user_id: Default::default(),
                model_experiments_access_level: Default::default(),
                model_registry_access_level: Default::default(),
                monitor_access_level: Default::default(),
                mr_default_target_self: Default::default(),
                name: Default::default(),
                name_with_namespace: Default::default(),
                namespace: Default::default(),
                only_allow_merge_if_all_discussions_are_resolved: Default::default(),
                only_allow_merge_if_all_status_checks_passed: Default::default(),
                only_allow_merge_if_pipeline_succeeds: Default::default(),
                only_mirror_protected_branches: Default::default(),
                open_issues_count: Default::default(),
                owner: Default::default(),
                packages_enabled: Default::default(),
                pages_access_level: Default::default(),
                path: Default::default(),
                path_with_namespace: Default::default(),
                pre_receive_secret_detection_enabled: Default::default(),
                prevent_merge_without_jira_issue: Default::default(),
                printing_merge_request_link_enabled: Default::default(),
                public_jobs: Default::default(),
                readme_url: Default::default(),
                releases_access_level: Default::default(),
                remove_source_branch_after_merge: Default::default(),
                repository_access_level: Default::default(),
                repository_object_format: Default::default(),
                repository_storage: Default::default(),
                request_access_enabled: Default::default(),
                requirements_access_level: Default::default(),
                requirements_enabled: Default::default(),
                resolve_outdated_diff_discussions: Default::default(),
                restrict_user_defined_variables: Default::default(),
                runner_token_expiration_interval: Default::default(),
                runners_token: Default::default(),
                secret_push_protection_enabled: Default::default(),
                security_and_compliance_access_level: Default::default(),
                security_and_compliance_enabled: Default::default(),
                service_desk_address: Default::default(),
                service_desk_enabled: Default::default(),
                shared_runners_enabled: Default::default(),
                shared_with_groups: Default::default(),
                snippets_access_level: Default::default(),
                snippets_enabled: Default::default(),
                squash_commit_template: Default::default(),
                squash_option: Default::default(),
                ssh_url_to_repo: Default::default(),
                star_count: Default::default(),
                statistics: Default::default(),
                suggestion_commit_message: Default::default(),
                tag_list: Default::default(),
                topics: Default::default(),
                updated_at: Default::default(),
                visibility: Default::default(),
                warn_about_potentially_unwanted_characters: Default::default(),
                web_url: Default::default(),
                wiki_access_level: Default::default(),
                wiki_enabled: Default::default(),
            }
        }
    }
    impl ApiEntitiesProject {
        pub fn builder() -> builder::ApiEntitiesProject {
            Default::default()
        }
    }
    ///ApiEntitiesProjectAccess
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access_level": {
    ///      "type": "string"
    ///    },
    ///    "notification_level": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectAccess {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub notification_level: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectAccess> for ApiEntitiesProjectAccess {
        fn from(value: &ApiEntitiesProjectAccess) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectAccess {
        fn default() -> Self {
            Self {
                access_level: Default::default(),
                notification_level: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectAccess {
        pub fn builder() -> builder::ApiEntitiesProjectAccess {
            Default::default()
        }
    }
    ///ApiEntitiesProjectLinks
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cluster_agents": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/cluster_agents"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/events"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/issues"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "labels": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/labels"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "members": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/members"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_requests": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/merge_requests"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repo_branches": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/repository/branches"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "self": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cluster_agents: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub events: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub members: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repo_branches: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "self",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub self_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectLinks> for ApiEntitiesProjectLinks {
        fn from(value: &ApiEntitiesProjectLinks) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectLinks {
        fn default() -> Self {
            Self {
                cluster_agents: Default::default(),
                events: Default::default(),
                issues: Default::default(),
                labels: Default::default(),
                members: Default::default(),
                merge_requests: Default::default(),
                repo_branches: Default::default(),
                self_: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectLinks {
        pub fn builder() -> builder::ApiEntitiesProjectLinks {
            Default::default()
        }
    }
    ///ApiEntitiesProjectStatistics
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "commit_count": {
    ///      "type": "string"
    ///    },
    ///    "container_registry_size": {
    ///      "type": "string"
    ///    },
    ///    "job_artifacts_size": {
    ///      "type": "string"
    ///    },
    ///    "lfs_objects_size": {
    ///      "type": "string"
    ///    },
    ///    "packages_size": {
    ///      "type": "string"
    ///    },
    ///    "pipeline_artifacts_size": {
    ///      "type": "string"
    ///    },
    ///    "repository_size": {
    ///      "type": "string"
    ///    },
    ///    "snippets_size": {
    ///      "type": "string"
    ///    },
    ///    "storage_size": {
    ///      "type": "string"
    ///    },
    ///    "uploads_size": {
    ///      "type": "string"
    ///    },
    ///    "wiki_size": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectStatistics {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_count: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub job_artifacts_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lfs_objects_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub packages_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pipeline_artifacts_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub storage_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub uploads_size: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_size: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectStatistics> for ApiEntitiesProjectStatistics {
        fn from(value: &ApiEntitiesProjectStatistics) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectStatistics {
        fn default() -> Self {
            Self {
                commit_count: Default::default(),
                container_registry_size: Default::default(),
                job_artifacts_size: Default::default(),
                lfs_objects_size: Default::default(),
                packages_size: Default::default(),
                pipeline_artifacts_size: Default::default(),
                repository_size: Default::default(),
                snippets_size: Default::default(),
                storage_size: Default::default(),
                uploads_size: Default::default(),
                wiki_size: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectStatistics {
        pub fn builder() -> builder::ApiEntitiesProjectStatistics {
            Default::default()
        }
    }
    ///API_Entities_ProjectWithAccess model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_ProjectWithAccess model",
    ///  "type": "object",
    ///  "properties": {
    ///    "_links": {
    ///      "type": "object",
    ///      "properties": {
    ///        "cluster_agents": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/cluster_agents"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "events": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/events"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "issues": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/issues"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "labels": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/labels"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "members": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/members"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "merge_requests": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/merge_requests"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "repo_branches": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4/repository/branches"
    ///          ],
    ///          "type": "string"
    ///        },
    ///        "self": {
    ///          "examples": [
    ///            "https://gitlab.example.com/api/v4/projects/4"
    ///          ],
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "allow_merge_on_skipped_pipeline": {
    ///      "type": "boolean"
    ///    },
    ///    "allow_pipeline_trigger_approve_deployment": {
    ///      "type": "boolean"
    ///    },
    ///    "analytics_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "approvals_before_merge": {
    ///      "type": "integer"
    ///    },
    ///    "archived": {
    ///      "type": "boolean"
    ///    },
    ///    "auto_cancel_pending_pipelines": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "auto_devops_deploy_strategy": {
    ///      "examples": [
    ///        "continuous"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "auto_devops_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "autoclose_referenced_issues": {
    ///      "type": "boolean"
    ///    },
    ///    "avatar_url": {
    ///      "examples": [
    ///        "http://example.com/uploads/project/avatar/3/uploads/avatar.png"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "build_git_strategy": {
    ///      "examples": [
    ///        "fetch"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "build_timeout": {
    ///      "examples": [
    ///        3600
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "builds_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "can_create_merge_request_in": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_allow_fork_pipelines_to_run_in_parent_project": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_config_path": {
    ///      "examples": [
    ///        ""
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ci_default_git_depth": {
    ///      "examples": [
    ///        20
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ci_delete_pipelines_in_seconds": {
    ///      "examples": [
    ///        86400
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ci_forward_deployment_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_forward_deployment_rollback_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_id_token_sub_claim_components": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ci_job_token_scope_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_pipeline_variables_minimum_override_role": {
    ///      "type": "string"
    ///    },
    ///    "ci_push_repository_for_job_token_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "ci_restrict_pipeline_cancellation_role": {
    ///      "type": "string"
    ///    },
    ///    "ci_separated_caches": {
    ///      "type": "boolean"
    ///    },
    ///    "compliance_frameworks": {
    ///      "type": "array"
    ///    },
    ///    "container_expiration_policy": {
    ///      "$ref": "#/components/schemas/API_Entities_ContainerExpirationPolicy"
    ///    },
    ///    "container_registry_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "container_registry_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "container_registry_image_prefix": {
    ///      "examples": [
    ///        "registry.gitlab.example.com/gitlab/gitlab-client"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "examples": [
    ///        "2020-05-07T04:27:17.016Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "creator_id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "custom_attributes": {
    ///      "$ref": "#/components/schemas/API_Entities_CustomAttribute"
    ///    },
    ///    "default_branch": {
    ///      "examples": [
    ///        "main"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "examples": [
    ///        "desc"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description_html": {
    ///      "type": "string"
    ///    },
    ///    "emails_disabled": {
    ///      "type": "boolean"
    ///    },
    ///    "emails_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "empty_repo": {
    ///      "type": "boolean"
    ///    },
    ///    "enforce_auth_checks_on_uploads": {
    ///      "type": "boolean"
    ///    },
    ///    "environments_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "external_authorization_classification_label": {
    ///      "type": "string"
    ///    },
    ///    "feature_flags_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "forked_from_project": {
    ///      "$ref": "#/components/schemas/API_Entities_BasicProjectDetails"
    ///    },
    ///    "forking_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "forks_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "group_runners_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "http_url_to_repo": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "import_error": {
    ///      "examples": [
    ///        "Import error"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_status": {
    ///      "examples": [
    ///        "none"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_type": {
    ///      "examples": [
    ///        "git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "import_url": {
    ///      "examples": [
    ///        "https://gitlab.com/gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "infrastructure_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issue_branch_template": {
    ///      "examples": [
    ///        "%(title)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "issues_template": {
    ///      "type": "string"
    ///    },
    ///    "jobs_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "keep_latest_artifact": {
    ///      "type": "boolean"
    ///    },
    ///    "last_activity_at": {
    ///      "examples": [
    ///        "2013-09-30T13:46:02Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "lfs_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "license": {
    ///      "$ref": "#/components/schemas/API_Entities_LicenseBasic"
    ///    },
    ///    "license_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/LICENCE"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "marked_for_deletion_at": {
    ///      "type": "string"
    ///    },
    ///    "marked_for_deletion_on": {
    ///      "type": "string"
    ///    },
    ///    "max_artifacts_size": {
    ///      "type": "integer"
    ///    },
    ///    "merge_commit_template": {
    ///      "examples": [
    ///        "%(title)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_method": {
    ///      "examples": [
    ///        "merge"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_pipelines_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_requests_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_template": {
    ///      "type": "string"
    ///    },
    ///    "merge_trains_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "merge_trains_skip_train_allowed": {
    ///      "type": "boolean"
    ///    },
    ///    "mirror": {
    ///      "type": "boolean"
    ///    },
    ///    "mirror_overwrites_diverged_branches": {
    ///      "type": "string"
    ///    },
    ///    "mirror_trigger_builds": {
    ///      "type": "string"
    ///    },
    ///    "mirror_user_id": {
    ///      "type": "string"
    ///    },
    ///    "model_experiments_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "model_registry_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "monitor_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "mr_default_target_self": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "name_with_namespace": {
    ///      "examples": [
    ///        "John Doe / project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "namespace": {
    ///      "$ref": "#/components/schemas/API_Entities_NamespaceBasic"
    ///    },
    ///    "only_allow_merge_if_all_discussions_are_resolved": {
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_all_status_checks_passed": {
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_pipeline_succeeds": {
    ///      "type": "boolean"
    ///    },
    ///    "only_mirror_protected_branches": {
    ///      "type": "string"
    ///    },
    ///    "open_issues_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "owner": {
    ///      "$ref": "#/components/schemas/API_Entities_UserBasic"
    ///    },
    ///    "packages_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "pages_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "examples": [
    ///        "project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "path_with_namespace": {
    ///      "examples": [
    ///        "namespace1/project1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "type": "object",
    ///      "properties": {
    ///        "group_access": {
    ///          "$ref": "#/components/schemas/API_Entities_GroupAccess"
    ///        },
    ///        "project_access": {
    ///          "$ref": "#/components/schemas/API_Entities_ProjectAccess"
    ///        }
    ///      }
    ///    },
    ///    "pre_receive_secret_detection_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "prevent_merge_without_jira_issue": {
    ///      "type": "boolean"
    ///    },
    ///    "printing_merge_request_link_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "public_jobs": {
    ///      "type": "boolean"
    ///    },
    ///    "readme_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab/blob/master/README.md"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "releases_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "remove_source_branch_after_merge": {
    ///      "type": "boolean"
    ///    },
    ///    "repository_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repository_object_format": {
    ///      "examples": [
    ///        "sha1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repository_storage": {
    ///      "examples": [
    ///        "default"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "request_access_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "requirements_access_level": {
    ///      "type": "string"
    ///    },
    ///    "requirements_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "resolve_outdated_diff_discussions": {
    ///      "type": "boolean"
    ///    },
    ///    "restrict_user_defined_variables": {
    ///      "type": "boolean"
    ///    },
    ///    "runner_token_expiration_interval": {
    ///      "examples": [
    ///        3600
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "runners_token": {
    ///      "examples": [
    ///        "b8547b1dc37721d05889db52fa2f02"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "secret_push_protection_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "security_and_compliance_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "security_and_compliance_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "service_desk_address": {
    ///      "examples": [
    ///        "address@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "service_desk_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "shared_runners_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "shared_with_groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "snippets_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "snippets_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "squash_commit_template": {
    ///      "examples": [
    ///        "%(source_branch)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "squash_option": {
    ///      "examples": [
    ///        "default_off"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ssh_url_to_repo": {
    ///      "examples": [
    ///        "git@gitlab.example.com:gitlab/gitlab.git"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "star_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "statistics": {
    ///      "$ref": "#/components/schemas/API_Entities_ProjectStatistics"
    ///    },
    ///    "suggestion_commit_message": {
    ///      "examples": [
    ///        "Suggestion message"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tag_list": {
    ///      "examples": [
    ///        "tag"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "topics": {
    ///      "examples": [
    ///        "topic"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "updated_at": {
    ///      "examples": [
    ///        "2020-05-07T04:27:17.016Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "visibility": {
    ///      "examples": [
    ///        "public"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "warn_about_potentially_unwanted_characters": {
    ///      "type": "boolean"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/gitlab/gitlab"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "wiki_access_level": {
    ///      "examples": [
    ///        "enabled"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "wiki_enabled": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectWithAccess {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_merge_on_skipped_pipeline: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_pipeline_trigger_approve_deployment: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub analytics_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub approvals_before_merge: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub archived: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_cancel_pending_pipelines: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_deploy_strategy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub autoclose_referenced_issues: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_git_strategy: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_timeout: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub builds_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub can_create_merge_request_in: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_allow_fork_pipelines_to_run_in_parent_project: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_config_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_default_git_depth: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_delete_pipelines_in_seconds: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_rollback_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ci_id_token_sub_claim_components: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_job_token_scope_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_pipeline_variables_minimum_override_role:
            ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_push_repository_for_job_token_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_restrict_pipeline_cancellation_role: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_separated_caches: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub compliance_frameworks: ::std::vec::Vec<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_expiration_policy:
            ::std::option::Option<ApiEntitiesContainerExpirationPolicy>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_image_prefix: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub created_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub creator_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub custom_attributes: ::std::option::Option<ApiEntitiesCustomAttribute>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_branch: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description_html: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_disabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub empty_repo: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enforce_auth_checks_on_uploads: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub environments_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub external_authorization_classification_label:
            ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub feature_flags_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forked_from_project: ::std::option::Option<ApiEntitiesBasicProjectDetails>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forking_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forks_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_runners_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub http_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub infrastructure_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issue_branch_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobs_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keep_latest_artifact: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_activity_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lfs_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license: ::std::option::Option<ApiEntitiesLicenseBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub license_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "_links",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub links: ::std::option::Option<ApiEntitiesProjectWithAccessLinks>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub marked_for_deletion_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub marked_for_deletion_on: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_artifacts_size: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_commit_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_method: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_pipelines_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_skip_train_allowed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_overwrites_diverged_branches: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_trigger_builds: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_user_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_experiments_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_registry_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub monitor_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mr_default_target_self: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub namespace: ::std::option::Option<ApiEntitiesNamespaceBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_discussions_are_resolved: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_status_checks_passed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_pipeline_succeeds: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_mirror_protected_branches: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub open_issues_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub owner: ::std::option::Option<ApiEntitiesUserBasic>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub packages_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pages_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path_with_namespace: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<ApiEntitiesProjectWithAccessPermissions>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pre_receive_secret_detection_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prevent_merge_without_jira_issue: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub printing_merge_request_link_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_jobs: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub readme_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub releases_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub remove_source_branch_after_merge: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_object_format: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_storage: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub request_access_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requirements_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requirements_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolve_outdated_diff_discussions: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub restrict_user_defined_variables: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runner_token_expiration_interval: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub runners_token: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub secret_push_protection_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub security_and_compliance_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub security_and_compliance_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub service_desk_address: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub service_desk_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shared_runners_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub shared_with_groups: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_enabled: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_commit_template: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_option: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssh_url_to_repo: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub star_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub statistics: ::std::option::Option<ApiEntitiesProjectStatistics>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suggestion_commit_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tag_list: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub topics: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub updated_at: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visibility: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub warn_about_potentially_unwanted_characters: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_access_level: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_enabled: ::std::option::Option<bool>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectWithAccess> for ApiEntitiesProjectWithAccess {
        fn from(value: &ApiEntitiesProjectWithAccess) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectWithAccess {
        fn default() -> Self {
            Self {
                allow_merge_on_skipped_pipeline: Default::default(),
                allow_pipeline_trigger_approve_deployment: Default::default(),
                analytics_access_level: Default::default(),
                approvals_before_merge: Default::default(),
                archived: Default::default(),
                auto_cancel_pending_pipelines: Default::default(),
                auto_devops_deploy_strategy: Default::default(),
                auto_devops_enabled: Default::default(),
                autoclose_referenced_issues: Default::default(),
                avatar_url: Default::default(),
                build_git_strategy: Default::default(),
                build_timeout: Default::default(),
                builds_access_level: Default::default(),
                can_create_merge_request_in: Default::default(),
                ci_allow_fork_pipelines_to_run_in_parent_project: Default::default(),
                ci_config_path: Default::default(),
                ci_default_git_depth: Default::default(),
                ci_delete_pipelines_in_seconds: Default::default(),
                ci_forward_deployment_enabled: Default::default(),
                ci_forward_deployment_rollback_allowed: Default::default(),
                ci_id_token_sub_claim_components: Default::default(),
                ci_job_token_scope_enabled: Default::default(),
                ci_pipeline_variables_minimum_override_role: Default::default(),
                ci_push_repository_for_job_token_allowed: Default::default(),
                ci_restrict_pipeline_cancellation_role: Default::default(),
                ci_separated_caches: Default::default(),
                compliance_frameworks: Default::default(),
                container_expiration_policy: Default::default(),
                container_registry_access_level: Default::default(),
                container_registry_enabled: Default::default(),
                container_registry_image_prefix: Default::default(),
                created_at: Default::default(),
                creator_id: Default::default(),
                custom_attributes: Default::default(),
                default_branch: Default::default(),
                description: Default::default(),
                description_html: Default::default(),
                emails_disabled: Default::default(),
                emails_enabled: Default::default(),
                empty_repo: Default::default(),
                enforce_auth_checks_on_uploads: Default::default(),
                environments_access_level: Default::default(),
                external_authorization_classification_label: Default::default(),
                feature_flags_access_level: Default::default(),
                forked_from_project: Default::default(),
                forking_access_level: Default::default(),
                forks_count: Default::default(),
                group_runners_enabled: Default::default(),
                http_url_to_repo: Default::default(),
                id: Default::default(),
                import_error: Default::default(),
                import_status: Default::default(),
                import_type: Default::default(),
                import_url: Default::default(),
                infrastructure_access_level: Default::default(),
                issue_branch_template: Default::default(),
                issues_access_level: Default::default(),
                issues_enabled: Default::default(),
                issues_template: Default::default(),
                jobs_enabled: Default::default(),
                keep_latest_artifact: Default::default(),
                last_activity_at: Default::default(),
                lfs_enabled: Default::default(),
                license: Default::default(),
                license_url: Default::default(),
                links: Default::default(),
                marked_for_deletion_at: Default::default(),
                marked_for_deletion_on: Default::default(),
                max_artifacts_size: Default::default(),
                merge_commit_template: Default::default(),
                merge_method: Default::default(),
                merge_pipelines_enabled: Default::default(),
                merge_requests_access_level: Default::default(),
                merge_requests_enabled: Default::default(),
                merge_requests_template: Default::default(),
                merge_trains_enabled: Default::default(),
                merge_trains_skip_train_allowed: Default::default(),
                mirror: Default::default(),
                mirror_overwrites_diverged_branches: Default::default(),
                mirror_trigger_builds: Default::default(),
                mirror_user_id: Default::default(),
                model_experiments_access_level: Default::default(),
                model_registry_access_level: Default::default(),
                monitor_access_level: Default::default(),
                mr_default_target_self: Default::default(),
                name: Default::default(),
                name_with_namespace: Default::default(),
                namespace: Default::default(),
                only_allow_merge_if_all_discussions_are_resolved: Default::default(),
                only_allow_merge_if_all_status_checks_passed: Default::default(),
                only_allow_merge_if_pipeline_succeeds: Default::default(),
                only_mirror_protected_branches: Default::default(),
                open_issues_count: Default::default(),
                owner: Default::default(),
                packages_enabled: Default::default(),
                pages_access_level: Default::default(),
                path: Default::default(),
                path_with_namespace: Default::default(),
                permissions: Default::default(),
                pre_receive_secret_detection_enabled: Default::default(),
                prevent_merge_without_jira_issue: Default::default(),
                printing_merge_request_link_enabled: Default::default(),
                public_jobs: Default::default(),
                readme_url: Default::default(),
                releases_access_level: Default::default(),
                remove_source_branch_after_merge: Default::default(),
                repository_access_level: Default::default(),
                repository_object_format: Default::default(),
                repository_storage: Default::default(),
                request_access_enabled: Default::default(),
                requirements_access_level: Default::default(),
                requirements_enabled: Default::default(),
                resolve_outdated_diff_discussions: Default::default(),
                restrict_user_defined_variables: Default::default(),
                runner_token_expiration_interval: Default::default(),
                runners_token: Default::default(),
                secret_push_protection_enabled: Default::default(),
                security_and_compliance_access_level: Default::default(),
                security_and_compliance_enabled: Default::default(),
                service_desk_address: Default::default(),
                service_desk_enabled: Default::default(),
                shared_runners_enabled: Default::default(),
                shared_with_groups: Default::default(),
                snippets_access_level: Default::default(),
                snippets_enabled: Default::default(),
                squash_commit_template: Default::default(),
                squash_option: Default::default(),
                ssh_url_to_repo: Default::default(),
                star_count: Default::default(),
                statistics: Default::default(),
                suggestion_commit_message: Default::default(),
                tag_list: Default::default(),
                topics: Default::default(),
                updated_at: Default::default(),
                visibility: Default::default(),
                warn_about_potentially_unwanted_characters: Default::default(),
                web_url: Default::default(),
                wiki_access_level: Default::default(),
                wiki_enabled: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectWithAccess {
        pub fn builder() -> builder::ApiEntitiesProjectWithAccess {
            Default::default()
        }
    }
    ///ApiEntitiesProjectWithAccessLinks
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cluster_agents": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/cluster_agents"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/events"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/issues"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "labels": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/labels"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "members": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/members"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "merge_requests": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/merge_requests"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "repo_branches": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4/repository/branches"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "self": {
    ///      "examples": [
    ///        "https://gitlab.example.com/api/v4/projects/4"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectWithAccessLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cluster_agents: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub events: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub members: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repo_branches: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "self",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub self_: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectWithAccessLinks>
        for ApiEntitiesProjectWithAccessLinks
    {
        fn from(value: &ApiEntitiesProjectWithAccessLinks) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectWithAccessLinks {
        fn default() -> Self {
            Self {
                cluster_agents: Default::default(),
                events: Default::default(),
                issues: Default::default(),
                labels: Default::default(),
                members: Default::default(),
                merge_requests: Default::default(),
                repo_branches: Default::default(),
                self_: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectWithAccessLinks {
        pub fn builder() -> builder::ApiEntitiesProjectWithAccessLinks {
            Default::default()
        }
    }
    ///ApiEntitiesProjectWithAccessPermissions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "group_access": {
    ///      "$ref": "#/components/schemas/API_Entities_GroupAccess"
    ///    },
    ///    "project_access": {
    ///      "$ref": "#/components/schemas/API_Entities_ProjectAccess"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesProjectWithAccessPermissions {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_access: ::std::option::Option<ApiEntitiesGroupAccess>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub project_access: ::std::option::Option<ApiEntitiesProjectAccess>,
    }
    impl ::std::convert::From<&ApiEntitiesProjectWithAccessPermissions>
        for ApiEntitiesProjectWithAccessPermissions
    {
        fn from(value: &ApiEntitiesProjectWithAccessPermissions) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesProjectWithAccessPermissions {
        fn default() -> Self {
            Self {
                group_access: Default::default(),
                project_access: Default::default(),
            }
        }
    }
    impl ApiEntitiesProjectWithAccessPermissions {
        pub fn builder() -> builder::ApiEntitiesProjectWithAccessPermissions {
            Default::default()
        }
    }
    ///ApiEntitiesPushEventPayload
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "action": {
    ///      "examples": [
    ///        "pushed"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "commit_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "commit_from": {
    ///      "examples": [
    ///        "50d4420237a9de7be1304607147aec22e4a14af7"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "commit_title": {
    ///      "examples": [
    ///        "Add simple search to projects in public area"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "commit_to": {
    ///      "examples": [
    ///        "c5feabde2d8cd023215af4d2ceeb7a64839fc428"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ref": {
    ///      "examples": [
    ///        "master"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ref_count": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "ref_type": {
    ///      "examples": [
    ///        "branch"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesPushEventPayload {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_from: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit_to: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ref_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ref_count: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ref_type: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesPushEventPayload> for ApiEntitiesPushEventPayload {
        fn from(value: &ApiEntitiesPushEventPayload) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesPushEventPayload {
        fn default() -> Self {
            Self {
                action: Default::default(),
                commit_count: Default::default(),
                commit_from: Default::default(),
                commit_title: Default::default(),
                commit_to: Default::default(),
                ref_: Default::default(),
                ref_count: Default::default(),
                ref_type: Default::default(),
            }
        }
    }
    impl ApiEntitiesPushEventPayload {
        pub fn builder() -> builder::ApiEntitiesPushEventPayload {
            Default::default()
        }
    }
    ///API_Entities_UserBasic model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_UserBasic model",
    ///  "type": "object",
    ///  "properties": {
    ///    "avatar_path": {
    ///      "examples": [
    ///        "/user/avatar/28/The-Big-Lebowski-400-400.png"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "avatar_url": {
    ///      "examples": [
    ///        "https://gravatar.com/avatar/1"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "custom_attributes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/API_Entities_CustomAttribute"
    ///      }
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "locked": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "Administrator"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "examples": [
    ///        "active"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "examples": [
    ///        "admin"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "web_url": {
    ///      "examples": [
    ///        "https://gitlab.example.com/root"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesUserBasic {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_path: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub custom_attributes: ::std::vec::Vec<ApiEntitiesCustomAttribute>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locked: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub web_url: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesUserBasic> for ApiEntitiesUserBasic {
        fn from(value: &ApiEntitiesUserBasic) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesUserBasic {
        fn default() -> Self {
            Self {
                avatar_path: Default::default(),
                avatar_url: Default::default(),
                custom_attributes: Default::default(),
                id: Default::default(),
                locked: Default::default(),
                name: Default::default(),
                state: Default::default(),
                username: Default::default(),
                web_url: Default::default(),
            }
        }
    }
    impl ApiEntitiesUserBasic {
        pub fn builder() -> builder::ApiEntitiesUserBasic {
            Default::default()
        }
    }
    ///API_Entities_WikiPageBasic model
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "API_Entities_WikiPageBasic model",
    ///  "type": "object",
    ///  "properties": {
    ///    "format": {
    ///      "examples": [
    ///        "markdown"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "examples": [
    ///        "deploy"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "examples": [
    ///        "deploy"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ApiEntitiesWikiPageBasic {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub slug: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&ApiEntitiesWikiPageBasic> for ApiEntitiesWikiPageBasic {
        fn from(value: &ApiEntitiesWikiPageBasic) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for ApiEntitiesWikiPageBasic {
        fn default() -> Self {
            Self {
                format: Default::default(),
                slug: Default::default(),
                title: Default::default(),
            }
        }
    }
    impl ApiEntitiesWikiPageBasic {
        pub fn builder() -> builder::ApiEntitiesWikiPageBasic {
            Default::default()
        }
    }
    ///GetApiV4EventsSort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "desc",
    ///  "type": "string",
    ///  "enum": [
    ///    "asc",
    ///    "desc"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetApiV4EventsSort {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl ::std::convert::From<&Self> for GetApiV4EventsSort {
        fn from(value: &GetApiV4EventsSort) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetApiV4EventsSort {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Asc => write!(f, "asc"),
                Self::Desc => write!(f, "desc"),
            }
        }
    }
    impl ::std::str::FromStr for GetApiV4EventsSort {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "asc" => Ok(Self::Asc),
                "desc" => Ok(Self::Desc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetApiV4EventsSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetApiV4EventsSort {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetApiV4EventsSort {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for GetApiV4EventsSort {
        fn default() -> Self {
            GetApiV4EventsSort::Desc
        }
    }
    ///GetApiV4EventsTargetType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "issue",
    ///    "milestone",
    ///    "merge_request",
    ///    "note",
    ///    "project",
    ///    "snippet",
    ///    "user",
    ///    "wiki",
    ///    "design"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetApiV4EventsTargetType {
        #[serde(rename = "issue")]
        Issue,
        #[serde(rename = "milestone")]
        Milestone,
        #[serde(rename = "merge_request")]
        MergeRequest,
        #[serde(rename = "note")]
        Note,
        #[serde(rename = "project")]
        Project,
        #[serde(rename = "snippet")]
        Snippet,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "wiki")]
        Wiki,
        #[serde(rename = "design")]
        Design,
    }
    impl ::std::convert::From<&Self> for GetApiV4EventsTargetType {
        fn from(value: &GetApiV4EventsTargetType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetApiV4EventsTargetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Issue => write!(f, "issue"),
                Self::Milestone => write!(f, "milestone"),
                Self::MergeRequest => write!(f, "merge_request"),
                Self::Note => write!(f, "note"),
                Self::Project => write!(f, "project"),
                Self::Snippet => write!(f, "snippet"),
                Self::User => write!(f, "user"),
                Self::Wiki => write!(f, "wiki"),
                Self::Design => write!(f, "design"),
            }
        }
    }
    impl ::std::str::FromStr for GetApiV4EventsTargetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "issue" => Ok(Self::Issue),
                "milestone" => Ok(Self::Milestone),
                "merge_request" => Ok(Self::MergeRequest),
                "note" => Ok(Self::Note),
                "project" => Ok(Self::Project),
                "snippet" => Ok(Self::Snippet),
                "user" => Ok(Self::User),
                "wiki" => Ok(Self::Wiki),
                "design" => Ok(Self::Design),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetApiV4EventsTargetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetApiV4EventsTargetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetApiV4EventsTargetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///GetApiV4ProjectsIdRepositoryCommitsOrder
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "default",
    ///  "type": "string",
    ///  "enum": [
    ///    "default",
    ///    "topo"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetApiV4ProjectsIdRepositoryCommitsOrder {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "topo")]
        Topo,
    }
    impl ::std::convert::From<&Self> for GetApiV4ProjectsIdRepositoryCommitsOrder {
        fn from(value: &GetApiV4ProjectsIdRepositoryCommitsOrder) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GetApiV4ProjectsIdRepositoryCommitsOrder {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Default => write!(f, "default"),
                Self::Topo => write!(f, "topo"),
            }
        }
    }
    impl ::std::str::FromStr for GetApiV4ProjectsIdRepositoryCommitsOrder {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "default" => Ok(Self::Default),
                "topo" => Ok(Self::Topo),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for GetApiV4ProjectsIdRepositoryCommitsOrder {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GetApiV4ProjectsIdRepositoryCommitsOrder {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GetApiV4ProjectsIdRepositoryCommitsOrder {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for GetApiV4ProjectsIdRepositoryCommitsOrder {
        fn default() -> Self {
            GetApiV4ProjectsIdRepositoryCommitsOrder::Default
        }
    }
    ///Commit multiple file changes as one commit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Commit multiple file changes as one commit",
    ///  "type": "object",
    ///  "required": [
    ///    "actions",
    ///    "branch",
    ///    "commit_message"
    ///  ],
    ///  "properties": {
    ///    "actions": {
    ///      "description": "Actions to perform in commit",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "action",
    ///          "content",
    ///          "execute_filemode",
    ///          "file_path",
    ///          "previous_path"
    ///        ],
    ///        "properties": {
    ///          "action": {
    ///            "description": "The action to perform, `create`, `delete`, `move`, `update`, `chmod`",
    ///            "type": "string",
    ///            "enum": [
    ///              "create",
    ///              "update",
    ///              "move",
    ///              "delete",
    ///              "chmod"
    ///            ]
    ///          },
    ///          "content": {
    ///            "description": "File content",
    ///            "examples": [
    ///              "Some file content"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "encoding": {
    ///            "description": "`text` or `base64`",
    ///            "default": "text",
    ///            "type": "string",
    ///            "enum": [
    ///              "text",
    ///              "base64"
    ///            ]
    ///          },
    ///          "execute_filemode": {
    ///            "description": "When `true/false` enables/disables the execute flag on the file.",
    ///            "type": "boolean"
    ///          },
    ///          "file_path": {
    ///            "description": "Full path to the file.",
    ///            "examples": [
    ///              "lib/class.rb"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "last_commit_id": {
    ///            "description": "Last known file commit id",
    ///            "examples": [
    ///              "2695effb5807a22ff3d138d593fd856244e155e7"
    ///            ],
    ///            "type": "string"
    ///          },
    ///          "previous_path": {
    ///            "description": "Original full path to the file being moved.",
    ///            "examples": [
    ///              "lib/class.rb"
    ///            ],
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "author_email": {
    ///      "description": "Author email for commit",
    ///      "examples": [
    ///        "janedoe@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "author_name": {
    ///      "description": "Author name for commit",
    ///      "examples": [
    ///        "Jane Doe"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "branch": {
    ///      "description": "Name of the branch to commit into. To create a new branch, also provide either `start_branch` or `start_sha`, and optionally `start_project`.",
    ///      "examples": [
    ///        "master"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "commit_message": {
    ///      "description": "Commit message",
    ///      "examples": [
    ///        "initial commit"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "force": {
    ///      "description": "When `true` overwrites the target branch with a new commit based on the `start_branch` or `start_sha`",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "start_branch": {
    ///      "description": "Name of the branch to start the new branch from",
    ///      "examples": [
    ///        "staging"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "start_project": {
    ///      "description": "The ID or path of the project to start the new branch from",
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": "integer"
    ///    },
    ///    "start_sha": {
    ///      "description": "SHA of the commit to start the new branch from",
    ///      "examples": [
    ///        "2695effb5807a22ff3d138d593fd856244e155e7"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "stats": {
    ///      "description": "Include commit stats",
    ///      "default": true,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostApiV4ProjectsIdRepositoryCommits {
        ///Actions to perform in commit
        pub actions: ::std::vec::Vec<PostApiV4ProjectsIdRepositoryCommitsActionsItem>,
        ///Author email for commit
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_email: ::std::option::Option<::std::string::String>,
        ///Author name for commit
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub author_name: ::std::option::Option<::std::string::String>,
        ///Name of the branch to commit into. To create a new branch, also provide either `start_branch` or `start_sha`, and optionally `start_project`.
        pub branch: ::std::string::String,
        ///Commit message
        pub commit_message: ::std::string::String,
        ///When `true` overwrites the target branch with a new commit based on the `start_branch` or `start_sha`
        #[serde(default)]
        pub force: bool,
        ///Name of the branch to start the new branch from
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_branch: ::std::option::Option<::std::string::String>,
        ///The ID or path of the project to start the new branch from
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_project: ::std::option::Option<i64>,
        ///SHA of the commit to start the new branch from
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_sha: ::std::option::Option<::std::string::String>,
        ///Include commit stats
        #[serde(default = "defaults::default_bool::<true>")]
        pub stats: bool,
    }
    impl ::std::convert::From<&PostApiV4ProjectsIdRepositoryCommits>
        for PostApiV4ProjectsIdRepositoryCommits
    {
        fn from(value: &PostApiV4ProjectsIdRepositoryCommits) -> Self {
            value.clone()
        }
    }
    impl PostApiV4ProjectsIdRepositoryCommits {
        pub fn builder() -> builder::PostApiV4ProjectsIdRepositoryCommits {
            Default::default()
        }
    }
    ///PostApiV4ProjectsIdRepositoryCommitsActionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "content",
    ///    "execute_filemode",
    ///    "file_path",
    ///    "previous_path"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "description": "The action to perform, `create`, `delete`, `move`, `update`, `chmod`",
    ///      "type": "string",
    ///      "enum": [
    ///        "create",
    ///        "update",
    ///        "move",
    ///        "delete",
    ///        "chmod"
    ///      ]
    ///    },
    ///    "content": {
    ///      "description": "File content",
    ///      "examples": [
    ///        "Some file content"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "encoding": {
    ///      "description": "`text` or `base64`",
    ///      "default": "text",
    ///      "type": "string",
    ///      "enum": [
    ///        "text",
    ///        "base64"
    ///      ]
    ///    },
    ///    "execute_filemode": {
    ///      "description": "When `true/false` enables/disables the execute flag on the file.",
    ///      "type": "boolean"
    ///    },
    ///    "file_path": {
    ///      "description": "Full path to the file.",
    ///      "examples": [
    ///        "lib/class.rb"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "last_commit_id": {
    ///      "description": "Last known file commit id",
    ///      "examples": [
    ///        "2695effb5807a22ff3d138d593fd856244e155e7"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "previous_path": {
    ///      "description": "Original full path to the file being moved.",
    ///      "examples": [
    ///        "lib/class.rb"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PostApiV4ProjectsIdRepositoryCommitsActionsItem {
        ///The action to perform, `create`, `delete`, `move`, `update`, `chmod`
        pub action: PostApiV4ProjectsIdRepositoryCommitsActionsItemAction,
        ///File content
        pub content: ::std::string::String,
        ///`text` or `base64`
        #[serde(
            default = "defaults::post_api_v4_projects_id_repository_commits_actions_item_encoding"
        )]
        pub encoding: PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding,
        ///When `true/false` enables/disables the execute flag on the file.
        pub execute_filemode: bool,
        ///Full path to the file.
        pub file_path: ::std::string::String,
        ///Last known file commit id
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last_commit_id: ::std::option::Option<::std::string::String>,
        ///Original full path to the file being moved.
        pub previous_path: ::std::string::String,
    }
    impl ::std::convert::From<&PostApiV4ProjectsIdRepositoryCommitsActionsItem>
        for PostApiV4ProjectsIdRepositoryCommitsActionsItem
    {
        fn from(value: &PostApiV4ProjectsIdRepositoryCommitsActionsItem) -> Self {
            value.clone()
        }
    }
    impl PostApiV4ProjectsIdRepositoryCommitsActionsItem {
        pub fn builder() -> builder::PostApiV4ProjectsIdRepositoryCommitsActionsItem {
            Default::default()
        }
    }
    ///The action to perform, `create`, `delete`, `move`, `update`, `chmod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The action to perform, `create`, `delete`, `move`, `update`, `chmod`",
    ///  "type": "string",
    ///  "enum": [
    ///    "create",
    ///    "update",
    ///    "move",
    ///    "delete",
    ///    "chmod"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostApiV4ProjectsIdRepositoryCommitsActionsItemAction {
        #[serde(rename = "create")]
        Create,
        #[serde(rename = "update")]
        Update,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "delete")]
        Delete,
        #[serde(rename = "chmod")]
        Chmod,
    }
    impl ::std::convert::From<&Self> for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction {
        fn from(value: &PostApiV4ProjectsIdRepositoryCommitsActionsItemAction) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Create => write!(f, "create"),
                Self::Update => write!(f, "update"),
                Self::Move => write!(f, "move"),
                Self::Delete => write!(f, "delete"),
                Self::Chmod => write!(f, "chmod"),
            }
        }
    }
    impl ::std::str::FromStr for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "create" => Ok(Self::Create),
                "update" => Ok(Self::Update),
                "move" => Ok(Self::Move),
                "delete" => Ok(Self::Delete),
                "chmod" => Ok(Self::Chmod),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PostApiV4ProjectsIdRepositoryCommitsActionsItemAction
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`text` or `base64`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "`text` or `base64`",
    ///  "default": "text",
    ///  "type": "string",
    ///  "enum": [
    ///    "text",
    ///    "base64"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        #[serde(rename = "text")]
        Text,
        #[serde(rename = "base64")]
        Base64,
    }
    impl ::std::convert::From<&Self> for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        fn from(value: &PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Text => write!(f, "text"),
                Self::Base64 => write!(f, "base64"),
            }
        }
    }
    impl ::std::str::FromStr for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "text" => Ok(Self::Text),
                "base64" => Ok(Self::Base64),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::default::Default for PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
        fn default() -> Self {
            PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding::Text
        }
    }
    ///Update an existing project
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Update an existing project",
    ///  "type": "object",
    ///  "properties": {
    ///    "allow_merge_on_skipped_pipeline": {
    ///      "description": "Allow to merge if pipeline is skipped",
    ///      "type": "boolean"
    ///    },
    ///    "allow_pipeline_trigger_approve_deployment": {
    ///      "description": "Allow pipeline triggerer to approve deployments",
    ///      "type": "boolean"
    ///    },
    ///    "analytics_access_level": {
    ///      "description": "Analytics access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "approvals_before_merge": {
    ///      "description": "How many approvers should approve merge request by default",
    ///      "type": "integer"
    ///    },
    ///    "auto_cancel_pending_pipelines": {
    ///      "description": "Auto-cancel pending pipelines",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "auto_devops_deploy_strategy": {
    ///      "description": "Auto Deploy strategy",
    ///      "type": "string",
    ///      "enum": [
    ///        "continuous",
    ///        "manual",
    ///        "timed_incremental"
    ///      ]
    ///    },
    ///    "auto_devops_enabled": {
    ///      "description": "Flag indication if Auto DevOps is enabled",
    ///      "type": "boolean"
    ///    },
    ///    "autoclose_referenced_issues": {
    ///      "description": "Flag indication if referenced issues auto-closing is enabled",
    ///      "type": "boolean"
    ///    },
    ///    "avatar": {
    ///      "description": "Avatar image for project",
    ///      "type": "string"
    ///    },
    ///    "build_git_strategy": {
    ///      "description": "The Git strategy. Defaults to `fetch`",
    ///      "type": "string",
    ///      "enum": [
    ///        "fetch",
    ///        "clone"
    ///      ]
    ///    },
    ///    "build_timeout": {
    ///      "description": "Build timeout",
    ///      "type": "integer"
    ///    },
    ///    "builds_access_level": {
    ///      "description": "Builds access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "ci_allow_fork_pipelines_to_run_in_parent_project": {
    ///      "description": "Allow fork merge request pipelines to run in parent project",
    ///      "type": "boolean"
    ///    },
    ///    "ci_config_path": {
    ///      "description": "The path to CI config file. Defaults to `.gitlab-ci.yml`",
    ///      "type": "string"
    ///    },
    ///    "ci_default_git_depth": {
    ///      "description": "Default number of revisions for shallow cloning",
    ///      "type": "integer"
    ///    },
    ///    "ci_delete_pipelines_in_seconds": {
    ///      "description": "Pipelines older than the configured time are deleted",
    ///      "type": "integer"
    ///    },
    ///    "ci_forward_deployment_enabled": {
    ///      "description": "Prevent older deployment jobs that are still pending",
    ///      "type": "boolean"
    ///    },
    ///    "ci_forward_deployment_rollback_allowed": {
    ///      "description": "Allow job retries for rollback deployments",
    ///      "type": "boolean"
    ///    },
    ///    "ci_id_token_sub_claim_components": {
    ///      "description": "Claims that will be used to build the sub claim in id tokens",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ci_pipeline_variables_minimum_override_role": {
    ///      "description": "Limit ability to override CI/CD variables when triggering a pipeline to only users with at least the set minimum role",
    ///      "type": "string",
    ///      "enum": [
    ///        "no_one_allowed",
    ///        "developer",
    ///        "maintainer",
    ///        "owner"
    ///      ]
    ///    },
    ///    "ci_push_repository_for_job_token_allowed": {
    ///      "description": "Allow pushing to this project's repository by authenticating with a CI/CD job token generated in this project.",
    ///      "type": "boolean"
    ///    },
    ///    "ci_restrict_pipeline_cancellation_role": {
    ///      "description": "Roles allowed to cancel pipelines and jobs.",
    ///      "type": "string"
    ///    },
    ///    "ci_separated_caches": {
    ///      "description": "Enable or disable separated caches based on branch protection.",
    ///      "type": "boolean"
    ///    },
    ///    "container_expiration_policy_attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "cadence": {
    ///          "description": "Container expiration policy cadence for recurring job",
    ///          "type": "string"
    ///        },
    ///        "enabled": {
    ///          "description": "Flag indication if container expiration policy is enabled",
    ///          "type": "boolean"
    ///        },
    ///        "keep_n": {
    ///          "description": "Container expiration policy number of images to keep",
    ///          "type": "integer",
    ///          "format": "int32"
    ///        },
    ///        "name_regex": {
    ///          "description": "Container expiration policy regex for image removal",
    ///          "type": "string"
    ///        },
    ///        "name_regex_keep": {
    ///          "description": "Container expiration policy regex for image retention",
    ///          "type": "string"
    ///        },
    ///        "older_than": {
    ///          "description": "Container expiration policy remove images older than value",
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "container_registry_access_level": {
    ///      "description": "Controls visibility of the container registry. One of `disabled`, `private` or `enabled`. `private` will make the container registry accessible only to project members (reporter role and above). `enabled` will make the container registry accessible to everyone who has access to the project. `disabled` will disable the container registry",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "container_registry_enabled": {
    ///      "description": "Deprecated: Use :container_registry_access_level instead. Flag indication if the container registry is enabled for that project",
    ///      "type": "boolean"
    ///    },
    ///    "default_branch": {
    ///      "description": "The default branch of the project",
    ///      "examples": [
    ///        "main"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "The description of the project",
    ///      "type": "string"
    ///    },
    ///    "emails_disabled": {
    ///      "description": "Deprecated: Use emails_enabled instead.",
    ///      "type": "boolean"
    ///    },
    ///    "emails_enabled": {
    ///      "description": "Enable email notifications",
    ///      "type": "boolean"
    ///    },
    ///    "enforce_auth_checks_on_uploads": {
    ///      "description": "Enforce auth check on uploads",
    ///      "type": "boolean"
    ///    },
    ///    "environments_access_level": {
    ///      "description": "Environments access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "external_authorization_classification_label": {
    ///      "description": "The classification label for the project",
    ///      "type": "string"
    ///    },
    ///    "fallback_approvals_required": {
    ///      "description": "Overall approvals required when no rule is present",
    ///      "type": "integer"
    ///    },
    ///    "feature_flags_access_level": {
    ///      "description": "Feature flags access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "forking_access_level": {
    ///      "description": "Forks access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "group_runners_enabled": {
    ///      "description": "Flag indication if group runners are enabled for that project",
    ///      "type": "boolean"
    ///    },
    ///    "import_url": {
    ///      "description": "URL from which the project is imported",
    ///      "type": "string"
    ///    },
    ///    "infrastructure_access_level": {
    ///      "description": "Infrastructure access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "issue_branch_template": {
    ///      "description": "Template used to create a branch from an issue",
    ///      "type": "string"
    ///    },
    ///    "issues_access_level": {
    ///      "description": "Issues access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "issues_enabled": {
    ///      "description": "Flag indication if the issue tracker is enabled",
    ///      "type": "boolean"
    ///    },
    ///    "issues_template": {
    ///      "description": "Default description for Issues. Description is parsed with GitLab Flavored Markdown.",
    ///      "type": "string"
    ///    },
    ///    "jobs_enabled": {
    ///      "description": "Flag indication if jobs are enabled",
    ///      "type": "boolean"
    ///    },
    ///    "keep_latest_artifact": {
    ///      "description": "Indicates if the latest artifact should be kept for this project.",
    ///      "type": "boolean"
    ///    },
    ///    "lfs_enabled": {
    ///      "description": "Flag indication if Git LFS is enabled for that project",
    ///      "type": "boolean"
    ///    },
    ///    "max_artifacts_size": {
    ///      "description": "Set the maximum file size for each job's artifacts",
    ///      "type": "integer"
    ///    },
    ///    "merge_commit_template": {
    ///      "description": "Template used to create merge commit message",
    ///      "type": "string"
    ///    },
    ///    "merge_method": {
    ///      "description": "The merge method used when merging merge requests",
    ///      "type": "string",
    ///      "enum": [
    ///        "ff",
    ///        "rebase_merge",
    ///        "merge"
    ///      ]
    ///    },
    ///    "merge_pipelines_enabled": {
    ///      "description": "Enable merged results pipelines.",
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_access_level": {
    ///      "description": "Merge requests access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "merge_requests_enabled": {
    ///      "description": "Flag indication if merge requests are enabled",
    ///      "type": "boolean"
    ///    },
    ///    "merge_requests_template": {
    ///      "description": "Default description for merge requests. Description is parsed with GitLab Flavored Markdown.",
    ///      "type": "string"
    ///    },
    ///    "merge_trains_enabled": {
    ///      "description": "Enable merge trains.",
    ///      "type": "boolean"
    ///    },
    ///    "merge_trains_skip_train_allowed": {
    ///      "description": "Allow merge train merge requests to be merged without waiting for pipelines to finish.",
    ///      "type": "boolean"
    ///    },
    ///    "mirror": {
    ///      "description": "[Deprecated] Enables pull mirroring in a project",
    ///      "type": "boolean"
    ///    },
    ///    "mirror_branch_regex": {
    ///      "description": "[Deprecated] Only mirror branches match regex",
    ///      "type": "string"
    ///    },
    ///    "mirror_overwrites_diverged_branches": {
    ///      "description": "[Deprecated] Pull mirror overwrites diverged branches",
    ///      "type": "boolean"
    ///    },
    ///    "mirror_trigger_builds": {
    ///      "description": "[Deprecated] Pull mirroring triggers builds",
    ///      "type": "boolean"
    ///    },
    ///    "mirror_user_id": {
    ///      "description": "[Deprecated] User responsible for all the activity surrounding a pull mirror event. Can only be set by admins",
    ///      "type": "integer"
    ///    },
    ///    "model_experiments_access_level": {
    ///      "description": "Model experiments access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "model_registry_access_level": {
    ///      "description": "Model registry access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "monitor_access_level": {
    ///      "description": "Monitor access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "mr_default_target_self": {
    ///      "description": "Merge requests of this forked project targets itself by default",
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "The name of the project",
    ///      "examples": [
    ///        "project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "only_allow_merge_if_all_discussions_are_resolved": {
    ///      "description": "Only allow to merge if all threads are resolved",
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_all_status_checks_passed": {
    ///      "description": "Blocks merge requests from merging unless all status checks have passed",
    ///      "type": "boolean"
    ///    },
    ///    "only_allow_merge_if_pipeline_succeeds": {
    ///      "description": "Only allow to merge if builds succeed",
    ///      "type": "boolean"
    ///    },
    ///    "only_mirror_protected_branches": {
    ///      "description": "[Deprecated] Only mirror protected branches",
    ///      "type": "boolean"
    ///    },
    ///    "packages_enabled": {
    ///      "description": "Enable project packages feature",
    ///      "type": "boolean"
    ///    },
    ///    "pages_access_level": {
    ///      "description": "Pages access level. One of `disabled`, `private`, `enabled` or `public`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled",
    ///        "public"
    ///      ]
    ///    },
    ///    "path": {
    ///      "description": "The path of the repository",
    ///      "examples": [
    ///        "group/project"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "prevent_merge_without_jira_issue": {
    ///      "description": "Require an associated issue from Jira",
    ///      "type": "boolean"
    ///    },
    ///    "printing_merge_request_link_enabled": {
    ///      "description": "Show link to create/view merge request when pushing from the command line",
    ///      "type": "boolean"
    ///    },
    ///    "public_builds": {
    ///      "description": "Deprecated: Use public_jobs instead.",
    ///      "type": "boolean"
    ///    },
    ///    "public_jobs": {
    ///      "description": "Perform public builds",
    ///      "type": "boolean"
    ///    },
    ///    "releases_access_level": {
    ///      "description": "Releases access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "remove_source_branch_after_merge": {
    ///      "description": "Remove the source branch by default after merge",
    ///      "type": "boolean"
    ///    },
    ///    "repository_access_level": {
    ///      "description": "Repository access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "repository_storage": {
    ///      "description": "Which storage shard the repository is on. Available only to admins",
    ///      "type": "string"
    ///    },
    ///    "request_access_enabled": {
    ///      "description": "Allow users to request member access",
    ///      "type": "boolean"
    ///    },
    ///    "requirements_access_level": {
    ///      "description": "Requirements feature access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "resolve_outdated_diff_discussions": {
    ///      "description": "Automatically resolve merge request diff threads on lines changed with a push",
    ///      "type": "boolean"
    ///    },
    ///    "restrict_user_defined_variables": {
    ///      "description": "Restrict use of user-defined variables when triggering a pipeline",
    ///      "type": "boolean"
    ///    },
    ///    "security_and_compliance_access_level": {
    ///      "description": "Security and compliance access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "service_desk_enabled": {
    ///      "description": "Disable or enable the service desk",
    ///      "type": "boolean"
    ///    },
    ///    "shared_runners_enabled": {
    ///      "description": "Flag indication if shared runners are enabled for that project",
    ///      "type": "boolean"
    ///    },
    ///    "show_default_award_emojis": {
    ///      "description": "Show default award emojis",
    ///      "type": "boolean"
    ///    },
    ///    "show_diff_preview_in_email": {
    ///      "description": "Include the code diff preview in merge request notification emails",
    ///      "type": "boolean"
    ///    },
    ///    "snippets_access_level": {
    ///      "description": "Snippets access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "snippets_enabled": {
    ///      "description": "Flag indication if snippets are enabled",
    ///      "type": "boolean"
    ///    },
    ///    "squash_commit_template": {
    ///      "description": "Template used to create squash commit message",
    ///      "type": "string"
    ///    },
    ///    "squash_option": {
    ///      "description": "Squash default for project. One of `never`, `always`, `default_on`, or `default_off`.",
    ///      "type": "string",
    ///      "enum": [
    ///        "never",
    ///        "always",
    ///        "default_on",
    ///        "default_off"
    ///      ]
    ///    },
    ///    "suggestion_commit_message": {
    ///      "description": "The commit message used to apply merge request suggestions",
    ///      "type": "string"
    ///    },
    ///    "tag_list": {
    ///      "description": "Deprecated: Use :topics instead",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "topics": {
    ///      "description": "The list of topics for a project",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "visibility": {
    ///      "description": "The visibility of the project.",
    ///      "type": "string",
    ///      "enum": [
    ///        "private",
    ///        "internal",
    ///        "public"
    ///      ]
    ///    },
    ///    "warn_about_potentially_unwanted_characters": {
    ///      "description": "Warn about potentially unwanted characters",
    ///      "type": "boolean"
    ///    },
    ///    "wiki_access_level": {
    ///      "description": "Wiki access level. One of `disabled`, `private` or `enabled`",
    ///      "type": "string",
    ///      "enum": [
    ///        "disabled",
    ///        "private",
    ///        "enabled"
    ///      ]
    ///    },
    ///    "wiki_enabled": {
    ///      "description": "Flag indication if the wiki is enabled",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PutApiV4ProjectsId {
        ///Allow to merge if pipeline is skipped
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_merge_on_skipped_pipeline: ::std::option::Option<bool>,
        ///Allow pipeline triggerer to approve deployments
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub allow_pipeline_trigger_approve_deployment: ::std::option::Option<bool>,
        ///Analytics access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub analytics_access_level: ::std::option::Option<PutApiV4ProjectsIdAnalyticsAccessLevel>,
        ///How many approvers should approve merge request by default
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub approvals_before_merge: ::std::option::Option<i64>,
        ///Auto-cancel pending pipelines
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_cancel_pending_pipelines:
            ::std::option::Option<PutApiV4ProjectsIdAutoCancelPendingPipelines>,
        ///Auto Deploy strategy
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_deploy_strategy:
            ::std::option::Option<PutApiV4ProjectsIdAutoDevopsDeployStrategy>,
        ///Flag indication if Auto DevOps is enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub auto_devops_enabled: ::std::option::Option<bool>,
        ///Flag indication if referenced issues auto-closing is enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub autoclose_referenced_issues: ::std::option::Option<bool>,
        ///Avatar image for project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub avatar: ::std::option::Option<::std::string::String>,
        ///The Git strategy. Defaults to `fetch`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_git_strategy: ::std::option::Option<PutApiV4ProjectsIdBuildGitStrategy>,
        ///Build timeout
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub build_timeout: ::std::option::Option<i64>,
        ///Builds access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub builds_access_level: ::std::option::Option<PutApiV4ProjectsIdBuildsAccessLevel>,
        ///Allow fork merge request pipelines to run in parent project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_allow_fork_pipelines_to_run_in_parent_project: ::std::option::Option<bool>,
        ///The path to CI config file. Defaults to `.gitlab-ci.yml`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_config_path: ::std::option::Option<::std::string::String>,
        ///Default number of revisions for shallow cloning
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_default_git_depth: ::std::option::Option<i64>,
        ///Pipelines older than the configured time are deleted
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_delete_pipelines_in_seconds: ::std::option::Option<i64>,
        ///Prevent older deployment jobs that are still pending
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_enabled: ::std::option::Option<bool>,
        ///Allow job retries for rollback deployments
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_forward_deployment_rollback_allowed: ::std::option::Option<bool>,
        ///Claims that will be used to build the sub claim in id tokens
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ci_id_token_sub_claim_components: ::std::vec::Vec<::std::string::String>,
        ///Limit ability to override CI/CD variables when triggering a pipeline to only users with at least the set minimum role
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_pipeline_variables_minimum_override_role:
            ::std::option::Option<PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole>,
        ///Allow pushing to this project's repository by authenticating with a CI/CD job token generated in this project.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_push_repository_for_job_token_allowed: ::std::option::Option<bool>,
        ///Roles allowed to cancel pipelines and jobs.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_restrict_pipeline_cancellation_role: ::std::option::Option<::std::string::String>,
        ///Enable or disable separated caches based on branch protection.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ci_separated_caches: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_expiration_policy_attributes:
            ::std::option::Option<PutApiV4ProjectsIdContainerExpirationPolicyAttributes>,
        ///Controls visibility of the container registry. One of `disabled`, `private` or `enabled`. `private` will make the container registry accessible only to project members (reporter role and above). `enabled` will make the container registry accessible to everyone who has access to the project. `disabled` will disable the container registry
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_access_level:
            ::std::option::Option<PutApiV4ProjectsIdContainerRegistryAccessLevel>,
        ///Deprecated: Use :container_registry_access_level instead. Flag indication if the container registry is enabled for that project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub container_registry_enabled: ::std::option::Option<bool>,
        ///The default branch of the project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub default_branch: ::std::option::Option<::std::string::String>,
        ///The description of the project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Deprecated: Use emails_enabled instead.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_disabled: ::std::option::Option<bool>,
        ///Enable email notifications
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub emails_enabled: ::std::option::Option<bool>,
        ///Enforce auth check on uploads
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enforce_auth_checks_on_uploads: ::std::option::Option<bool>,
        ///Environments access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub environments_access_level:
            ::std::option::Option<PutApiV4ProjectsIdEnvironmentsAccessLevel>,
        ///The classification label for the project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub external_authorization_classification_label:
            ::std::option::Option<::std::string::String>,
        ///Overall approvals required when no rule is present
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fallback_approvals_required: ::std::option::Option<i64>,
        ///Feature flags access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub feature_flags_access_level:
            ::std::option::Option<PutApiV4ProjectsIdFeatureFlagsAccessLevel>,
        ///Forks access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forking_access_level: ::std::option::Option<PutApiV4ProjectsIdForkingAccessLevel>,
        ///Flag indication if group runners are enabled for that project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_runners_enabled: ::std::option::Option<bool>,
        ///URL from which the project is imported
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub import_url: ::std::option::Option<::std::string::String>,
        ///Infrastructure access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub infrastructure_access_level:
            ::std::option::Option<PutApiV4ProjectsIdInfrastructureAccessLevel>,
        ///Template used to create a branch from an issue
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issue_branch_template: ::std::option::Option<::std::string::String>,
        ///Issues access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_access_level: ::std::option::Option<PutApiV4ProjectsIdIssuesAccessLevel>,
        ///Flag indication if the issue tracker is enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_enabled: ::std::option::Option<bool>,
        ///Default description for Issues. Description is parsed with GitLab Flavored Markdown.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub issues_template: ::std::option::Option<::std::string::String>,
        ///Flag indication if jobs are enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobs_enabled: ::std::option::Option<bool>,
        ///Indicates if the latest artifact should be kept for this project.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keep_latest_artifact: ::std::option::Option<bool>,
        ///Flag indication if Git LFS is enabled for that project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lfs_enabled: ::std::option::Option<bool>,
        ///Set the maximum file size for each job's artifacts
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_artifacts_size: ::std::option::Option<i64>,
        ///Template used to create merge commit message
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_commit_template: ::std::option::Option<::std::string::String>,
        ///The merge method used when merging merge requests
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_method: ::std::option::Option<PutApiV4ProjectsIdMergeMethod>,
        ///Enable merged results pipelines.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_pipelines_enabled: ::std::option::Option<bool>,
        ///Merge requests access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_access_level:
            ::std::option::Option<PutApiV4ProjectsIdMergeRequestsAccessLevel>,
        ///Flag indication if merge requests are enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_enabled: ::std::option::Option<bool>,
        ///Default description for merge requests. Description is parsed with GitLab Flavored Markdown.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_requests_template: ::std::option::Option<::std::string::String>,
        ///Enable merge trains.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_enabled: ::std::option::Option<bool>,
        ///Allow merge train merge requests to be merged without waiting for pipelines to finish.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub merge_trains_skip_train_allowed: ::std::option::Option<bool>,
        ///[Deprecated] Enables pull mirroring in a project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror: ::std::option::Option<bool>,
        ///[Deprecated] Only mirror branches match regex
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_branch_regex: ::std::option::Option<::std::string::String>,
        ///[Deprecated] Pull mirror overwrites diverged branches
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_overwrites_diverged_branches: ::std::option::Option<bool>,
        ///[Deprecated] Pull mirroring triggers builds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_trigger_builds: ::std::option::Option<bool>,
        ///[Deprecated] User responsible for all the activity surrounding a pull mirror event. Can only be set by admins
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mirror_user_id: ::std::option::Option<i64>,
        ///Model experiments access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_experiments_access_level:
            ::std::option::Option<PutApiV4ProjectsIdModelExperimentsAccessLevel>,
        ///Model registry access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub model_registry_access_level:
            ::std::option::Option<PutApiV4ProjectsIdModelRegistryAccessLevel>,
        ///Monitor access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub monitor_access_level: ::std::option::Option<PutApiV4ProjectsIdMonitorAccessLevel>,
        ///Merge requests of this forked project targets itself by default
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mr_default_target_self: ::std::option::Option<bool>,
        ///The name of the project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Only allow to merge if all threads are resolved
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_discussions_are_resolved: ::std::option::Option<bool>,
        ///Blocks merge requests from merging unless all status checks have passed
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_all_status_checks_passed: ::std::option::Option<bool>,
        ///Only allow to merge if builds succeed
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_allow_merge_if_pipeline_succeeds: ::std::option::Option<bool>,
        ///[Deprecated] Only mirror protected branches
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub only_mirror_protected_branches: ::std::option::Option<bool>,
        ///Enable project packages feature
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub packages_enabled: ::std::option::Option<bool>,
        ///Pages access level. One of `disabled`, `private`, `enabled` or `public`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pages_access_level: ::std::option::Option<PutApiV4ProjectsIdPagesAccessLevel>,
        ///The path of the repository
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        ///Require an associated issue from Jira
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prevent_merge_without_jira_issue: ::std::option::Option<bool>,
        ///Show link to create/view merge request when pushing from the command line
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub printing_merge_request_link_enabled: ::std::option::Option<bool>,
        ///Deprecated: Use public_jobs instead.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_builds: ::std::option::Option<bool>,
        ///Perform public builds
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_jobs: ::std::option::Option<bool>,
        ///Releases access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub releases_access_level: ::std::option::Option<PutApiV4ProjectsIdReleasesAccessLevel>,
        ///Remove the source branch by default after merge
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub remove_source_branch_after_merge: ::std::option::Option<bool>,
        ///Repository access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_access_level: ::std::option::Option<PutApiV4ProjectsIdRepositoryAccessLevel>,
        ///Which storage shard the repository is on. Available only to admins
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub repository_storage: ::std::option::Option<::std::string::String>,
        ///Allow users to request member access
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub request_access_enabled: ::std::option::Option<bool>,
        ///Requirements feature access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requirements_access_level:
            ::std::option::Option<PutApiV4ProjectsIdRequirementsAccessLevel>,
        ///Automatically resolve merge request diff threads on lines changed with a push
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolve_outdated_diff_discussions: ::std::option::Option<bool>,
        ///Restrict use of user-defined variables when triggering a pipeline
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub restrict_user_defined_variables: ::std::option::Option<bool>,
        ///Security and compliance access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub security_and_compliance_access_level:
            ::std::option::Option<PutApiV4ProjectsIdSecurityAndComplianceAccessLevel>,
        ///Disable or enable the service desk
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub service_desk_enabled: ::std::option::Option<bool>,
        ///Flag indication if shared runners are enabled for that project
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub shared_runners_enabled: ::std::option::Option<bool>,
        ///Show default award emojis
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub show_default_award_emojis: ::std::option::Option<bool>,
        ///Include the code diff preview in merge request notification emails
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub show_diff_preview_in_email: ::std::option::Option<bool>,
        ///Snippets access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_access_level: ::std::option::Option<PutApiV4ProjectsIdSnippetsAccessLevel>,
        ///Flag indication if snippets are enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snippets_enabled: ::std::option::Option<bool>,
        ///Template used to create squash commit message
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_commit_template: ::std::option::Option<::std::string::String>,
        ///Squash default for project. One of `never`, `always`, `default_on`, or `default_off`.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub squash_option: ::std::option::Option<PutApiV4ProjectsIdSquashOption>,
        ///The commit message used to apply merge request suggestions
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suggestion_commit_message: ::std::option::Option<::std::string::String>,
        ///Deprecated: Use :topics instead
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tag_list: ::std::vec::Vec<::std::string::String>,
        ///The list of topics for a project
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub topics: ::std::vec::Vec<::std::string::String>,
        ///The visibility of the project.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub visibility: ::std::option::Option<PutApiV4ProjectsIdVisibility>,
        ///Warn about potentially unwanted characters
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub warn_about_potentially_unwanted_characters: ::std::option::Option<bool>,
        ///Wiki access level. One of `disabled`, `private` or `enabled`
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_access_level: ::std::option::Option<PutApiV4ProjectsIdWikiAccessLevel>,
        ///Flag indication if the wiki is enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub wiki_enabled: ::std::option::Option<bool>,
    }
    impl ::std::convert::From<&PutApiV4ProjectsId> for PutApiV4ProjectsId {
        fn from(value: &PutApiV4ProjectsId) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PutApiV4ProjectsId {
        fn default() -> Self {
            Self {
                allow_merge_on_skipped_pipeline: Default::default(),
                allow_pipeline_trigger_approve_deployment: Default::default(),
                analytics_access_level: Default::default(),
                approvals_before_merge: Default::default(),
                auto_cancel_pending_pipelines: Default::default(),
                auto_devops_deploy_strategy: Default::default(),
                auto_devops_enabled: Default::default(),
                autoclose_referenced_issues: Default::default(),
                avatar: Default::default(),
                build_git_strategy: Default::default(),
                build_timeout: Default::default(),
                builds_access_level: Default::default(),
                ci_allow_fork_pipelines_to_run_in_parent_project: Default::default(),
                ci_config_path: Default::default(),
                ci_default_git_depth: Default::default(),
                ci_delete_pipelines_in_seconds: Default::default(),
                ci_forward_deployment_enabled: Default::default(),
                ci_forward_deployment_rollback_allowed: Default::default(),
                ci_id_token_sub_claim_components: Default::default(),
                ci_pipeline_variables_minimum_override_role: Default::default(),
                ci_push_repository_for_job_token_allowed: Default::default(),
                ci_restrict_pipeline_cancellation_role: Default::default(),
                ci_separated_caches: Default::default(),
                container_expiration_policy_attributes: Default::default(),
                container_registry_access_level: Default::default(),
                container_registry_enabled: Default::default(),
                default_branch: Default::default(),
                description: Default::default(),
                emails_disabled: Default::default(),
                emails_enabled: Default::default(),
                enforce_auth_checks_on_uploads: Default::default(),
                environments_access_level: Default::default(),
                external_authorization_classification_label: Default::default(),
                fallback_approvals_required: Default::default(),
                feature_flags_access_level: Default::default(),
                forking_access_level: Default::default(),
                group_runners_enabled: Default::default(),
                import_url: Default::default(),
                infrastructure_access_level: Default::default(),
                issue_branch_template: Default::default(),
                issues_access_level: Default::default(),
                issues_enabled: Default::default(),
                issues_template: Default::default(),
                jobs_enabled: Default::default(),
                keep_latest_artifact: Default::default(),
                lfs_enabled: Default::default(),
                max_artifacts_size: Default::default(),
                merge_commit_template: Default::default(),
                merge_method: Default::default(),
                merge_pipelines_enabled: Default::default(),
                merge_requests_access_level: Default::default(),
                merge_requests_enabled: Default::default(),
                merge_requests_template: Default::default(),
                merge_trains_enabled: Default::default(),
                merge_trains_skip_train_allowed: Default::default(),
                mirror: Default::default(),
                mirror_branch_regex: Default::default(),
                mirror_overwrites_diverged_branches: Default::default(),
                mirror_trigger_builds: Default::default(),
                mirror_user_id: Default::default(),
                model_experiments_access_level: Default::default(),
                model_registry_access_level: Default::default(),
                monitor_access_level: Default::default(),
                mr_default_target_self: Default::default(),
                name: Default::default(),
                only_allow_merge_if_all_discussions_are_resolved: Default::default(),
                only_allow_merge_if_all_status_checks_passed: Default::default(),
                only_allow_merge_if_pipeline_succeeds: Default::default(),
                only_mirror_protected_branches: Default::default(),
                packages_enabled: Default::default(),
                pages_access_level: Default::default(),
                path: Default::default(),
                prevent_merge_without_jira_issue: Default::default(),
                printing_merge_request_link_enabled: Default::default(),
                public_builds: Default::default(),
                public_jobs: Default::default(),
                releases_access_level: Default::default(),
                remove_source_branch_after_merge: Default::default(),
                repository_access_level: Default::default(),
                repository_storage: Default::default(),
                request_access_enabled: Default::default(),
                requirements_access_level: Default::default(),
                resolve_outdated_diff_discussions: Default::default(),
                restrict_user_defined_variables: Default::default(),
                security_and_compliance_access_level: Default::default(),
                service_desk_enabled: Default::default(),
                shared_runners_enabled: Default::default(),
                show_default_award_emojis: Default::default(),
                show_diff_preview_in_email: Default::default(),
                snippets_access_level: Default::default(),
                snippets_enabled: Default::default(),
                squash_commit_template: Default::default(),
                squash_option: Default::default(),
                suggestion_commit_message: Default::default(),
                tag_list: Default::default(),
                topics: Default::default(),
                visibility: Default::default(),
                warn_about_potentially_unwanted_characters: Default::default(),
                wiki_access_level: Default::default(),
                wiki_enabled: Default::default(),
            }
        }
    }
    impl PutApiV4ProjectsId {
        pub fn builder() -> builder::PutApiV4ProjectsId {
            Default::default()
        }
    }
    ///Analytics access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Analytics access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdAnalyticsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdAnalyticsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdAnalyticsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdAnalyticsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdAnalyticsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdAnalyticsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdAnalyticsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdAnalyticsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Auto-cancel pending pipelines
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Auto-cancel pending pipelines",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdAutoCancelPendingPipelines {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdAutoCancelPendingPipelines {
        fn from(value: &PutApiV4ProjectsIdAutoCancelPendingPipelines) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdAutoCancelPendingPipelines {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdAutoCancelPendingPipelines {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdAutoCancelPendingPipelines {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdAutoCancelPendingPipelines
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdAutoCancelPendingPipelines
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Auto Deploy strategy
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Auto Deploy strategy",
    ///  "type": "string",
    ///  "enum": [
    ///    "continuous",
    ///    "manual",
    ///    "timed_incremental"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        #[serde(rename = "continuous")]
        Continuous,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "timed_incremental")]
        TimedIncremental,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        fn from(value: &PutApiV4ProjectsIdAutoDevopsDeployStrategy) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Continuous => write!(f, "continuous"),
                Self::Manual => write!(f, "manual"),
                Self::TimedIncremental => write!(f, "timed_incremental"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "continuous" => Ok(Self::Continuous),
                "manual" => Ok(Self::Manual),
                "timed_incremental" => Ok(Self::TimedIncremental),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdAutoDevopsDeployStrategy
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdAutoDevopsDeployStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The Git strategy. Defaults to `fetch`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The Git strategy. Defaults to `fetch`",
    ///  "type": "string",
    ///  "enum": [
    ///    "fetch",
    ///    "clone"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdBuildGitStrategy {
        #[serde(rename = "fetch")]
        Fetch,
        #[serde(rename = "clone")]
        Clone,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdBuildGitStrategy {
        fn from(value: &PutApiV4ProjectsIdBuildGitStrategy) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdBuildGitStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fetch => write!(f, "fetch"),
                Self::Clone => write!(f, "clone"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdBuildGitStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "fetch" => Ok(Self::Fetch),
                "clone" => Ok(Self::Clone),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdBuildGitStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdBuildGitStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdBuildGitStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Builds access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Builds access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdBuildsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdBuildsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdBuildsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdBuildsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdBuildsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdBuildsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdBuildsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdBuildsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Limit ability to override CI/CD variables when triggering a pipeline to only users with at least the set minimum role
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Limit ability to override CI/CD variables when triggering a pipeline to only users with at least the set minimum role",
    ///  "type": "string",
    ///  "enum": [
    ///    "no_one_allowed",
    ///    "developer",
    ///    "maintainer",
    ///    "owner"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole {
        #[serde(rename = "no_one_allowed")]
        NoOneAllowed,
        #[serde(rename = "developer")]
        Developer,
        #[serde(rename = "maintainer")]
        Maintainer,
        #[serde(rename = "owner")]
        Owner,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole {
        fn from(value: &PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NoOneAllowed => write!(f, "no_one_allowed"),
                Self::Developer => write!(f, "developer"),
                Self::Maintainer => write!(f, "maintainer"),
                Self::Owner => write!(f, "owner"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "no_one_allowed" => Ok(Self::NoOneAllowed),
                "developer" => Ok(Self::Developer),
                "maintainer" => Ok(Self::Maintainer),
                "owner" => Ok(Self::Owner),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///PutApiV4ProjectsIdContainerExpirationPolicyAttributes
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cadence": {
    ///      "description": "Container expiration policy cadence for recurring job",
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "description": "Flag indication if container expiration policy is enabled",
    ///      "type": "boolean"
    ///    },
    ///    "keep_n": {
    ///      "description": "Container expiration policy number of images to keep",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "name_regex": {
    ///      "description": "Container expiration policy regex for image removal",
    ///      "type": "string"
    ///    },
    ///    "name_regex_keep": {
    ///      "description": "Container expiration policy regex for image retention",
    ///      "type": "string"
    ///    },
    ///    "older_than": {
    ///      "description": "Container expiration policy remove images older than value",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
        ///Container expiration policy cadence for recurring job
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cadence: ::std::option::Option<::std::string::String>,
        ///Flag indication if container expiration policy is enabled
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub enabled: ::std::option::Option<bool>,
        ///Container expiration policy number of images to keep
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keep_n: ::std::option::Option<i32>,
        ///Container expiration policy regex for image removal
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_regex: ::std::option::Option<::std::string::String>,
        ///Container expiration policy regex for image retention
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name_regex_keep: ::std::option::Option<::std::string::String>,
        ///Container expiration policy remove images older than value
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub older_than: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&PutApiV4ProjectsIdContainerExpirationPolicyAttributes>
        for PutApiV4ProjectsIdContainerExpirationPolicyAttributes
    {
        fn from(value: &PutApiV4ProjectsIdContainerExpirationPolicyAttributes) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
        fn default() -> Self {
            Self {
                cadence: Default::default(),
                enabled: Default::default(),
                keep_n: Default::default(),
                name_regex: Default::default(),
                name_regex_keep: Default::default(),
                older_than: Default::default(),
            }
        }
    }
    impl PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
        pub fn builder() -> builder::PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
            Default::default()
        }
    }
    ///Controls visibility of the container registry. One of `disabled`, `private` or `enabled`. `private` will make the container registry accessible only to project members (reporter role and above). `enabled` will make the container registry accessible to everyone who has access to the project. `disabled` will disable the container registry
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Controls visibility of the container registry. One of `disabled`, `private` or `enabled`. `private` will make the container registry accessible only to project members (reporter role and above). `enabled` will make the container registry accessible to everyone who has access to the project. `disabled` will disable the container registry",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdContainerRegistryAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdContainerRegistryAccessLevel {
        fn from(value: &PutApiV4ProjectsIdContainerRegistryAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdContainerRegistryAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdContainerRegistryAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdContainerRegistryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdContainerRegistryAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdContainerRegistryAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Environments access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Environments access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdEnvironmentsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdEnvironmentsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdEnvironmentsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Feature flags access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Feature flags access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdFeatureFlagsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdFeatureFlagsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Forks access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Forks access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdForkingAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdForkingAccessLevel {
        fn from(value: &PutApiV4ProjectsIdForkingAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdForkingAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdForkingAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdForkingAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdForkingAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdForkingAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Infrastructure access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Infrastructure access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdInfrastructureAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdInfrastructureAccessLevel {
        fn from(value: &PutApiV4ProjectsIdInfrastructureAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdInfrastructureAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdInfrastructureAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdInfrastructureAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdInfrastructureAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdInfrastructureAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Issues access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Issues access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdIssuesAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdIssuesAccessLevel {
        fn from(value: &PutApiV4ProjectsIdIssuesAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdIssuesAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdIssuesAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdIssuesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdIssuesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdIssuesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The merge method used when merging merge requests
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The merge method used when merging merge requests",
    ///  "type": "string",
    ///  "enum": [
    ///    "ff",
    ///    "rebase_merge",
    ///    "merge"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdMergeMethod {
        #[serde(rename = "ff")]
        Ff,
        #[serde(rename = "rebase_merge")]
        RebaseMerge,
        #[serde(rename = "merge")]
        Merge,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdMergeMethod {
        fn from(value: &PutApiV4ProjectsIdMergeMethod) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdMergeMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ff => write!(f, "ff"),
                Self::RebaseMerge => write!(f, "rebase_merge"),
                Self::Merge => write!(f, "merge"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdMergeMethod {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ff" => Ok(Self::Ff),
                "rebase_merge" => Ok(Self::RebaseMerge),
                "merge" => Ok(Self::Merge),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdMergeMethod {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdMergeMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdMergeMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Merge requests access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Merge requests access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdMergeRequestsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdMergeRequestsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdMergeRequestsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdMergeRequestsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdMergeRequestsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdMergeRequestsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdMergeRequestsAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdMergeRequestsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Model experiments access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Model experiments access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdModelExperimentsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdModelExperimentsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdModelExperimentsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdModelExperimentsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdModelExperimentsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdModelExperimentsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdModelExperimentsAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdModelExperimentsAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Model registry access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Model registry access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdModelRegistryAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdModelRegistryAccessLevel {
        fn from(value: &PutApiV4ProjectsIdModelRegistryAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdModelRegistryAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdModelRegistryAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdModelRegistryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdModelRegistryAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdModelRegistryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Monitor access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Monitor access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdMonitorAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdMonitorAccessLevel {
        fn from(value: &PutApiV4ProjectsIdMonitorAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdMonitorAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdMonitorAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdMonitorAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdMonitorAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdMonitorAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Pages access level. One of `disabled`, `private`, `enabled` or `public`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Pages access level. One of `disabled`, `private`, `enabled` or `public`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled",
    ///    "public"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdPagesAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "public")]
        Public,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdPagesAccessLevel {
        fn from(value: &PutApiV4ProjectsIdPagesAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdPagesAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
                Self::Public => write!(f, "public"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdPagesAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                "public" => Ok(Self::Public),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdPagesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdPagesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdPagesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Releases access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Releases access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdReleasesAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdReleasesAccessLevel {
        fn from(value: &PutApiV4ProjectsIdReleasesAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdReleasesAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdReleasesAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdReleasesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdReleasesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdReleasesAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Repository access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Repository access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdRepositoryAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdRepositoryAccessLevel {
        fn from(value: &PutApiV4ProjectsIdRepositoryAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdRepositoryAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdRepositoryAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdRepositoryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdRepositoryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdRepositoryAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Requirements feature access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Requirements feature access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdRequirementsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdRequirementsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdRequirementsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdRequirementsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdRequirementsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdRequirementsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdRequirementsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdRequirementsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Security and compliance access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Security and compliance access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdSecurityAndComplianceAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel {
        fn from(value: &PutApiV4ProjectsIdSecurityAndComplianceAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
        for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
        for PutApiV4ProjectsIdSecurityAndComplianceAccessLevel
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Snippets access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Snippets access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdSnippetsAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdSnippetsAccessLevel {
        fn from(value: &PutApiV4ProjectsIdSnippetsAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdSnippetsAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdSnippetsAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdSnippetsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdSnippetsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdSnippetsAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Squash default for project. One of `never`, `always`, `default_on`, or `default_off`.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Squash default for project. One of `never`, `always`, `default_on`, or `default_off`.",
    ///  "type": "string",
    ///  "enum": [
    ///    "never",
    ///    "always",
    ///    "default_on",
    ///    "default_off"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdSquashOption {
        #[serde(rename = "never")]
        Never,
        #[serde(rename = "always")]
        Always,
        #[serde(rename = "default_on")]
        DefaultOn,
        #[serde(rename = "default_off")]
        DefaultOff,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdSquashOption {
        fn from(value: &PutApiV4ProjectsIdSquashOption) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdSquashOption {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Never => write!(f, "never"),
                Self::Always => write!(f, "always"),
                Self::DefaultOn => write!(f, "default_on"),
                Self::DefaultOff => write!(f, "default_off"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdSquashOption {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "never" => Ok(Self::Never),
                "always" => Ok(Self::Always),
                "default_on" => Ok(Self::DefaultOn),
                "default_off" => Ok(Self::DefaultOff),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdSquashOption {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdSquashOption {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdSquashOption {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///The visibility of the project.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The visibility of the project.",
    ///  "type": "string",
    ///  "enum": [
    ///    "private",
    ///    "internal",
    ///    "public"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdVisibility {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "public")]
        Public,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdVisibility {
        fn from(value: &PutApiV4ProjectsIdVisibility) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdVisibility {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Private => write!(f, "private"),
                Self::Internal => write!(f, "internal"),
                Self::Public => write!(f, "public"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdVisibility {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "private" => Ok(Self::Private),
                "internal" => Ok(Self::Internal),
                "public" => Ok(Self::Public),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdVisibility {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdVisibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdVisibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Wiki access level. One of `disabled`, `private` or `enabled`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Wiki access level. One of `disabled`, `private` or `enabled`",
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "private",
    ///    "enabled"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PutApiV4ProjectsIdWikiAccessLevel {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "enabled")]
        Enabled,
    }
    impl ::std::convert::From<&Self> for PutApiV4ProjectsIdWikiAccessLevel {
        fn from(value: &PutApiV4ProjectsIdWikiAccessLevel) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for PutApiV4ProjectsIdWikiAccessLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Private => write!(f, "private"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }
    impl ::std::str::FromStr for PutApiV4ProjectsIdWikiAccessLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "private" => Ok(Self::Private),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for PutApiV4ProjectsIdWikiAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PutApiV4ProjectsIdWikiAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PutApiV4ProjectsIdWikiAccessLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesBasicProjectDetails {
            avatar_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            custom_attributes: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                ::std::string::String,
            >,
            default_branch: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            forks_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            http_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            last_activity_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            license: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesLicenseBasic>,
                ::std::string::String,
            >,
            license_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            namespace: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesNamespaceBasic>,
                ::std::string::String,
            >,
            path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            readme_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_storage: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ssh_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            star_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            tag_list: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            topics: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesBasicProjectDetails {
            fn default() -> Self {
                Self {
                    avatar_url: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    custom_attributes: Ok(Default::default()),
                    default_branch: Ok(Default::default()),
                    description: Ok(Default::default()),
                    forks_count: Ok(Default::default()),
                    http_url_to_repo: Ok(Default::default()),
                    id: Ok(Default::default()),
                    last_activity_at: Ok(Default::default()),
                    license: Ok(Default::default()),
                    license_url: Ok(Default::default()),
                    name: Ok(Default::default()),
                    name_with_namespace: Ok(Default::default()),
                    namespace: Ok(Default::default()),
                    path: Ok(Default::default()),
                    path_with_namespace: Ok(Default::default()),
                    readme_url: Ok(Default::default()),
                    repository_storage: Ok(Default::default()),
                    ssh_url_to_repo: Ok(Default::default()),
                    star_count: Ok(Default::default()),
                    tag_list: Ok(Default::default()),
                    topics: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesBasicProjectDetails {
            pub fn avatar_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_url: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn custom_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.custom_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn forks_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.forks_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for forks_count: {}", e));
                self
            }
            pub fn http_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.http_url_to_repo = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for http_url_to_repo: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn last_activity_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_activity_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for last_activity_at: {}",
                        e
                    )
                });
                self
            }
            pub fn license<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesLicenseBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.license = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license: {}", e));
                self
            }
            pub fn license_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.license_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license_url: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn name_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for name_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesNamespaceBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.namespace = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for namespace: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn path_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for path_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn readme_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.readme_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for readme_url: {}", e));
                self
            }
            pub fn repository_storage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_storage = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_storage: {}",
                        e
                    )
                });
                self
            }
            pub fn ssh_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssh_url_to_repo = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ssh_url_to_repo: {}", e)
                });
                self
            }
            pub fn star_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.star_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for star_count: {}", e));
                self
            }
            pub fn tag_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tag_list = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag_list: {}", e));
                self
            }
            pub fn topics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.topics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for topics: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesBasicProjectDetails>
            for super::ApiEntitiesBasicProjectDetails
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesBasicProjectDetails,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    avatar_url: value.avatar_url?,
                    created_at: value.created_at?,
                    custom_attributes: value.custom_attributes?,
                    default_branch: value.default_branch?,
                    description: value.description?,
                    forks_count: value.forks_count?,
                    http_url_to_repo: value.http_url_to_repo?,
                    id: value.id?,
                    last_activity_at: value.last_activity_at?,
                    license: value.license?,
                    license_url: value.license_url?,
                    name: value.name?,
                    name_with_namespace: value.name_with_namespace?,
                    namespace: value.namespace?,
                    path: value.path?,
                    path_with_namespace: value.path_with_namespace?,
                    readme_url: value.readme_url?,
                    repository_storage: value.repository_storage?,
                    ssh_url_to_repo: value.ssh_url_to_repo?,
                    star_count: value.star_count?,
                    tag_list: value.tag_list?,
                    topics: value.topics?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesBasicProjectDetails>
            for ApiEntitiesBasicProjectDetails
        {
            fn from(value: super::ApiEntitiesBasicProjectDetails) -> Self {
                Self {
                    avatar_url: Ok(value.avatar_url),
                    created_at: Ok(value.created_at),
                    custom_attributes: Ok(value.custom_attributes),
                    default_branch: Ok(value.default_branch),
                    description: Ok(value.description),
                    forks_count: Ok(value.forks_count),
                    http_url_to_repo: Ok(value.http_url_to_repo),
                    id: Ok(value.id),
                    last_activity_at: Ok(value.last_activity_at),
                    license: Ok(value.license),
                    license_url: Ok(value.license_url),
                    name: Ok(value.name),
                    name_with_namespace: Ok(value.name_with_namespace),
                    namespace: Ok(value.namespace),
                    path: Ok(value.path),
                    path_with_namespace: Ok(value.path_with_namespace),
                    readme_url: Ok(value.readme_url),
                    repository_storage: Ok(value.repository_storage),
                    ssh_url_to_repo: Ok(value.ssh_url_to_repo),
                    star_count: Ok(value.star_count),
                    tag_list: Ok(value.tag_list),
                    topics: Ok(value.topics),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesCiPipelineBasic {
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            iid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            project_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ref_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            sha: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            source: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesCiPipelineBasic {
            fn default() -> Self {
                Self {
                    created_at: Ok(Default::default()),
                    id: Ok(Default::default()),
                    iid: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    ref_: Ok(Default::default()),
                    sha: Ok(Default::default()),
                    source: Ok(Default::default()),
                    status: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesCiPipelineBasic {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn iid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.iid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for iid: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn ref_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ref_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesCiPipelineBasic> for super::ApiEntitiesCiPipelineBasic {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesCiPipelineBasic,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    iid: value.iid?,
                    project_id: value.project_id?,
                    ref_: value.ref_?,
                    sha: value.sha?,
                    source: value.source?,
                    status: value.status?,
                    updated_at: value.updated_at?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesCiPipelineBasic> for ApiEntitiesCiPipelineBasic {
            fn from(value: super::ApiEntitiesCiPipelineBasic) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    iid: Ok(value.iid),
                    project_id: Ok(value.project_id),
                    ref_: Ok(value.ref_),
                    sha: Ok(value.sha),
                    source: Ok(value.source),
                    status: Ok(value.status),
                    updated_at: Ok(value.updated_at),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesCommit {
            author_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            author_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            authored_date: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            committed_date: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            committer_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            committer_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            extended_trailers: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            parent_ids: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            short_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            trailers: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesCommit {
            fn default() -> Self {
                Self {
                    author_email: Ok(Default::default()),
                    author_name: Ok(Default::default()),
                    authored_date: Ok(Default::default()),
                    committed_date: Ok(Default::default()),
                    committer_email: Ok(Default::default()),
                    committer_name: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    extended_trailers: Ok(Default::default()),
                    id: Ok(Default::default()),
                    message: Ok(Default::default()),
                    parent_ids: Ok(Default::default()),
                    short_id: Ok(Default::default()),
                    title: Ok(Default::default()),
                    trailers: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesCommit {
            pub fn author_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_email = value.try_into().map_err(|e| {
                    format!("error converting supplied value for author_email: {}", e)
                });
                self
            }
            pub fn author_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author_name: {}", e));
                self
            }
            pub fn authored_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.authored_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for authored_date: {}", e)
                });
                self
            }
            pub fn committed_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committed_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_date: {}", e)
                });
                self
            }
            pub fn committer_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.committer_email = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committer_email: {}", e)
                });
                self
            }
            pub fn committer_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.committer_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committer_name: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn extended_trailers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.extended_trailers = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for extended_trailers: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn parent_ids<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.parent_ids = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parent_ids: {}", e));
                self
            }
            pub fn short_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for short_id: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn trailers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.trailers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trailers: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesCommit> for super::ApiEntitiesCommit {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesCommit,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    author_email: value.author_email?,
                    author_name: value.author_name?,
                    authored_date: value.authored_date?,
                    committed_date: value.committed_date?,
                    committer_email: value.committer_email?,
                    committer_name: value.committer_name?,
                    created_at: value.created_at?,
                    extended_trailers: value.extended_trailers?,
                    id: value.id?,
                    message: value.message?,
                    parent_ids: value.parent_ids?,
                    short_id: value.short_id?,
                    title: value.title?,
                    trailers: value.trailers?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesCommit> for ApiEntitiesCommit {
            fn from(value: super::ApiEntitiesCommit) -> Self {
                Self {
                    author_email: Ok(value.author_email),
                    author_name: Ok(value.author_name),
                    authored_date: Ok(value.authored_date),
                    committed_date: Ok(value.committed_date),
                    committer_email: Ok(value.committer_email),
                    committer_name: Ok(value.committer_name),
                    created_at: Ok(value.created_at),
                    extended_trailers: Ok(value.extended_trailers),
                    id: Ok(value.id),
                    message: Ok(value.message),
                    parent_ids: Ok(value.parent_ids),
                    short_id: Ok(value.short_id),
                    title: Ok(value.title),
                    trailers: Ok(value.trailers),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesCommitDetail {
            author_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            author_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            authored_date: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            committed_date: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            committer_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            committer_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            extended_trailers: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_pipeline: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesCiPipelineBasic>,
                ::std::string::String,
            >,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            parent_ids: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            project_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            short_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            stats: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesCommitStats>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            trailers: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesCommitDetail {
            fn default() -> Self {
                Self {
                    author_email: Ok(Default::default()),
                    author_name: Ok(Default::default()),
                    authored_date: Ok(Default::default()),
                    committed_date: Ok(Default::default()),
                    committer_email: Ok(Default::default()),
                    committer_name: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    extended_trailers: Ok(Default::default()),
                    id: Ok(Default::default()),
                    last_pipeline: Ok(Default::default()),
                    message: Ok(Default::default()),
                    parent_ids: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    short_id: Ok(Default::default()),
                    stats: Ok(Default::default()),
                    status: Ok(Default::default()),
                    title: Ok(Default::default()),
                    trailers: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesCommitDetail {
            pub fn author_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_email = value.try_into().map_err(|e| {
                    format!("error converting supplied value for author_email: {}", e)
                });
                self
            }
            pub fn author_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author_name: {}", e));
                self
            }
            pub fn authored_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.authored_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for authored_date: {}", e)
                });
                self
            }
            pub fn committed_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committed_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_date: {}", e)
                });
                self
            }
            pub fn committer_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.committer_email = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committer_email: {}", e)
                });
                self
            }
            pub fn committer_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.committer_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committer_name: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn extended_trailers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.extended_trailers = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for extended_trailers: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn last_pipeline<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesCiPipelineBasic>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_pipeline = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_pipeline: {}", e)
                });
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn parent_ids<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.parent_ids = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parent_ids: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn short_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for short_id: {}", e));
                self
            }
            pub fn stats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesCommitStats>>,
                T::Error: ::std::fmt::Display,
            {
                self.stats = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stats: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn trailers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.trailers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trailers: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesCommitDetail> for super::ApiEntitiesCommitDetail {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesCommitDetail,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    author_email: value.author_email?,
                    author_name: value.author_name?,
                    authored_date: value.authored_date?,
                    committed_date: value.committed_date?,
                    committer_email: value.committer_email?,
                    committer_name: value.committer_name?,
                    created_at: value.created_at?,
                    extended_trailers: value.extended_trailers?,
                    id: value.id?,
                    last_pipeline: value.last_pipeline?,
                    message: value.message?,
                    parent_ids: value.parent_ids?,
                    project_id: value.project_id?,
                    short_id: value.short_id?,
                    stats: value.stats?,
                    status: value.status?,
                    title: value.title?,
                    trailers: value.trailers?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesCommitDetail> for ApiEntitiesCommitDetail {
            fn from(value: super::ApiEntitiesCommitDetail) -> Self {
                Self {
                    author_email: Ok(value.author_email),
                    author_name: Ok(value.author_name),
                    authored_date: Ok(value.authored_date),
                    committed_date: Ok(value.committed_date),
                    committer_email: Ok(value.committer_email),
                    committer_name: Ok(value.committer_name),
                    created_at: Ok(value.created_at),
                    extended_trailers: Ok(value.extended_trailers),
                    id: Ok(value.id),
                    last_pipeline: Ok(value.last_pipeline),
                    message: Ok(value.message),
                    parent_ids: Ok(value.parent_ids),
                    project_id: Ok(value.project_id),
                    short_id: Ok(value.short_id),
                    stats: Ok(value.stats),
                    status: Ok(value.status),
                    title: Ok(value.title),
                    trailers: Ok(value.trailers),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesCommitStats {
            additions: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            deletions: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            total: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }
        impl ::std::default::Default for ApiEntitiesCommitStats {
            fn default() -> Self {
                Self {
                    additions: Ok(Default::default()),
                    deletions: Ok(Default::default()),
                    total: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesCommitStats {
            pub fn additions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.additions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for additions: {}", e));
                self
            }
            pub fn deletions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.deletions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deletions: {}", e));
                self
            }
            pub fn total<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesCommitStats> for super::ApiEntitiesCommitStats {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesCommitStats,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    additions: value.additions?,
                    deletions: value.deletions?,
                    total: value.total?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesCommitStats> for ApiEntitiesCommitStats {
            fn from(value: super::ApiEntitiesCommitStats) -> Self {
                Self {
                    additions: Ok(value.additions),
                    deletions: Ok(value.deletions),
                    total: Ok(value.total),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesContainerExpirationPolicy {
            cadence: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            keep_n: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name_regex: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name_regex_keep: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            next_run_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            older_than: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesContainerExpirationPolicy {
            fn default() -> Self {
                Self {
                    cadence: Ok(Default::default()),
                    enabled: Ok(Default::default()),
                    keep_n: Ok(Default::default()),
                    name_regex: Ok(Default::default()),
                    name_regex_keep: Ok(Default::default()),
                    next_run_at: Ok(Default::default()),
                    older_than: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesContainerExpirationPolicy {
            pub fn cadence<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cadence = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cadence: {}", e));
                self
            }
            pub fn enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.enabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for enabled: {}", e));
                self
            }
            pub fn keep_n<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.keep_n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for keep_n: {}", e));
                self
            }
            pub fn name_regex<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_regex = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name_regex: {}", e));
                self
            }
            pub fn name_regex_keep<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_regex_keep = value.try_into().map_err(|e| {
                    format!("error converting supplied value for name_regex_keep: {}", e)
                });
                self
            }
            pub fn next_run_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_run_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_run_at: {}", e));
                self
            }
            pub fn older_than<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.older_than = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for older_than: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesContainerExpirationPolicy>
            for super::ApiEntitiesContainerExpirationPolicy
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesContainerExpirationPolicy,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cadence: value.cadence?,
                    enabled: value.enabled?,
                    keep_n: value.keep_n?,
                    name_regex: value.name_regex?,
                    name_regex_keep: value.name_regex_keep?,
                    next_run_at: value.next_run_at?,
                    older_than: value.older_than?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesContainerExpirationPolicy>
            for ApiEntitiesContainerExpirationPolicy
        {
            fn from(value: super::ApiEntitiesContainerExpirationPolicy) -> Self {
                Self {
                    cadence: Ok(value.cadence),
                    enabled: Ok(value.enabled),
                    keep_n: Ok(value.keep_n),
                    name_regex: Ok(value.name_regex),
                    name_regex_keep: Ok(value.name_regex_keep),
                    next_run_at: Ok(value.next_run_at),
                    older_than: Ok(value.older_than),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesCustomAttribute {
            key: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesCustomAttribute {
            fn default() -> Self {
                Self {
                    key: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesCustomAttribute {
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesCustomAttribute> for super::ApiEntitiesCustomAttribute {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesCustomAttribute,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    key: value.key?,
                    value: value.value?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesCustomAttribute> for ApiEntitiesCustomAttribute {
            fn from(value: super::ApiEntitiesCustomAttribute) -> Self {
                Self {
                    key: Ok(value.key),
                    value: Ok(value.value),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesEvent {
            action_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            author: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesUserBasic>,
                ::std::string::String,
            >,
            author_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            author_username: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            imported: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            imported_from: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            note: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesNote>,
                ::std::string::String,
            >,
            project_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            push_data: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesPushEventPayload>,
                ::std::string::String,
            >,
            target_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            target_iid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            target_title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            target_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_page: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesWikiPageBasic>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesEvent {
            fn default() -> Self {
                Self {
                    action_name: Ok(Default::default()),
                    author: Ok(Default::default()),
                    author_id: Ok(Default::default()),
                    author_username: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    id: Ok(Default::default()),
                    imported: Ok(Default::default()),
                    imported_from: Ok(Default::default()),
                    note: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    push_data: Ok(Default::default()),
                    target_id: Ok(Default::default()),
                    target_iid: Ok(Default::default()),
                    target_title: Ok(Default::default()),
                    target_type: Ok(Default::default()),
                    wiki_page: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesEvent {
            pub fn action_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.action_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action_name: {}", e));
                self
            }
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesUserBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {}", e));
                self
            }
            pub fn author_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author_id: {}", e));
                self
            }
            pub fn author_username<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_username = value.try_into().map_err(|e| {
                    format!("error converting supplied value for author_username: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn imported<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.imported = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for imported: {}", e));
                self
            }
            pub fn imported_from<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.imported_from = value.try_into().map_err(|e| {
                    format!("error converting supplied value for imported_from: {}", e)
                });
                self
            }
            pub fn note<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesNote>>,
                T::Error: ::std::fmt::Display,
            {
                self.note = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for note: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn push_data<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesPushEventPayload>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.push_data = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for push_data: {}", e));
                self
            }
            pub fn target_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.target_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for target_id: {}", e));
                self
            }
            pub fn target_iid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.target_iid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for target_iid: {}", e));
                self
            }
            pub fn target_title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.target_title = value.try_into().map_err(|e| {
                    format!("error converting supplied value for target_title: {}", e)
                });
                self
            }
            pub fn target_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.target_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for target_type: {}", e));
                self
            }
            pub fn wiki_page<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesWikiPageBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_page = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for wiki_page: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesEvent> for super::ApiEntitiesEvent {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesEvent,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    action_name: value.action_name?,
                    author: value.author?,
                    author_id: value.author_id?,
                    author_username: value.author_username?,
                    created_at: value.created_at?,
                    id: value.id?,
                    imported: value.imported?,
                    imported_from: value.imported_from?,
                    note: value.note?,
                    project_id: value.project_id?,
                    push_data: value.push_data?,
                    target_id: value.target_id?,
                    target_iid: value.target_iid?,
                    target_title: value.target_title?,
                    target_type: value.target_type?,
                    wiki_page: value.wiki_page?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesEvent> for ApiEntitiesEvent {
            fn from(value: super::ApiEntitiesEvent) -> Self {
                Self {
                    action_name: Ok(value.action_name),
                    author: Ok(value.author),
                    author_id: Ok(value.author_id),
                    author_username: Ok(value.author_username),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    imported: Ok(value.imported),
                    imported_from: Ok(value.imported_from),
                    note: Ok(value.note),
                    project_id: Ok(value.project_id),
                    push_data: Ok(value.push_data),
                    target_id: Ok(value.target_id),
                    target_iid: Ok(value.target_iid),
                    target_title: Ok(value.target_title),
                    target_type: Ok(value.target_type),
                    wiki_page: Ok(value.wiki_page),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesGroupAccess {
            access_level: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            notification_level:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }
        impl ::std::default::Default for ApiEntitiesGroupAccess {
            fn default() -> Self {
                Self {
                    access_level: Ok(Default::default()),
                    notification_level: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesGroupAccess {
            pub fn access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.access_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_level: {}", e)
                });
                self
            }
            pub fn notification_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.notification_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for notification_level: {}",
                        e
                    )
                });
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesGroupAccess> for super::ApiEntitiesGroupAccess {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesGroupAccess,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_level: value.access_level?,
                    notification_level: value.notification_level?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesGroupAccess> for ApiEntitiesGroupAccess {
            fn from(value: super::ApiEntitiesGroupAccess) -> Self {
                Self {
                    access_level: Ok(value.access_level),
                    notification_level: Ok(value.notification_level),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesLicenseBasic {
            html_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            key: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            nickname: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            source_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesLicenseBasic {
            fn default() -> Self {
                Self {
                    html_url: Ok(Default::default()),
                    key: Ok(Default::default()),
                    name: Ok(Default::default()),
                    nickname: Ok(Default::default()),
                    source_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesLicenseBasic {
            pub fn html_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.html_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for html_url: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn nickname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.nickname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nickname: {}", e));
                self
            }
            pub fn source_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.source_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesLicenseBasic> for super::ApiEntitiesLicenseBasic {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesLicenseBasic,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    html_url: value.html_url?,
                    key: value.key?,
                    name: value.name?,
                    nickname: value.nickname?,
                    source_url: value.source_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesLicenseBasic> for ApiEntitiesLicenseBasic {
            fn from(value: super::ApiEntitiesLicenseBasic) -> Self {
                Self {
                    html_url: Ok(value.html_url),
                    key: Ok(value.key),
                    name: Ok(value.name),
                    nickname: Ok(value.nickname),
                    source_url: Ok(value.source_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesNamespaceBasic {
            avatar_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            full_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            kind: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            parent_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesNamespaceBasic {
            fn default() -> Self {
                Self {
                    avatar_url: Ok(Default::default()),
                    full_path: Ok(Default::default()),
                    id: Ok(Default::default()),
                    kind: Ok(Default::default()),
                    name: Ok(Default::default()),
                    parent_id: Ok(Default::default()),
                    path: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesNamespaceBasic {
            pub fn avatar_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_url: {}", e));
                self
            }
            pub fn full_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.full_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for full_path: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn kind<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.kind = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kind: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn parent_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.parent_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parent_id: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesNamespaceBasic> for super::ApiEntitiesNamespaceBasic {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesNamespaceBasic,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    avatar_url: value.avatar_url?,
                    full_path: value.full_path?,
                    id: value.id?,
                    kind: value.kind?,
                    name: value.name?,
                    parent_id: value.parent_id?,
                    path: value.path?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesNamespaceBasic> for ApiEntitiesNamespaceBasic {
            fn from(value: super::ApiEntitiesNamespaceBasic) -> Self {
                Self {
                    avatar_url: Ok(value.avatar_url),
                    full_path: Ok(value.full_path),
                    id: Ok(value.id),
                    kind: Ok(value.kind),
                    name: Ok(value.name),
                    parent_id: Ok(value.parent_id),
                    path: Ok(value.path),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesNote {
            attachment: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            author: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesUserBasic>,
                ::std::string::String,
            >,
            body: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commands_changes: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            commit_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            confidential: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            created_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            imported: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            imported_from: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            internal: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            noteable_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            noteable_iid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            noteable_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            position: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
            project_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            resolvable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            resolved: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            resolved_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            resolved_by: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesUserBasic>,
                ::std::string::String,
            >,
            system: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesNote {
            fn default() -> Self {
                Self {
                    attachment: Ok(Default::default()),
                    author: Ok(Default::default()),
                    body: Ok(Default::default()),
                    commands_changes: Ok(Default::default()),
                    commit_id: Ok(Default::default()),
                    confidential: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    id: Ok(Default::default()),
                    imported: Ok(Default::default()),
                    imported_from: Ok(Default::default()),
                    internal: Ok(Default::default()),
                    noteable_id: Ok(Default::default()),
                    noteable_iid: Ok(Default::default()),
                    noteable_type: Ok(Default::default()),
                    position: Ok(Default::default()),
                    project_id: Ok(Default::default()),
                    resolvable: Ok(Default::default()),
                    resolved: Ok(Default::default()),
                    resolved_at: Ok(Default::default()),
                    resolved_by: Ok(Default::default()),
                    system: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesNote {
            pub fn attachment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.attachment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attachment: {}", e));
                self
            }
            pub fn author<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesUserBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.author = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author: {}", e));
                self
            }
            pub fn body<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.body = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for body: {}", e));
                self
            }
            pub fn commands_changes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.commands_changes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for commands_changes: {}",
                        e
                    )
                });
                self
            }
            pub fn commit_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit_id: {}", e));
                self
            }
            pub fn confidential<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.confidential = value.try_into().map_err(|e| {
                    format!("error converting supplied value for confidential: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn imported<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.imported = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for imported: {}", e));
                self
            }
            pub fn imported_from<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.imported_from = value.try_into().map_err(|e| {
                    format!("error converting supplied value for imported_from: {}", e)
                });
                self
            }
            pub fn internal<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.internal = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for internal: {}", e));
                self
            }
            pub fn noteable_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.noteable_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for noteable_id: {}", e));
                self
            }
            pub fn noteable_iid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.noteable_iid = value.try_into().map_err(|e| {
                    format!("error converting supplied value for noteable_iid: {}", e)
                });
                self
            }
            pub fn noteable_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.noteable_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for noteable_type: {}", e)
                });
                self
            }
            pub fn position<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.position = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for position: {}", e));
                self
            }
            pub fn project_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for project_id: {}", e));
                self
            }
            pub fn resolvable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolvable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for resolvable: {}", e));
                self
            }
            pub fn resolved<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolved = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for resolved: {}", e));
                self
            }
            pub fn resolved_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolved_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for resolved_at: {}", e));
                self
            }
            pub fn resolved_by<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesUserBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolved_by = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for resolved_by: {}", e));
                self
            }
            pub fn system<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.system = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for system: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesNote> for super::ApiEntitiesNote {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesNote,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attachment: value.attachment?,
                    author: value.author?,
                    body: value.body?,
                    commands_changes: value.commands_changes?,
                    commit_id: value.commit_id?,
                    confidential: value.confidential?,
                    created_at: value.created_at?,
                    id: value.id?,
                    imported: value.imported?,
                    imported_from: value.imported_from?,
                    internal: value.internal?,
                    noteable_id: value.noteable_id?,
                    noteable_iid: value.noteable_iid?,
                    noteable_type: value.noteable_type?,
                    position: value.position?,
                    project_id: value.project_id?,
                    resolvable: value.resolvable?,
                    resolved: value.resolved?,
                    resolved_at: value.resolved_at?,
                    resolved_by: value.resolved_by?,
                    system: value.system?,
                    type_: value.type_?,
                    updated_at: value.updated_at?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesNote> for ApiEntitiesNote {
            fn from(value: super::ApiEntitiesNote) -> Self {
                Self {
                    attachment: Ok(value.attachment),
                    author: Ok(value.author),
                    body: Ok(value.body),
                    commands_changes: Ok(value.commands_changes),
                    commit_id: Ok(value.commit_id),
                    confidential: Ok(value.confidential),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    imported: Ok(value.imported),
                    imported_from: Ok(value.imported_from),
                    internal: Ok(value.internal),
                    noteable_id: Ok(value.noteable_id),
                    noteable_iid: Ok(value.noteable_iid),
                    noteable_type: Ok(value.noteable_type),
                    position: Ok(value.position),
                    project_id: Ok(value.project_id),
                    resolvable: Ok(value.resolvable),
                    resolved: Ok(value.resolved),
                    resolved_at: Ok(value.resolved_at),
                    resolved_by: Ok(value.resolved_by),
                    system: Ok(value.system),
                    type_: Ok(value.type_),
                    updated_at: Ok(value.updated_at),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProject {
            allow_merge_on_skipped_pipeline:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            allow_pipeline_trigger_approve_deployment:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            analytics_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            approvals_before_merge:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            archived: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            auto_cancel_pending_pipelines: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            auto_devops_deploy_strategy: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            auto_devops_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            autoclose_referenced_issues:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            avatar_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            build_git_strategy: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            build_timeout: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            builds_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            can_create_merge_request_in:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_allow_fork_pipelines_to_run_in_parent_project:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_config_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_default_git_depth:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_delete_pipelines_in_seconds:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_forward_deployment_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_forward_deployment_rollback_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_id_token_sub_claim_components: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            ci_job_token_scope_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_pipeline_variables_minimum_override_role: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_push_repository_for_job_token_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_restrict_pipeline_cancellation_role: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_separated_caches:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            compliance_frameworks:
                ::std::result::Result<::std::vec::Vec<::serde_json::Value>, ::std::string::String>,
            container_expiration_policy: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesContainerExpirationPolicy>,
                ::std::string::String,
            >,
            container_registry_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            container_registry_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            container_registry_image_prefix: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            creator_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            custom_attributes: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                ::std::string::String,
            >,
            default_branch: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description_html: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            emails_disabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            emails_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            empty_repo: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            enforce_auth_checks_on_uploads:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            environments_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            external_authorization_classification_label: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            feature_flags_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            forked_from_project: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesBasicProjectDetails>,
                ::std::string::String,
            >,
            forking_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            forks_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            group_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            http_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            import_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            infrastructure_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issue_branch_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            issues_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            jobs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            keep_latest_artifact:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            last_activity_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            lfs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            license: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesLicenseBasic>,
                ::std::string::String,
            >,
            license_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            links: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectLinks>,
                ::std::string::String,
            >,
            marked_for_deletion_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            marked_for_deletion_on: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            max_artifacts_size:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            merge_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_pipelines_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_requests_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_trains_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_trains_skip_train_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror_overwrites_diverged_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mirror_trigger_builds: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mirror_user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            model_experiments_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            model_registry_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            monitor_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mr_default_target_self:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            namespace: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesNamespaceBasic>,
                ::std::string::String,
            >,
            only_allow_merge_if_all_discussions_are_resolved:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_all_status_checks_passed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_pipeline_succeeds:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_mirror_protected_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            open_issues_count:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            owner: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesUserBasic>,
                ::std::string::String,
            >,
            packages_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            pages_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            pre_receive_secret_detection_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            prevent_merge_without_jira_issue:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            printing_merge_request_link_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            public_jobs: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            readme_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            releases_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            remove_source_branch_after_merge:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            repository_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_object_format: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_storage: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            request_access_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            requirements_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            requirements_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            resolve_outdated_diff_discussions:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            restrict_user_defined_variables:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            runner_token_expiration_interval:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            runners_token: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            secret_push_protection_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            security_and_compliance_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            security_and_compliance_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            service_desk_address: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            service_desk_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            shared_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            shared_with_groups: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            snippets_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            snippets_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            squash_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            squash_option: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ssh_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            star_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            statistics: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectStatistics>,
                ::std::string::String,
            >,
            suggestion_commit_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tag_list: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            topics: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            warn_about_potentially_unwanted_characters:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        }
        impl ::std::default::Default for ApiEntitiesProject {
            fn default() -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(Default::default()),
                    allow_pipeline_trigger_approve_deployment: Ok(Default::default()),
                    analytics_access_level: Ok(Default::default()),
                    approvals_before_merge: Ok(Default::default()),
                    archived: Ok(Default::default()),
                    auto_cancel_pending_pipelines: Ok(Default::default()),
                    auto_devops_deploy_strategy: Ok(Default::default()),
                    auto_devops_enabled: Ok(Default::default()),
                    autoclose_referenced_issues: Ok(Default::default()),
                    avatar_url: Ok(Default::default()),
                    build_git_strategy: Ok(Default::default()),
                    build_timeout: Ok(Default::default()),
                    builds_access_level: Ok(Default::default()),
                    can_create_merge_request_in: Ok(Default::default()),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(Default::default()),
                    ci_config_path: Ok(Default::default()),
                    ci_default_git_depth: Ok(Default::default()),
                    ci_delete_pipelines_in_seconds: Ok(Default::default()),
                    ci_forward_deployment_enabled: Ok(Default::default()),
                    ci_forward_deployment_rollback_allowed: Ok(Default::default()),
                    ci_id_token_sub_claim_components: Ok(Default::default()),
                    ci_job_token_scope_enabled: Ok(Default::default()),
                    ci_pipeline_variables_minimum_override_role: Ok(Default::default()),
                    ci_push_repository_for_job_token_allowed: Ok(Default::default()),
                    ci_restrict_pipeline_cancellation_role: Ok(Default::default()),
                    ci_separated_caches: Ok(Default::default()),
                    compliance_frameworks: Ok(Default::default()),
                    container_expiration_policy: Ok(Default::default()),
                    container_registry_access_level: Ok(Default::default()),
                    container_registry_enabled: Ok(Default::default()),
                    container_registry_image_prefix: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    creator_id: Ok(Default::default()),
                    custom_attributes: Ok(Default::default()),
                    default_branch: Ok(Default::default()),
                    description: Ok(Default::default()),
                    description_html: Ok(Default::default()),
                    emails_disabled: Ok(Default::default()),
                    emails_enabled: Ok(Default::default()),
                    empty_repo: Ok(Default::default()),
                    enforce_auth_checks_on_uploads: Ok(Default::default()),
                    environments_access_level: Ok(Default::default()),
                    external_authorization_classification_label: Ok(Default::default()),
                    feature_flags_access_level: Ok(Default::default()),
                    forked_from_project: Ok(Default::default()),
                    forking_access_level: Ok(Default::default()),
                    forks_count: Ok(Default::default()),
                    group_runners_enabled: Ok(Default::default()),
                    http_url_to_repo: Ok(Default::default()),
                    id: Ok(Default::default()),
                    import_error: Ok(Default::default()),
                    import_status: Ok(Default::default()),
                    import_type: Ok(Default::default()),
                    import_url: Ok(Default::default()),
                    infrastructure_access_level: Ok(Default::default()),
                    issue_branch_template: Ok(Default::default()),
                    issues_access_level: Ok(Default::default()),
                    issues_enabled: Ok(Default::default()),
                    issues_template: Ok(Default::default()),
                    jobs_enabled: Ok(Default::default()),
                    keep_latest_artifact: Ok(Default::default()),
                    last_activity_at: Ok(Default::default()),
                    lfs_enabled: Ok(Default::default()),
                    license: Ok(Default::default()),
                    license_url: Ok(Default::default()),
                    links: Ok(Default::default()),
                    marked_for_deletion_at: Ok(Default::default()),
                    marked_for_deletion_on: Ok(Default::default()),
                    max_artifacts_size: Ok(Default::default()),
                    merge_commit_template: Ok(Default::default()),
                    merge_method: Ok(Default::default()),
                    merge_pipelines_enabled: Ok(Default::default()),
                    merge_requests_access_level: Ok(Default::default()),
                    merge_requests_enabled: Ok(Default::default()),
                    merge_requests_template: Ok(Default::default()),
                    merge_trains_enabled: Ok(Default::default()),
                    merge_trains_skip_train_allowed: Ok(Default::default()),
                    mirror: Ok(Default::default()),
                    mirror_overwrites_diverged_branches: Ok(Default::default()),
                    mirror_trigger_builds: Ok(Default::default()),
                    mirror_user_id: Ok(Default::default()),
                    model_experiments_access_level: Ok(Default::default()),
                    model_registry_access_level: Ok(Default::default()),
                    monitor_access_level: Ok(Default::default()),
                    mr_default_target_self: Ok(Default::default()),
                    name: Ok(Default::default()),
                    name_with_namespace: Ok(Default::default()),
                    namespace: Ok(Default::default()),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(Default::default()),
                    only_allow_merge_if_all_status_checks_passed: Ok(Default::default()),
                    only_allow_merge_if_pipeline_succeeds: Ok(Default::default()),
                    only_mirror_protected_branches: Ok(Default::default()),
                    open_issues_count: Ok(Default::default()),
                    owner: Ok(Default::default()),
                    packages_enabled: Ok(Default::default()),
                    pages_access_level: Ok(Default::default()),
                    path: Ok(Default::default()),
                    path_with_namespace: Ok(Default::default()),
                    pre_receive_secret_detection_enabled: Ok(Default::default()),
                    prevent_merge_without_jira_issue: Ok(Default::default()),
                    printing_merge_request_link_enabled: Ok(Default::default()),
                    public_jobs: Ok(Default::default()),
                    readme_url: Ok(Default::default()),
                    releases_access_level: Ok(Default::default()),
                    remove_source_branch_after_merge: Ok(Default::default()),
                    repository_access_level: Ok(Default::default()),
                    repository_object_format: Ok(Default::default()),
                    repository_storage: Ok(Default::default()),
                    request_access_enabled: Ok(Default::default()),
                    requirements_access_level: Ok(Default::default()),
                    requirements_enabled: Ok(Default::default()),
                    resolve_outdated_diff_discussions: Ok(Default::default()),
                    restrict_user_defined_variables: Ok(Default::default()),
                    runner_token_expiration_interval: Ok(Default::default()),
                    runners_token: Ok(Default::default()),
                    secret_push_protection_enabled: Ok(Default::default()),
                    security_and_compliance_access_level: Ok(Default::default()),
                    security_and_compliance_enabled: Ok(Default::default()),
                    service_desk_address: Ok(Default::default()),
                    service_desk_enabled: Ok(Default::default()),
                    shared_runners_enabled: Ok(Default::default()),
                    shared_with_groups: Ok(Default::default()),
                    snippets_access_level: Ok(Default::default()),
                    snippets_enabled: Ok(Default::default()),
                    squash_commit_template: Ok(Default::default()),
                    squash_option: Ok(Default::default()),
                    ssh_url_to_repo: Ok(Default::default()),
                    star_count: Ok(Default::default()),
                    statistics: Ok(Default::default()),
                    suggestion_commit_message: Ok(Default::default()),
                    tag_list: Ok(Default::default()),
                    topics: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    visibility: Ok(Default::default()),
                    warn_about_potentially_unwanted_characters: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                    wiki_access_level: Ok(Default::default()),
                    wiki_enabled: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProject {
            pub fn allow_merge_on_skipped_pipeline<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_merge_on_skipped_pipeline = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for allow_merge_on_skipped_pipeline: {}",
                        e
                    )
                });
                self
            }
            pub fn allow_pipeline_trigger_approve_deployment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_pipeline_trigger_approve_deployment = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for allow_pipeline_trigger_approve_deployment: {}",
                            e
                        )
                    });
                self
            }
            pub fn analytics_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.analytics_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for analytics_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn approvals_before_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.approvals_before_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for approvals_before_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn archived<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.archived = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for archived: {}", e));
                self
            }
            pub fn auto_cancel_pending_pipelines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_cancel_pending_pipelines = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_cancel_pending_pipelines: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_deploy_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_deploy_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_deploy_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn autoclose_referenced_issues<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.autoclose_referenced_issues = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for autoclose_referenced_issues: {}",
                        e
                    )
                });
                self
            }
            pub fn avatar_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_url: {}", e));
                self
            }
            pub fn build_git_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.build_git_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for build_git_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn build_timeout<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.build_timeout = value.try_into().map_err(|e| {
                    format!("error converting supplied value for build_timeout: {}", e)
                });
                self
            }
            pub fn builds_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.builds_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for builds_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn can_create_merge_request_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.can_create_merge_request_in = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for can_create_merge_request_in: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_allow_fork_pipelines_to_run_in_parent_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_allow_fork_pipelines_to_run_in_parent_project = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_allow_fork_pipelines_to_run_in_parent_project: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_config_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_config_path = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ci_config_path: {}", e)
                });
                self
            }
            pub fn ci_default_git_depth<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_default_git_depth = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_default_git_depth: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_delete_pipelines_in_seconds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_delete_pipelines_in_seconds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_delete_pipelines_in_seconds: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_forward_deployment_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_rollback_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_rollback_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_forward_deployment_rollback_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_id_token_sub_claim_components<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_id_token_sub_claim_components = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_id_token_sub_claim_components: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_job_token_scope_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_job_token_scope_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_job_token_scope_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_pipeline_variables_minimum_override_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_pipeline_variables_minimum_override_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_pipeline_variables_minimum_override_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_push_repository_for_job_token_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_push_repository_for_job_token_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_push_repository_for_job_token_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_restrict_pipeline_cancellation_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_restrict_pipeline_cancellation_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_restrict_pipeline_cancellation_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_separated_caches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_separated_caches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_separated_caches: {}",
                        e
                    )
                });
                self
            }
            pub fn compliance_frameworks<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
                T::Error: ::std::fmt::Display,
            {
                self.compliance_frameworks = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for compliance_frameworks: {}",
                        e
                    )
                });
                self
            }
            pub fn container_expiration_policy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesContainerExpirationPolicy>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.container_expiration_policy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_expiration_policy: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_image_prefix<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_image_prefix = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_image_prefix: {}",
                        e
                    )
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn creator_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.creator_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for creator_id: {}", e));
                self
            }
            pub fn custom_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.custom_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn description_html<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description_html = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for description_html: {}",
                        e
                    )
                });
                self
            }
            pub fn emails_disabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_disabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_disabled: {}", e)
                });
                self
            }
            pub fn emails_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_enabled: {}", e)
                });
                self
            }
            pub fn empty_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.empty_repo = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for empty_repo: {}", e));
                self
            }
            pub fn enforce_auth_checks_on_uploads<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.enforce_auth_checks_on_uploads = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for enforce_auth_checks_on_uploads: {}",
                        e
                    )
                });
                self
            }
            pub fn environments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.environments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for environments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn external_authorization_classification_label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.external_authorization_classification_label = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for external_authorization_classification_label: {}",
                            e
                        )
                    });
                self
            }
            pub fn feature_flags_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.feature_flags_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for feature_flags_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn forked_from_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesBasicProjectDetails>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.forked_from_project = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for forked_from_project: {}",
                        e
                    )
                });
                self
            }
            pub fn forking_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.forking_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for forking_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn forks_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.forks_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for forks_count: {}", e));
                self
            }
            pub fn group_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for group_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn http_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.http_url_to_repo = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for http_url_to_repo: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn import_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for import_error: {}", e)
                });
                self
            }
            pub fn import_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for import_status: {}", e)
                });
                self
            }
            pub fn import_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for import_type: {}", e));
                self
            }
            pub fn import_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for import_url: {}", e));
                self
            }
            pub fn infrastructure_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.infrastructure_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for infrastructure_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issue_branch_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issue_branch_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issue_branch_template: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issues_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_enabled: {}", e)
                });
                self
            }
            pub fn issues_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_template = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_template: {}", e)
                });
                self
            }
            pub fn jobs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.jobs_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for jobs_enabled: {}", e)
                });
                self
            }
            pub fn keep_latest_artifact<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.keep_latest_artifact = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for keep_latest_artifact: {}",
                        e
                    )
                });
                self
            }
            pub fn last_activity_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_activity_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for last_activity_at: {}",
                        e
                    )
                });
                self
            }
            pub fn lfs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.lfs_enabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lfs_enabled: {}", e));
                self
            }
            pub fn license<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesLicenseBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.license = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license: {}", e));
                self
            }
            pub fn license_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.license_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license_url: {}", e));
                self
            }
            pub fn links<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesProjectLinks>>,
                T::Error: ::std::fmt::Display,
            {
                self.links = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for links: {}", e));
                self
            }
            pub fn marked_for_deletion_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.marked_for_deletion_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for marked_for_deletion_at: {}",
                        e
                    )
                });
                self
            }
            pub fn marked_for_deletion_on<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.marked_for_deletion_on = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for marked_for_deletion_on: {}",
                        e
                    )
                });
                self
            }
            pub fn max_artifacts_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_artifacts_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for max_artifacts_size: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for merge_method: {}", e)
                });
                self
            }
            pub fn merge_pipelines_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_pipelines_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_pipelines_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_skip_train_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_skip_train_allowed = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_skip_train_allowed: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mirror: {}", e));
                self
            }
            pub fn mirror_overwrites_diverged_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_overwrites_diverged_branches = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mirror_overwrites_diverged_branches: {}",
                            e
                        )
                    });
                self
            }
            pub fn mirror_trigger_builds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_trigger_builds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mirror_trigger_builds: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror_user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_user_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mirror_user_id: {}", e)
                });
                self
            }
            pub fn model_experiments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.model_experiments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_experiments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn model_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.model_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn monitor_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.monitor_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for monitor_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn mr_default_target_self<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mr_default_target_self = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mr_default_target_self: {}",
                        e
                    )
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn name_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for name_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesNamespaceBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.namespace = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for namespace: {}", e));
                self
            }
            pub fn only_allow_merge_if_all_discussions_are_resolved<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_discussions_are_resolved = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_discussions_are_resolved: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_all_status_checks_passed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_status_checks_passed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_status_checks_passed: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_pipeline_succeeds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_pipeline_succeeds = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_pipeline_succeeds: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_mirror_protected_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_mirror_protected_branches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for only_mirror_protected_branches: {}",
                        e
                    )
                });
                self
            }
            pub fn open_issues_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_issues_count = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for open_issues_count: {}",
                        e
                    )
                });
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesUserBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
            pub fn packages_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.packages_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for packages_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn pages_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pages_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for pages_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn path_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for path_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn pre_receive_secret_detection_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.pre_receive_secret_detection_enabled = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for pre_receive_secret_detection_enabled: {}",
                            e
                        )
                    });
                self
            }
            pub fn prevent_merge_without_jira_issue<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.prevent_merge_without_jira_issue = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for prevent_merge_without_jira_issue: {}",
                        e
                    )
                });
                self
            }
            pub fn printing_merge_request_link_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.printing_merge_request_link_enabled = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for printing_merge_request_link_enabled: {}",
                            e
                        )
                    });
                self
            }
            pub fn public_jobs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.public_jobs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for public_jobs: {}", e));
                self
            }
            pub fn readme_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.readme_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for readme_url: {}", e));
                self
            }
            pub fn releases_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.releases_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for releases_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn remove_source_branch_after_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.remove_source_branch_after_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for remove_source_branch_after_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_object_format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_object_format = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_object_format: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_storage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_storage = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_storage: {}",
                        e
                    )
                });
                self
            }
            pub fn request_access_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.request_access_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for request_access_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn requirements_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.requirements_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for requirements_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn requirements_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.requirements_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for requirements_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn resolve_outdated_diff_discussions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolve_outdated_diff_discussions = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for resolve_outdated_diff_discussions: {}",
                        e
                    )
                });
                self
            }
            pub fn restrict_user_defined_variables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.restrict_user_defined_variables = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for restrict_user_defined_variables: {}",
                        e
                    )
                });
                self
            }
            pub fn runner_token_expiration_interval<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.runner_token_expiration_interval = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for runner_token_expiration_interval: {}",
                        e
                    )
                });
                self
            }
            pub fn runners_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.runners_token = value.try_into().map_err(|e| {
                    format!("error converting supplied value for runners_token: {}", e)
                });
                self
            }
            pub fn secret_push_protection_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.secret_push_protection_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for secret_push_protection_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn security_and_compliance_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_and_compliance_access_level = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for security_and_compliance_access_level: {}",
                            e
                        )
                    });
                self
            }
            pub fn security_and_compliance_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_and_compliance_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for security_and_compliance_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn service_desk_address<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.service_desk_address = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for service_desk_address: {}",
                        e
                    )
                });
                self
            }
            pub fn service_desk_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.service_desk_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for service_desk_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn shared_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.shared_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for shared_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn shared_with_groups<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.shared_with_groups = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for shared_with_groups: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.squash_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for squash_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_option<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.squash_option = value.try_into().map_err(|e| {
                    format!("error converting supplied value for squash_option: {}", e)
                });
                self
            }
            pub fn ssh_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssh_url_to_repo = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ssh_url_to_repo: {}", e)
                });
                self
            }
            pub fn star_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.star_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for star_count: {}", e));
                self
            }
            pub fn statistics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesProjectStatistics>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.statistics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for statistics: {}", e));
                self
            }
            pub fn suggestion_commit_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suggestion_commit_message = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for suggestion_commit_message: {}",
                        e
                    )
                });
                self
            }
            pub fn tag_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tag_list = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag_list: {}", e));
                self
            }
            pub fn topics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.topics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for topics: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
            pub fn warn_about_potentially_unwanted_characters<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.warn_about_potentially_unwanted_characters = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for warn_about_potentially_unwanted_characters: {}",
                            e
                        )
                    });
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
            pub fn wiki_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for wiki_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn wiki_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for wiki_enabled: {}", e)
                });
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProject> for super::ApiEntitiesProject {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProject,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    allow_merge_on_skipped_pipeline: value.allow_merge_on_skipped_pipeline?,
                    allow_pipeline_trigger_approve_deployment: value
                        .allow_pipeline_trigger_approve_deployment?,
                    analytics_access_level: value.analytics_access_level?,
                    approvals_before_merge: value.approvals_before_merge?,
                    archived: value.archived?,
                    auto_cancel_pending_pipelines: value.auto_cancel_pending_pipelines?,
                    auto_devops_deploy_strategy: value.auto_devops_deploy_strategy?,
                    auto_devops_enabled: value.auto_devops_enabled?,
                    autoclose_referenced_issues: value.autoclose_referenced_issues?,
                    avatar_url: value.avatar_url?,
                    build_git_strategy: value.build_git_strategy?,
                    build_timeout: value.build_timeout?,
                    builds_access_level: value.builds_access_level?,
                    can_create_merge_request_in: value.can_create_merge_request_in?,
                    ci_allow_fork_pipelines_to_run_in_parent_project: value
                        .ci_allow_fork_pipelines_to_run_in_parent_project?,
                    ci_config_path: value.ci_config_path?,
                    ci_default_git_depth: value.ci_default_git_depth?,
                    ci_delete_pipelines_in_seconds: value.ci_delete_pipelines_in_seconds?,
                    ci_forward_deployment_enabled: value.ci_forward_deployment_enabled?,
                    ci_forward_deployment_rollback_allowed: value
                        .ci_forward_deployment_rollback_allowed?,
                    ci_id_token_sub_claim_components: value.ci_id_token_sub_claim_components?,
                    ci_job_token_scope_enabled: value.ci_job_token_scope_enabled?,
                    ci_pipeline_variables_minimum_override_role: value
                        .ci_pipeline_variables_minimum_override_role?,
                    ci_push_repository_for_job_token_allowed: value
                        .ci_push_repository_for_job_token_allowed?,
                    ci_restrict_pipeline_cancellation_role: value
                        .ci_restrict_pipeline_cancellation_role?,
                    ci_separated_caches: value.ci_separated_caches?,
                    compliance_frameworks: value.compliance_frameworks?,
                    container_expiration_policy: value.container_expiration_policy?,
                    container_registry_access_level: value.container_registry_access_level?,
                    container_registry_enabled: value.container_registry_enabled?,
                    container_registry_image_prefix: value.container_registry_image_prefix?,
                    created_at: value.created_at?,
                    creator_id: value.creator_id?,
                    custom_attributes: value.custom_attributes?,
                    default_branch: value.default_branch?,
                    description: value.description?,
                    description_html: value.description_html?,
                    emails_disabled: value.emails_disabled?,
                    emails_enabled: value.emails_enabled?,
                    empty_repo: value.empty_repo?,
                    enforce_auth_checks_on_uploads: value.enforce_auth_checks_on_uploads?,
                    environments_access_level: value.environments_access_level?,
                    external_authorization_classification_label: value
                        .external_authorization_classification_label?,
                    feature_flags_access_level: value.feature_flags_access_level?,
                    forked_from_project: value.forked_from_project?,
                    forking_access_level: value.forking_access_level?,
                    forks_count: value.forks_count?,
                    group_runners_enabled: value.group_runners_enabled?,
                    http_url_to_repo: value.http_url_to_repo?,
                    id: value.id?,
                    import_error: value.import_error?,
                    import_status: value.import_status?,
                    import_type: value.import_type?,
                    import_url: value.import_url?,
                    infrastructure_access_level: value.infrastructure_access_level?,
                    issue_branch_template: value.issue_branch_template?,
                    issues_access_level: value.issues_access_level?,
                    issues_enabled: value.issues_enabled?,
                    issues_template: value.issues_template?,
                    jobs_enabled: value.jobs_enabled?,
                    keep_latest_artifact: value.keep_latest_artifact?,
                    last_activity_at: value.last_activity_at?,
                    lfs_enabled: value.lfs_enabled?,
                    license: value.license?,
                    license_url: value.license_url?,
                    links: value.links?,
                    marked_for_deletion_at: value.marked_for_deletion_at?,
                    marked_for_deletion_on: value.marked_for_deletion_on?,
                    max_artifacts_size: value.max_artifacts_size?,
                    merge_commit_template: value.merge_commit_template?,
                    merge_method: value.merge_method?,
                    merge_pipelines_enabled: value.merge_pipelines_enabled?,
                    merge_requests_access_level: value.merge_requests_access_level?,
                    merge_requests_enabled: value.merge_requests_enabled?,
                    merge_requests_template: value.merge_requests_template?,
                    merge_trains_enabled: value.merge_trains_enabled?,
                    merge_trains_skip_train_allowed: value.merge_trains_skip_train_allowed?,
                    mirror: value.mirror?,
                    mirror_overwrites_diverged_branches: value
                        .mirror_overwrites_diverged_branches?,
                    mirror_trigger_builds: value.mirror_trigger_builds?,
                    mirror_user_id: value.mirror_user_id?,
                    model_experiments_access_level: value.model_experiments_access_level?,
                    model_registry_access_level: value.model_registry_access_level?,
                    monitor_access_level: value.monitor_access_level?,
                    mr_default_target_self: value.mr_default_target_self?,
                    name: value.name?,
                    name_with_namespace: value.name_with_namespace?,
                    namespace: value.namespace?,
                    only_allow_merge_if_all_discussions_are_resolved: value
                        .only_allow_merge_if_all_discussions_are_resolved?,
                    only_allow_merge_if_all_status_checks_passed: value
                        .only_allow_merge_if_all_status_checks_passed?,
                    only_allow_merge_if_pipeline_succeeds: value
                        .only_allow_merge_if_pipeline_succeeds?,
                    only_mirror_protected_branches: value.only_mirror_protected_branches?,
                    open_issues_count: value.open_issues_count?,
                    owner: value.owner?,
                    packages_enabled: value.packages_enabled?,
                    pages_access_level: value.pages_access_level?,
                    path: value.path?,
                    path_with_namespace: value.path_with_namespace?,
                    pre_receive_secret_detection_enabled: value
                        .pre_receive_secret_detection_enabled?,
                    prevent_merge_without_jira_issue: value.prevent_merge_without_jira_issue?,
                    printing_merge_request_link_enabled: value
                        .printing_merge_request_link_enabled?,
                    public_jobs: value.public_jobs?,
                    readme_url: value.readme_url?,
                    releases_access_level: value.releases_access_level?,
                    remove_source_branch_after_merge: value.remove_source_branch_after_merge?,
                    repository_access_level: value.repository_access_level?,
                    repository_object_format: value.repository_object_format?,
                    repository_storage: value.repository_storage?,
                    request_access_enabled: value.request_access_enabled?,
                    requirements_access_level: value.requirements_access_level?,
                    requirements_enabled: value.requirements_enabled?,
                    resolve_outdated_diff_discussions: value.resolve_outdated_diff_discussions?,
                    restrict_user_defined_variables: value.restrict_user_defined_variables?,
                    runner_token_expiration_interval: value.runner_token_expiration_interval?,
                    runners_token: value.runners_token?,
                    secret_push_protection_enabled: value.secret_push_protection_enabled?,
                    security_and_compliance_access_level: value
                        .security_and_compliance_access_level?,
                    security_and_compliance_enabled: value.security_and_compliance_enabled?,
                    service_desk_address: value.service_desk_address?,
                    service_desk_enabled: value.service_desk_enabled?,
                    shared_runners_enabled: value.shared_runners_enabled?,
                    shared_with_groups: value.shared_with_groups?,
                    snippets_access_level: value.snippets_access_level?,
                    snippets_enabled: value.snippets_enabled?,
                    squash_commit_template: value.squash_commit_template?,
                    squash_option: value.squash_option?,
                    ssh_url_to_repo: value.ssh_url_to_repo?,
                    star_count: value.star_count?,
                    statistics: value.statistics?,
                    suggestion_commit_message: value.suggestion_commit_message?,
                    tag_list: value.tag_list?,
                    topics: value.topics?,
                    updated_at: value.updated_at?,
                    visibility: value.visibility?,
                    warn_about_potentially_unwanted_characters: value
                        .warn_about_potentially_unwanted_characters?,
                    web_url: value.web_url?,
                    wiki_access_level: value.wiki_access_level?,
                    wiki_enabled: value.wiki_enabled?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProject> for ApiEntitiesProject {
            fn from(value: super::ApiEntitiesProject) -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(value.allow_merge_on_skipped_pipeline),
                    allow_pipeline_trigger_approve_deployment: Ok(
                        value.allow_pipeline_trigger_approve_deployment
                    ),
                    analytics_access_level: Ok(value.analytics_access_level),
                    approvals_before_merge: Ok(value.approvals_before_merge),
                    archived: Ok(value.archived),
                    auto_cancel_pending_pipelines: Ok(value.auto_cancel_pending_pipelines),
                    auto_devops_deploy_strategy: Ok(value.auto_devops_deploy_strategy),
                    auto_devops_enabled: Ok(value.auto_devops_enabled),
                    autoclose_referenced_issues: Ok(value.autoclose_referenced_issues),
                    avatar_url: Ok(value.avatar_url),
                    build_git_strategy: Ok(value.build_git_strategy),
                    build_timeout: Ok(value.build_timeout),
                    builds_access_level: Ok(value.builds_access_level),
                    can_create_merge_request_in: Ok(value.can_create_merge_request_in),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(
                        value.ci_allow_fork_pipelines_to_run_in_parent_project
                    ),
                    ci_config_path: Ok(value.ci_config_path),
                    ci_default_git_depth: Ok(value.ci_default_git_depth),
                    ci_delete_pipelines_in_seconds: Ok(value.ci_delete_pipelines_in_seconds),
                    ci_forward_deployment_enabled: Ok(value.ci_forward_deployment_enabled),
                    ci_forward_deployment_rollback_allowed: Ok(
                        value.ci_forward_deployment_rollback_allowed
                    ),
                    ci_id_token_sub_claim_components: Ok(value.ci_id_token_sub_claim_components),
                    ci_job_token_scope_enabled: Ok(value.ci_job_token_scope_enabled),
                    ci_pipeline_variables_minimum_override_role: Ok(
                        value.ci_pipeline_variables_minimum_override_role
                    ),
                    ci_push_repository_for_job_token_allowed: Ok(
                        value.ci_push_repository_for_job_token_allowed
                    ),
                    ci_restrict_pipeline_cancellation_role: Ok(
                        value.ci_restrict_pipeline_cancellation_role
                    ),
                    ci_separated_caches: Ok(value.ci_separated_caches),
                    compliance_frameworks: Ok(value.compliance_frameworks),
                    container_expiration_policy: Ok(value.container_expiration_policy),
                    container_registry_access_level: Ok(value.container_registry_access_level),
                    container_registry_enabled: Ok(value.container_registry_enabled),
                    container_registry_image_prefix: Ok(value.container_registry_image_prefix),
                    created_at: Ok(value.created_at),
                    creator_id: Ok(value.creator_id),
                    custom_attributes: Ok(value.custom_attributes),
                    default_branch: Ok(value.default_branch),
                    description: Ok(value.description),
                    description_html: Ok(value.description_html),
                    emails_disabled: Ok(value.emails_disabled),
                    emails_enabled: Ok(value.emails_enabled),
                    empty_repo: Ok(value.empty_repo),
                    enforce_auth_checks_on_uploads: Ok(value.enforce_auth_checks_on_uploads),
                    environments_access_level: Ok(value.environments_access_level),
                    external_authorization_classification_label: Ok(
                        value.external_authorization_classification_label
                    ),
                    feature_flags_access_level: Ok(value.feature_flags_access_level),
                    forked_from_project: Ok(value.forked_from_project),
                    forking_access_level: Ok(value.forking_access_level),
                    forks_count: Ok(value.forks_count),
                    group_runners_enabled: Ok(value.group_runners_enabled),
                    http_url_to_repo: Ok(value.http_url_to_repo),
                    id: Ok(value.id),
                    import_error: Ok(value.import_error),
                    import_status: Ok(value.import_status),
                    import_type: Ok(value.import_type),
                    import_url: Ok(value.import_url),
                    infrastructure_access_level: Ok(value.infrastructure_access_level),
                    issue_branch_template: Ok(value.issue_branch_template),
                    issues_access_level: Ok(value.issues_access_level),
                    issues_enabled: Ok(value.issues_enabled),
                    issues_template: Ok(value.issues_template),
                    jobs_enabled: Ok(value.jobs_enabled),
                    keep_latest_artifact: Ok(value.keep_latest_artifact),
                    last_activity_at: Ok(value.last_activity_at),
                    lfs_enabled: Ok(value.lfs_enabled),
                    license: Ok(value.license),
                    license_url: Ok(value.license_url),
                    links: Ok(value.links),
                    marked_for_deletion_at: Ok(value.marked_for_deletion_at),
                    marked_for_deletion_on: Ok(value.marked_for_deletion_on),
                    max_artifacts_size: Ok(value.max_artifacts_size),
                    merge_commit_template: Ok(value.merge_commit_template),
                    merge_method: Ok(value.merge_method),
                    merge_pipelines_enabled: Ok(value.merge_pipelines_enabled),
                    merge_requests_access_level: Ok(value.merge_requests_access_level),
                    merge_requests_enabled: Ok(value.merge_requests_enabled),
                    merge_requests_template: Ok(value.merge_requests_template),
                    merge_trains_enabled: Ok(value.merge_trains_enabled),
                    merge_trains_skip_train_allowed: Ok(value.merge_trains_skip_train_allowed),
                    mirror: Ok(value.mirror),
                    mirror_overwrites_diverged_branches: Ok(
                        value.mirror_overwrites_diverged_branches
                    ),
                    mirror_trigger_builds: Ok(value.mirror_trigger_builds),
                    mirror_user_id: Ok(value.mirror_user_id),
                    model_experiments_access_level: Ok(value.model_experiments_access_level),
                    model_registry_access_level: Ok(value.model_registry_access_level),
                    monitor_access_level: Ok(value.monitor_access_level),
                    mr_default_target_self: Ok(value.mr_default_target_self),
                    name: Ok(value.name),
                    name_with_namespace: Ok(value.name_with_namespace),
                    namespace: Ok(value.namespace),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(
                        value.only_allow_merge_if_all_discussions_are_resolved
                    ),
                    only_allow_merge_if_all_status_checks_passed: Ok(
                        value.only_allow_merge_if_all_status_checks_passed
                    ),
                    only_allow_merge_if_pipeline_succeeds: Ok(
                        value.only_allow_merge_if_pipeline_succeeds
                    ),
                    only_mirror_protected_branches: Ok(value.only_mirror_protected_branches),
                    open_issues_count: Ok(value.open_issues_count),
                    owner: Ok(value.owner),
                    packages_enabled: Ok(value.packages_enabled),
                    pages_access_level: Ok(value.pages_access_level),
                    path: Ok(value.path),
                    path_with_namespace: Ok(value.path_with_namespace),
                    pre_receive_secret_detection_enabled: Ok(
                        value.pre_receive_secret_detection_enabled
                    ),
                    prevent_merge_without_jira_issue: Ok(value.prevent_merge_without_jira_issue),
                    printing_merge_request_link_enabled: Ok(
                        value.printing_merge_request_link_enabled
                    ),
                    public_jobs: Ok(value.public_jobs),
                    readme_url: Ok(value.readme_url),
                    releases_access_level: Ok(value.releases_access_level),
                    remove_source_branch_after_merge: Ok(value.remove_source_branch_after_merge),
                    repository_access_level: Ok(value.repository_access_level),
                    repository_object_format: Ok(value.repository_object_format),
                    repository_storage: Ok(value.repository_storage),
                    request_access_enabled: Ok(value.request_access_enabled),
                    requirements_access_level: Ok(value.requirements_access_level),
                    requirements_enabled: Ok(value.requirements_enabled),
                    resolve_outdated_diff_discussions: Ok(value.resolve_outdated_diff_discussions),
                    restrict_user_defined_variables: Ok(value.restrict_user_defined_variables),
                    runner_token_expiration_interval: Ok(value.runner_token_expiration_interval),
                    runners_token: Ok(value.runners_token),
                    secret_push_protection_enabled: Ok(value.secret_push_protection_enabled),
                    security_and_compliance_access_level: Ok(
                        value.security_and_compliance_access_level
                    ),
                    security_and_compliance_enabled: Ok(value.security_and_compliance_enabled),
                    service_desk_address: Ok(value.service_desk_address),
                    service_desk_enabled: Ok(value.service_desk_enabled),
                    shared_runners_enabled: Ok(value.shared_runners_enabled),
                    shared_with_groups: Ok(value.shared_with_groups),
                    snippets_access_level: Ok(value.snippets_access_level),
                    snippets_enabled: Ok(value.snippets_enabled),
                    squash_commit_template: Ok(value.squash_commit_template),
                    squash_option: Ok(value.squash_option),
                    ssh_url_to_repo: Ok(value.ssh_url_to_repo),
                    star_count: Ok(value.star_count),
                    statistics: Ok(value.statistics),
                    suggestion_commit_message: Ok(value.suggestion_commit_message),
                    tag_list: Ok(value.tag_list),
                    topics: Ok(value.topics),
                    updated_at: Ok(value.updated_at),
                    visibility: Ok(value.visibility),
                    warn_about_potentially_unwanted_characters: Ok(
                        value.warn_about_potentially_unwanted_characters
                    ),
                    web_url: Ok(value.web_url),
                    wiki_access_level: Ok(value.wiki_access_level),
                    wiki_enabled: Ok(value.wiki_enabled),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectAccess {
            access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            notification_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesProjectAccess {
            fn default() -> Self {
                Self {
                    access_level: Ok(Default::default()),
                    notification_level: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectAccess {
            pub fn access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.access_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_level: {}", e)
                });
                self
            }
            pub fn notification_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.notification_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for notification_level: {}",
                        e
                    )
                });
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectAccess> for super::ApiEntitiesProjectAccess {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectAccess,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_level: value.access_level?,
                    notification_level: value.notification_level?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectAccess> for ApiEntitiesProjectAccess {
            fn from(value: super::ApiEntitiesProjectAccess) -> Self {
                Self {
                    access_level: Ok(value.access_level),
                    notification_level: Ok(value.notification_level),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectLinks {
            cluster_agents: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            events: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            members: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_requests: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repo_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            self_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesProjectLinks {
            fn default() -> Self {
                Self {
                    cluster_agents: Ok(Default::default()),
                    events: Ok(Default::default()),
                    issues: Ok(Default::default()),
                    labels: Ok(Default::default()),
                    members: Ok(Default::default()),
                    merge_requests: Ok(Default::default()),
                    repo_branches: Ok(Default::default()),
                    self_: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectLinks {
            pub fn cluster_agents<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cluster_agents = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cluster_agents: {}", e)
                });
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn issues<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for issues: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn members<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.members = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for members: {}", e));
                self
            }
            pub fn merge_requests<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests = value.try_into().map_err(|e| {
                    format!("error converting supplied value for merge_requests: {}", e)
                });
                self
            }
            pub fn repo_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repo_branches = value.try_into().map_err(|e| {
                    format!("error converting supplied value for repo_branches: {}", e)
                });
                self
            }
            pub fn self_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.self_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for self_: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectLinks> for super::ApiEntitiesProjectLinks {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectLinks,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cluster_agents: value.cluster_agents?,
                    events: value.events?,
                    issues: value.issues?,
                    labels: value.labels?,
                    members: value.members?,
                    merge_requests: value.merge_requests?,
                    repo_branches: value.repo_branches?,
                    self_: value.self_?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectLinks> for ApiEntitiesProjectLinks {
            fn from(value: super::ApiEntitiesProjectLinks) -> Self {
                Self {
                    cluster_agents: Ok(value.cluster_agents),
                    events: Ok(value.events),
                    issues: Ok(value.issues),
                    labels: Ok(value.labels),
                    members: Ok(value.members),
                    merge_requests: Ok(value.merge_requests),
                    repo_branches: Ok(value.repo_branches),
                    self_: Ok(value.self_),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectStatistics {
            commit_count: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            container_registry_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            job_artifacts_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lfs_objects_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            packages_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            pipeline_artifacts_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            snippets_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            storage_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            uploads_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_size: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesProjectStatistics {
            fn default() -> Self {
                Self {
                    commit_count: Ok(Default::default()),
                    container_registry_size: Ok(Default::default()),
                    job_artifacts_size: Ok(Default::default()),
                    lfs_objects_size: Ok(Default::default()),
                    packages_size: Ok(Default::default()),
                    pipeline_artifacts_size: Ok(Default::default()),
                    repository_size: Ok(Default::default()),
                    snippets_size: Ok(Default::default()),
                    storage_size: Ok(Default::default()),
                    uploads_size: Ok(Default::default()),
                    wiki_size: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectStatistics {
            pub fn commit_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_count = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commit_count: {}", e)
                });
                self
            }
            pub fn container_registry_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_size: {}",
                        e
                    )
                });
                self
            }
            pub fn job_artifacts_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.job_artifacts_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for job_artifacts_size: {}",
                        e
                    )
                });
                self
            }
            pub fn lfs_objects_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.lfs_objects_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for lfs_objects_size: {}",
                        e
                    )
                });
                self
            }
            pub fn packages_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.packages_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for packages_size: {}", e)
                });
                self
            }
            pub fn pipeline_artifacts_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pipeline_artifacts_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for pipeline_artifacts_size: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for repository_size: {}", e)
                });
                self
            }
            pub fn snippets_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for snippets_size: {}", e)
                });
                self
            }
            pub fn storage_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.storage_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for storage_size: {}", e)
                });
                self
            }
            pub fn uploads_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.uploads_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for uploads_size: {}", e)
                });
                self
            }
            pub fn wiki_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for wiki_size: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectStatistics> for super::ApiEntitiesProjectStatistics {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectStatistics,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commit_count: value.commit_count?,
                    container_registry_size: value.container_registry_size?,
                    job_artifacts_size: value.job_artifacts_size?,
                    lfs_objects_size: value.lfs_objects_size?,
                    packages_size: value.packages_size?,
                    pipeline_artifacts_size: value.pipeline_artifacts_size?,
                    repository_size: value.repository_size?,
                    snippets_size: value.snippets_size?,
                    storage_size: value.storage_size?,
                    uploads_size: value.uploads_size?,
                    wiki_size: value.wiki_size?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectStatistics> for ApiEntitiesProjectStatistics {
            fn from(value: super::ApiEntitiesProjectStatistics) -> Self {
                Self {
                    commit_count: Ok(value.commit_count),
                    container_registry_size: Ok(value.container_registry_size),
                    job_artifacts_size: Ok(value.job_artifacts_size),
                    lfs_objects_size: Ok(value.lfs_objects_size),
                    packages_size: Ok(value.packages_size),
                    pipeline_artifacts_size: Ok(value.pipeline_artifacts_size),
                    repository_size: Ok(value.repository_size),
                    snippets_size: Ok(value.snippets_size),
                    storage_size: Ok(value.storage_size),
                    uploads_size: Ok(value.uploads_size),
                    wiki_size: Ok(value.wiki_size),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectWithAccess {
            allow_merge_on_skipped_pipeline:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            allow_pipeline_trigger_approve_deployment:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            analytics_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            approvals_before_merge:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            archived: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            auto_cancel_pending_pipelines: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            auto_devops_deploy_strategy: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            auto_devops_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            autoclose_referenced_issues:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            avatar_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            build_git_strategy: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            build_timeout: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            builds_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            can_create_merge_request_in:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_allow_fork_pipelines_to_run_in_parent_project:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_config_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_default_git_depth:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_delete_pipelines_in_seconds:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_forward_deployment_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_forward_deployment_rollback_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_id_token_sub_claim_components: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            ci_job_token_scope_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_pipeline_variables_minimum_override_role: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_push_repository_for_job_token_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_restrict_pipeline_cancellation_role: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_separated_caches:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            compliance_frameworks:
                ::std::result::Result<::std::vec::Vec<::serde_json::Value>, ::std::string::String>,
            container_expiration_policy: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesContainerExpirationPolicy>,
                ::std::string::String,
            >,
            container_registry_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            container_registry_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            container_registry_image_prefix: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            creator_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            custom_attributes: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                ::std::string::String,
            >,
            default_branch: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description_html: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            emails_disabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            emails_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            empty_repo: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            enforce_auth_checks_on_uploads:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            environments_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            external_authorization_classification_label: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            feature_flags_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            forked_from_project: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesBasicProjectDetails>,
                ::std::string::String,
            >,
            forking_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            forks_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            group_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            http_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            import_error: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            import_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            infrastructure_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issue_branch_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            issues_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            jobs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            keep_latest_artifact:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            last_activity_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            lfs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            license: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesLicenseBasic>,
                ::std::string::String,
            >,
            license_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            links: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectWithAccessLinks>,
                ::std::string::String,
            >,
            marked_for_deletion_at: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            marked_for_deletion_on: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            max_artifacts_size:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            merge_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_method: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_pipelines_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_requests_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_trains_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_trains_skip_train_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror_overwrites_diverged_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mirror_trigger_builds: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mirror_user_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            model_experiments_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            model_registry_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            monitor_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mr_default_target_self:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            namespace: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesNamespaceBasic>,
                ::std::string::String,
            >,
            only_allow_merge_if_all_discussions_are_resolved:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_all_status_checks_passed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_pipeline_succeeds:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_mirror_protected_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            open_issues_count:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            owner: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesUserBasic>,
                ::std::string::String,
            >,
            packages_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            pages_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            path_with_namespace: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            permissions: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectWithAccessPermissions>,
                ::std::string::String,
            >,
            pre_receive_secret_detection_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            prevent_merge_without_jira_issue:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            printing_merge_request_link_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            public_jobs: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            readme_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            releases_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            remove_source_branch_after_merge:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            repository_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_object_format: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repository_storage: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            request_access_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            requirements_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            requirements_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            resolve_outdated_diff_discussions:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            restrict_user_defined_variables:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            runner_token_expiration_interval:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            runners_token: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            secret_push_protection_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            security_and_compliance_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            security_and_compliance_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            service_desk_address: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            service_desk_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            shared_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            shared_with_groups: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            snippets_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            snippets_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            squash_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            squash_option: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ssh_url_to_repo: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            star_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            statistics: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectStatistics>,
                ::std::string::String,
            >,
            suggestion_commit_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tag_list: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            topics: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            updated_at: ::std::result::Result<
                ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            warn_about_potentially_unwanted_characters:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_access_level: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            wiki_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        }
        impl ::std::default::Default for ApiEntitiesProjectWithAccess {
            fn default() -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(Default::default()),
                    allow_pipeline_trigger_approve_deployment: Ok(Default::default()),
                    analytics_access_level: Ok(Default::default()),
                    approvals_before_merge: Ok(Default::default()),
                    archived: Ok(Default::default()),
                    auto_cancel_pending_pipelines: Ok(Default::default()),
                    auto_devops_deploy_strategy: Ok(Default::default()),
                    auto_devops_enabled: Ok(Default::default()),
                    autoclose_referenced_issues: Ok(Default::default()),
                    avatar_url: Ok(Default::default()),
                    build_git_strategy: Ok(Default::default()),
                    build_timeout: Ok(Default::default()),
                    builds_access_level: Ok(Default::default()),
                    can_create_merge_request_in: Ok(Default::default()),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(Default::default()),
                    ci_config_path: Ok(Default::default()),
                    ci_default_git_depth: Ok(Default::default()),
                    ci_delete_pipelines_in_seconds: Ok(Default::default()),
                    ci_forward_deployment_enabled: Ok(Default::default()),
                    ci_forward_deployment_rollback_allowed: Ok(Default::default()),
                    ci_id_token_sub_claim_components: Ok(Default::default()),
                    ci_job_token_scope_enabled: Ok(Default::default()),
                    ci_pipeline_variables_minimum_override_role: Ok(Default::default()),
                    ci_push_repository_for_job_token_allowed: Ok(Default::default()),
                    ci_restrict_pipeline_cancellation_role: Ok(Default::default()),
                    ci_separated_caches: Ok(Default::default()),
                    compliance_frameworks: Ok(Default::default()),
                    container_expiration_policy: Ok(Default::default()),
                    container_registry_access_level: Ok(Default::default()),
                    container_registry_enabled: Ok(Default::default()),
                    container_registry_image_prefix: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    creator_id: Ok(Default::default()),
                    custom_attributes: Ok(Default::default()),
                    default_branch: Ok(Default::default()),
                    description: Ok(Default::default()),
                    description_html: Ok(Default::default()),
                    emails_disabled: Ok(Default::default()),
                    emails_enabled: Ok(Default::default()),
                    empty_repo: Ok(Default::default()),
                    enforce_auth_checks_on_uploads: Ok(Default::default()),
                    environments_access_level: Ok(Default::default()),
                    external_authorization_classification_label: Ok(Default::default()),
                    feature_flags_access_level: Ok(Default::default()),
                    forked_from_project: Ok(Default::default()),
                    forking_access_level: Ok(Default::default()),
                    forks_count: Ok(Default::default()),
                    group_runners_enabled: Ok(Default::default()),
                    http_url_to_repo: Ok(Default::default()),
                    id: Ok(Default::default()),
                    import_error: Ok(Default::default()),
                    import_status: Ok(Default::default()),
                    import_type: Ok(Default::default()),
                    import_url: Ok(Default::default()),
                    infrastructure_access_level: Ok(Default::default()),
                    issue_branch_template: Ok(Default::default()),
                    issues_access_level: Ok(Default::default()),
                    issues_enabled: Ok(Default::default()),
                    issues_template: Ok(Default::default()),
                    jobs_enabled: Ok(Default::default()),
                    keep_latest_artifact: Ok(Default::default()),
                    last_activity_at: Ok(Default::default()),
                    lfs_enabled: Ok(Default::default()),
                    license: Ok(Default::default()),
                    license_url: Ok(Default::default()),
                    links: Ok(Default::default()),
                    marked_for_deletion_at: Ok(Default::default()),
                    marked_for_deletion_on: Ok(Default::default()),
                    max_artifacts_size: Ok(Default::default()),
                    merge_commit_template: Ok(Default::default()),
                    merge_method: Ok(Default::default()),
                    merge_pipelines_enabled: Ok(Default::default()),
                    merge_requests_access_level: Ok(Default::default()),
                    merge_requests_enabled: Ok(Default::default()),
                    merge_requests_template: Ok(Default::default()),
                    merge_trains_enabled: Ok(Default::default()),
                    merge_trains_skip_train_allowed: Ok(Default::default()),
                    mirror: Ok(Default::default()),
                    mirror_overwrites_diverged_branches: Ok(Default::default()),
                    mirror_trigger_builds: Ok(Default::default()),
                    mirror_user_id: Ok(Default::default()),
                    model_experiments_access_level: Ok(Default::default()),
                    model_registry_access_level: Ok(Default::default()),
                    monitor_access_level: Ok(Default::default()),
                    mr_default_target_self: Ok(Default::default()),
                    name: Ok(Default::default()),
                    name_with_namespace: Ok(Default::default()),
                    namespace: Ok(Default::default()),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(Default::default()),
                    only_allow_merge_if_all_status_checks_passed: Ok(Default::default()),
                    only_allow_merge_if_pipeline_succeeds: Ok(Default::default()),
                    only_mirror_protected_branches: Ok(Default::default()),
                    open_issues_count: Ok(Default::default()),
                    owner: Ok(Default::default()),
                    packages_enabled: Ok(Default::default()),
                    pages_access_level: Ok(Default::default()),
                    path: Ok(Default::default()),
                    path_with_namespace: Ok(Default::default()),
                    permissions: Ok(Default::default()),
                    pre_receive_secret_detection_enabled: Ok(Default::default()),
                    prevent_merge_without_jira_issue: Ok(Default::default()),
                    printing_merge_request_link_enabled: Ok(Default::default()),
                    public_jobs: Ok(Default::default()),
                    readme_url: Ok(Default::default()),
                    releases_access_level: Ok(Default::default()),
                    remove_source_branch_after_merge: Ok(Default::default()),
                    repository_access_level: Ok(Default::default()),
                    repository_object_format: Ok(Default::default()),
                    repository_storage: Ok(Default::default()),
                    request_access_enabled: Ok(Default::default()),
                    requirements_access_level: Ok(Default::default()),
                    requirements_enabled: Ok(Default::default()),
                    resolve_outdated_diff_discussions: Ok(Default::default()),
                    restrict_user_defined_variables: Ok(Default::default()),
                    runner_token_expiration_interval: Ok(Default::default()),
                    runners_token: Ok(Default::default()),
                    secret_push_protection_enabled: Ok(Default::default()),
                    security_and_compliance_access_level: Ok(Default::default()),
                    security_and_compliance_enabled: Ok(Default::default()),
                    service_desk_address: Ok(Default::default()),
                    service_desk_enabled: Ok(Default::default()),
                    shared_runners_enabled: Ok(Default::default()),
                    shared_with_groups: Ok(Default::default()),
                    snippets_access_level: Ok(Default::default()),
                    snippets_enabled: Ok(Default::default()),
                    squash_commit_template: Ok(Default::default()),
                    squash_option: Ok(Default::default()),
                    ssh_url_to_repo: Ok(Default::default()),
                    star_count: Ok(Default::default()),
                    statistics: Ok(Default::default()),
                    suggestion_commit_message: Ok(Default::default()),
                    tag_list: Ok(Default::default()),
                    topics: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                    visibility: Ok(Default::default()),
                    warn_about_potentially_unwanted_characters: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                    wiki_access_level: Ok(Default::default()),
                    wiki_enabled: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectWithAccess {
            pub fn allow_merge_on_skipped_pipeline<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_merge_on_skipped_pipeline = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for allow_merge_on_skipped_pipeline: {}",
                        e
                    )
                });
                self
            }
            pub fn allow_pipeline_trigger_approve_deployment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_pipeline_trigger_approve_deployment = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for allow_pipeline_trigger_approve_deployment: {}",
                            e
                        )
                    });
                self
            }
            pub fn analytics_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.analytics_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for analytics_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn approvals_before_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.approvals_before_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for approvals_before_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn archived<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.archived = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for archived: {}", e));
                self
            }
            pub fn auto_cancel_pending_pipelines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_cancel_pending_pipelines = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_cancel_pending_pipelines: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_deploy_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_deploy_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_deploy_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn autoclose_referenced_issues<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.autoclose_referenced_issues = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for autoclose_referenced_issues: {}",
                        e
                    )
                });
                self
            }
            pub fn avatar_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_url: {}", e));
                self
            }
            pub fn build_git_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.build_git_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for build_git_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn build_timeout<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.build_timeout = value.try_into().map_err(|e| {
                    format!("error converting supplied value for build_timeout: {}", e)
                });
                self
            }
            pub fn builds_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.builds_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for builds_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn can_create_merge_request_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.can_create_merge_request_in = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for can_create_merge_request_in: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_allow_fork_pipelines_to_run_in_parent_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_allow_fork_pipelines_to_run_in_parent_project = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_allow_fork_pipelines_to_run_in_parent_project: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_config_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_config_path = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ci_config_path: {}", e)
                });
                self
            }
            pub fn ci_default_git_depth<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_default_git_depth = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_default_git_depth: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_delete_pipelines_in_seconds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_delete_pipelines_in_seconds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_delete_pipelines_in_seconds: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_forward_deployment_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_rollback_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_rollback_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_forward_deployment_rollback_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_id_token_sub_claim_components<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_id_token_sub_claim_components = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_id_token_sub_claim_components: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_job_token_scope_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_job_token_scope_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_job_token_scope_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_pipeline_variables_minimum_override_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_pipeline_variables_minimum_override_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_pipeline_variables_minimum_override_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_push_repository_for_job_token_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_push_repository_for_job_token_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_push_repository_for_job_token_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_restrict_pipeline_cancellation_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_restrict_pipeline_cancellation_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_restrict_pipeline_cancellation_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_separated_caches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_separated_caches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_separated_caches: {}",
                        e
                    )
                });
                self
            }
            pub fn compliance_frameworks<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
                T::Error: ::std::fmt::Display,
            {
                self.compliance_frameworks = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for compliance_frameworks: {}",
                        e
                    )
                });
                self
            }
            pub fn container_expiration_policy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesContainerExpirationPolicy>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.container_expiration_policy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_expiration_policy: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_image_prefix<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_image_prefix = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_image_prefix: {}",
                        e
                    )
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn creator_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.creator_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for creator_id: {}", e));
                self
            }
            pub fn custom_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesCustomAttribute>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.custom_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn description_html<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description_html = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for description_html: {}",
                        e
                    )
                });
                self
            }
            pub fn emails_disabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_disabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_disabled: {}", e)
                });
                self
            }
            pub fn emails_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_enabled: {}", e)
                });
                self
            }
            pub fn empty_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.empty_repo = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for empty_repo: {}", e));
                self
            }
            pub fn enforce_auth_checks_on_uploads<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.enforce_auth_checks_on_uploads = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for enforce_auth_checks_on_uploads: {}",
                        e
                    )
                });
                self
            }
            pub fn environments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.environments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for environments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn external_authorization_classification_label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.external_authorization_classification_label = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for external_authorization_classification_label: {}",
                            e
                        )
                    });
                self
            }
            pub fn feature_flags_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.feature_flags_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for feature_flags_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn forked_from_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesBasicProjectDetails>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.forked_from_project = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for forked_from_project: {}",
                        e
                    )
                });
                self
            }
            pub fn forking_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.forking_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for forking_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn forks_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.forks_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for forks_count: {}", e));
                self
            }
            pub fn group_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for group_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn http_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.http_url_to_repo = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for http_url_to_repo: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn import_error<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_error = value.try_into().map_err(|e| {
                    format!("error converting supplied value for import_error: {}", e)
                });
                self
            }
            pub fn import_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for import_status: {}", e)
                });
                self
            }
            pub fn import_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for import_type: {}", e));
                self
            }
            pub fn import_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for import_url: {}", e));
                self
            }
            pub fn infrastructure_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.infrastructure_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for infrastructure_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issue_branch_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issue_branch_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issue_branch_template: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issues_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_enabled: {}", e)
                });
                self
            }
            pub fn issues_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_template = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_template: {}", e)
                });
                self
            }
            pub fn jobs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.jobs_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for jobs_enabled: {}", e)
                });
                self
            }
            pub fn keep_latest_artifact<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.keep_latest_artifact = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for keep_latest_artifact: {}",
                        e
                    )
                });
                self
            }
            pub fn last_activity_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.last_activity_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for last_activity_at: {}",
                        e
                    )
                });
                self
            }
            pub fn lfs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.lfs_enabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lfs_enabled: {}", e));
                self
            }
            pub fn license<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesLicenseBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.license = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license: {}", e));
                self
            }
            pub fn license_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.license_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for license_url: {}", e));
                self
            }
            pub fn links<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesProjectWithAccessLinks>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.links = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for links: {}", e));
                self
            }
            pub fn marked_for_deletion_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.marked_for_deletion_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for marked_for_deletion_at: {}",
                        e
                    )
                });
                self
            }
            pub fn marked_for_deletion_on<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.marked_for_deletion_on = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for marked_for_deletion_on: {}",
                        e
                    )
                });
                self
            }
            pub fn max_artifacts_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_artifacts_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for max_artifacts_size: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for merge_method: {}", e)
                });
                self
            }
            pub fn merge_pipelines_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_pipelines_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_pipelines_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_skip_train_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_skip_train_allowed = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_skip_train_allowed: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mirror: {}", e));
                self
            }
            pub fn mirror_overwrites_diverged_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_overwrites_diverged_branches = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mirror_overwrites_diverged_branches: {}",
                            e
                        )
                    });
                self
            }
            pub fn mirror_trigger_builds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_trigger_builds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mirror_trigger_builds: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror_user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_user_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mirror_user_id: {}", e)
                });
                self
            }
            pub fn model_experiments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.model_experiments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_experiments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn model_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.model_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn monitor_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.monitor_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for monitor_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn mr_default_target_self<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mr_default_target_self = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mr_default_target_self: {}",
                        e
                    )
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn name_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for name_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesNamespaceBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.namespace = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for namespace: {}", e));
                self
            }
            pub fn only_allow_merge_if_all_discussions_are_resolved<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_discussions_are_resolved = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_discussions_are_resolved: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_all_status_checks_passed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_status_checks_passed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_status_checks_passed: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_pipeline_succeeds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_pipeline_succeeds = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_pipeline_succeeds: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_mirror_protected_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_mirror_protected_branches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for only_mirror_protected_branches: {}",
                        e
                    )
                });
                self
            }
            pub fn open_issues_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_issues_count = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for open_issues_count: {}",
                        e
                    )
                });
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesUserBasic>>,
                T::Error: ::std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
            pub fn packages_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.packages_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for packages_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn pages_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pages_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for pages_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn path_with_namespace<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path_with_namespace = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for path_with_namespace: {}",
                        e
                    )
                });
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesProjectWithAccessPermissions>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn pre_receive_secret_detection_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.pre_receive_secret_detection_enabled = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for pre_receive_secret_detection_enabled: {}",
                            e
                        )
                    });
                self
            }
            pub fn prevent_merge_without_jira_issue<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.prevent_merge_without_jira_issue = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for prevent_merge_without_jira_issue: {}",
                        e
                    )
                });
                self
            }
            pub fn printing_merge_request_link_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.printing_merge_request_link_enabled = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for printing_merge_request_link_enabled: {}",
                            e
                        )
                    });
                self
            }
            pub fn public_jobs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.public_jobs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for public_jobs: {}", e));
                self
            }
            pub fn readme_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.readme_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for readme_url: {}", e));
                self
            }
            pub fn releases_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.releases_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for releases_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn remove_source_branch_after_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.remove_source_branch_after_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for remove_source_branch_after_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_object_format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_object_format = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_object_format: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_storage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_storage = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_storage: {}",
                        e
                    )
                });
                self
            }
            pub fn request_access_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.request_access_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for request_access_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn requirements_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.requirements_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for requirements_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn requirements_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.requirements_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for requirements_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn resolve_outdated_diff_discussions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolve_outdated_diff_discussions = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for resolve_outdated_diff_discussions: {}",
                        e
                    )
                });
                self
            }
            pub fn restrict_user_defined_variables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.restrict_user_defined_variables = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for restrict_user_defined_variables: {}",
                        e
                    )
                });
                self
            }
            pub fn runner_token_expiration_interval<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.runner_token_expiration_interval = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for runner_token_expiration_interval: {}",
                        e
                    )
                });
                self
            }
            pub fn runners_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.runners_token = value.try_into().map_err(|e| {
                    format!("error converting supplied value for runners_token: {}", e)
                });
                self
            }
            pub fn secret_push_protection_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.secret_push_protection_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for secret_push_protection_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn security_and_compliance_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_and_compliance_access_level = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for security_and_compliance_access_level: {}",
                            e
                        )
                    });
                self
            }
            pub fn security_and_compliance_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_and_compliance_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for security_and_compliance_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn service_desk_address<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.service_desk_address = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for service_desk_address: {}",
                        e
                    )
                });
                self
            }
            pub fn service_desk_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.service_desk_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for service_desk_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn shared_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.shared_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for shared_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn shared_with_groups<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.shared_with_groups = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for shared_with_groups: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.squash_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for squash_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_option<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.squash_option = value.try_into().map_err(|e| {
                    format!("error converting supplied value for squash_option: {}", e)
                });
                self
            }
            pub fn ssh_url_to_repo<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssh_url_to_repo = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ssh_url_to_repo: {}", e)
                });
                self
            }
            pub fn star_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.star_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for star_count: {}", e));
                self
            }
            pub fn statistics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ApiEntitiesProjectStatistics>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.statistics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for statistics: {}", e));
                self
            }
            pub fn suggestion_commit_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suggestion_commit_message = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for suggestion_commit_message: {}",
                        e
                    )
                });
                self
            }
            pub fn tag_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tag_list = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag_list: {}", e));
                self
            }
            pub fn topics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.topics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for topics: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
            pub fn warn_about_potentially_unwanted_characters<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.warn_about_potentially_unwanted_characters = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for warn_about_potentially_unwanted_characters: {}",
                            e
                        )
                    });
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
            pub fn wiki_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for wiki_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn wiki_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for wiki_enabled: {}", e)
                });
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectWithAccess> for super::ApiEntitiesProjectWithAccess {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectWithAccess,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    allow_merge_on_skipped_pipeline: value.allow_merge_on_skipped_pipeline?,
                    allow_pipeline_trigger_approve_deployment: value
                        .allow_pipeline_trigger_approve_deployment?,
                    analytics_access_level: value.analytics_access_level?,
                    approvals_before_merge: value.approvals_before_merge?,
                    archived: value.archived?,
                    auto_cancel_pending_pipelines: value.auto_cancel_pending_pipelines?,
                    auto_devops_deploy_strategy: value.auto_devops_deploy_strategy?,
                    auto_devops_enabled: value.auto_devops_enabled?,
                    autoclose_referenced_issues: value.autoclose_referenced_issues?,
                    avatar_url: value.avatar_url?,
                    build_git_strategy: value.build_git_strategy?,
                    build_timeout: value.build_timeout?,
                    builds_access_level: value.builds_access_level?,
                    can_create_merge_request_in: value.can_create_merge_request_in?,
                    ci_allow_fork_pipelines_to_run_in_parent_project: value
                        .ci_allow_fork_pipelines_to_run_in_parent_project?,
                    ci_config_path: value.ci_config_path?,
                    ci_default_git_depth: value.ci_default_git_depth?,
                    ci_delete_pipelines_in_seconds: value.ci_delete_pipelines_in_seconds?,
                    ci_forward_deployment_enabled: value.ci_forward_deployment_enabled?,
                    ci_forward_deployment_rollback_allowed: value
                        .ci_forward_deployment_rollback_allowed?,
                    ci_id_token_sub_claim_components: value.ci_id_token_sub_claim_components?,
                    ci_job_token_scope_enabled: value.ci_job_token_scope_enabled?,
                    ci_pipeline_variables_minimum_override_role: value
                        .ci_pipeline_variables_minimum_override_role?,
                    ci_push_repository_for_job_token_allowed: value
                        .ci_push_repository_for_job_token_allowed?,
                    ci_restrict_pipeline_cancellation_role: value
                        .ci_restrict_pipeline_cancellation_role?,
                    ci_separated_caches: value.ci_separated_caches?,
                    compliance_frameworks: value.compliance_frameworks?,
                    container_expiration_policy: value.container_expiration_policy?,
                    container_registry_access_level: value.container_registry_access_level?,
                    container_registry_enabled: value.container_registry_enabled?,
                    container_registry_image_prefix: value.container_registry_image_prefix?,
                    created_at: value.created_at?,
                    creator_id: value.creator_id?,
                    custom_attributes: value.custom_attributes?,
                    default_branch: value.default_branch?,
                    description: value.description?,
                    description_html: value.description_html?,
                    emails_disabled: value.emails_disabled?,
                    emails_enabled: value.emails_enabled?,
                    empty_repo: value.empty_repo?,
                    enforce_auth_checks_on_uploads: value.enforce_auth_checks_on_uploads?,
                    environments_access_level: value.environments_access_level?,
                    external_authorization_classification_label: value
                        .external_authorization_classification_label?,
                    feature_flags_access_level: value.feature_flags_access_level?,
                    forked_from_project: value.forked_from_project?,
                    forking_access_level: value.forking_access_level?,
                    forks_count: value.forks_count?,
                    group_runners_enabled: value.group_runners_enabled?,
                    http_url_to_repo: value.http_url_to_repo?,
                    id: value.id?,
                    import_error: value.import_error?,
                    import_status: value.import_status?,
                    import_type: value.import_type?,
                    import_url: value.import_url?,
                    infrastructure_access_level: value.infrastructure_access_level?,
                    issue_branch_template: value.issue_branch_template?,
                    issues_access_level: value.issues_access_level?,
                    issues_enabled: value.issues_enabled?,
                    issues_template: value.issues_template?,
                    jobs_enabled: value.jobs_enabled?,
                    keep_latest_artifact: value.keep_latest_artifact?,
                    last_activity_at: value.last_activity_at?,
                    lfs_enabled: value.lfs_enabled?,
                    license: value.license?,
                    license_url: value.license_url?,
                    links: value.links?,
                    marked_for_deletion_at: value.marked_for_deletion_at?,
                    marked_for_deletion_on: value.marked_for_deletion_on?,
                    max_artifacts_size: value.max_artifacts_size?,
                    merge_commit_template: value.merge_commit_template?,
                    merge_method: value.merge_method?,
                    merge_pipelines_enabled: value.merge_pipelines_enabled?,
                    merge_requests_access_level: value.merge_requests_access_level?,
                    merge_requests_enabled: value.merge_requests_enabled?,
                    merge_requests_template: value.merge_requests_template?,
                    merge_trains_enabled: value.merge_trains_enabled?,
                    merge_trains_skip_train_allowed: value.merge_trains_skip_train_allowed?,
                    mirror: value.mirror?,
                    mirror_overwrites_diverged_branches: value
                        .mirror_overwrites_diverged_branches?,
                    mirror_trigger_builds: value.mirror_trigger_builds?,
                    mirror_user_id: value.mirror_user_id?,
                    model_experiments_access_level: value.model_experiments_access_level?,
                    model_registry_access_level: value.model_registry_access_level?,
                    monitor_access_level: value.monitor_access_level?,
                    mr_default_target_self: value.mr_default_target_self?,
                    name: value.name?,
                    name_with_namespace: value.name_with_namespace?,
                    namespace: value.namespace?,
                    only_allow_merge_if_all_discussions_are_resolved: value
                        .only_allow_merge_if_all_discussions_are_resolved?,
                    only_allow_merge_if_all_status_checks_passed: value
                        .only_allow_merge_if_all_status_checks_passed?,
                    only_allow_merge_if_pipeline_succeeds: value
                        .only_allow_merge_if_pipeline_succeeds?,
                    only_mirror_protected_branches: value.only_mirror_protected_branches?,
                    open_issues_count: value.open_issues_count?,
                    owner: value.owner?,
                    packages_enabled: value.packages_enabled?,
                    pages_access_level: value.pages_access_level?,
                    path: value.path?,
                    path_with_namespace: value.path_with_namespace?,
                    permissions: value.permissions?,
                    pre_receive_secret_detection_enabled: value
                        .pre_receive_secret_detection_enabled?,
                    prevent_merge_without_jira_issue: value.prevent_merge_without_jira_issue?,
                    printing_merge_request_link_enabled: value
                        .printing_merge_request_link_enabled?,
                    public_jobs: value.public_jobs?,
                    readme_url: value.readme_url?,
                    releases_access_level: value.releases_access_level?,
                    remove_source_branch_after_merge: value.remove_source_branch_after_merge?,
                    repository_access_level: value.repository_access_level?,
                    repository_object_format: value.repository_object_format?,
                    repository_storage: value.repository_storage?,
                    request_access_enabled: value.request_access_enabled?,
                    requirements_access_level: value.requirements_access_level?,
                    requirements_enabled: value.requirements_enabled?,
                    resolve_outdated_diff_discussions: value.resolve_outdated_diff_discussions?,
                    restrict_user_defined_variables: value.restrict_user_defined_variables?,
                    runner_token_expiration_interval: value.runner_token_expiration_interval?,
                    runners_token: value.runners_token?,
                    secret_push_protection_enabled: value.secret_push_protection_enabled?,
                    security_and_compliance_access_level: value
                        .security_and_compliance_access_level?,
                    security_and_compliance_enabled: value.security_and_compliance_enabled?,
                    service_desk_address: value.service_desk_address?,
                    service_desk_enabled: value.service_desk_enabled?,
                    shared_runners_enabled: value.shared_runners_enabled?,
                    shared_with_groups: value.shared_with_groups?,
                    snippets_access_level: value.snippets_access_level?,
                    snippets_enabled: value.snippets_enabled?,
                    squash_commit_template: value.squash_commit_template?,
                    squash_option: value.squash_option?,
                    ssh_url_to_repo: value.ssh_url_to_repo?,
                    star_count: value.star_count?,
                    statistics: value.statistics?,
                    suggestion_commit_message: value.suggestion_commit_message?,
                    tag_list: value.tag_list?,
                    topics: value.topics?,
                    updated_at: value.updated_at?,
                    visibility: value.visibility?,
                    warn_about_potentially_unwanted_characters: value
                        .warn_about_potentially_unwanted_characters?,
                    web_url: value.web_url?,
                    wiki_access_level: value.wiki_access_level?,
                    wiki_enabled: value.wiki_enabled?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectWithAccess> for ApiEntitiesProjectWithAccess {
            fn from(value: super::ApiEntitiesProjectWithAccess) -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(value.allow_merge_on_skipped_pipeline),
                    allow_pipeline_trigger_approve_deployment: Ok(
                        value.allow_pipeline_trigger_approve_deployment
                    ),
                    analytics_access_level: Ok(value.analytics_access_level),
                    approvals_before_merge: Ok(value.approvals_before_merge),
                    archived: Ok(value.archived),
                    auto_cancel_pending_pipelines: Ok(value.auto_cancel_pending_pipelines),
                    auto_devops_deploy_strategy: Ok(value.auto_devops_deploy_strategy),
                    auto_devops_enabled: Ok(value.auto_devops_enabled),
                    autoclose_referenced_issues: Ok(value.autoclose_referenced_issues),
                    avatar_url: Ok(value.avatar_url),
                    build_git_strategy: Ok(value.build_git_strategy),
                    build_timeout: Ok(value.build_timeout),
                    builds_access_level: Ok(value.builds_access_level),
                    can_create_merge_request_in: Ok(value.can_create_merge_request_in),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(
                        value.ci_allow_fork_pipelines_to_run_in_parent_project
                    ),
                    ci_config_path: Ok(value.ci_config_path),
                    ci_default_git_depth: Ok(value.ci_default_git_depth),
                    ci_delete_pipelines_in_seconds: Ok(value.ci_delete_pipelines_in_seconds),
                    ci_forward_deployment_enabled: Ok(value.ci_forward_deployment_enabled),
                    ci_forward_deployment_rollback_allowed: Ok(
                        value.ci_forward_deployment_rollback_allowed
                    ),
                    ci_id_token_sub_claim_components: Ok(value.ci_id_token_sub_claim_components),
                    ci_job_token_scope_enabled: Ok(value.ci_job_token_scope_enabled),
                    ci_pipeline_variables_minimum_override_role: Ok(
                        value.ci_pipeline_variables_minimum_override_role
                    ),
                    ci_push_repository_for_job_token_allowed: Ok(
                        value.ci_push_repository_for_job_token_allowed
                    ),
                    ci_restrict_pipeline_cancellation_role: Ok(
                        value.ci_restrict_pipeline_cancellation_role
                    ),
                    ci_separated_caches: Ok(value.ci_separated_caches),
                    compliance_frameworks: Ok(value.compliance_frameworks),
                    container_expiration_policy: Ok(value.container_expiration_policy),
                    container_registry_access_level: Ok(value.container_registry_access_level),
                    container_registry_enabled: Ok(value.container_registry_enabled),
                    container_registry_image_prefix: Ok(value.container_registry_image_prefix),
                    created_at: Ok(value.created_at),
                    creator_id: Ok(value.creator_id),
                    custom_attributes: Ok(value.custom_attributes),
                    default_branch: Ok(value.default_branch),
                    description: Ok(value.description),
                    description_html: Ok(value.description_html),
                    emails_disabled: Ok(value.emails_disabled),
                    emails_enabled: Ok(value.emails_enabled),
                    empty_repo: Ok(value.empty_repo),
                    enforce_auth_checks_on_uploads: Ok(value.enforce_auth_checks_on_uploads),
                    environments_access_level: Ok(value.environments_access_level),
                    external_authorization_classification_label: Ok(
                        value.external_authorization_classification_label
                    ),
                    feature_flags_access_level: Ok(value.feature_flags_access_level),
                    forked_from_project: Ok(value.forked_from_project),
                    forking_access_level: Ok(value.forking_access_level),
                    forks_count: Ok(value.forks_count),
                    group_runners_enabled: Ok(value.group_runners_enabled),
                    http_url_to_repo: Ok(value.http_url_to_repo),
                    id: Ok(value.id),
                    import_error: Ok(value.import_error),
                    import_status: Ok(value.import_status),
                    import_type: Ok(value.import_type),
                    import_url: Ok(value.import_url),
                    infrastructure_access_level: Ok(value.infrastructure_access_level),
                    issue_branch_template: Ok(value.issue_branch_template),
                    issues_access_level: Ok(value.issues_access_level),
                    issues_enabled: Ok(value.issues_enabled),
                    issues_template: Ok(value.issues_template),
                    jobs_enabled: Ok(value.jobs_enabled),
                    keep_latest_artifact: Ok(value.keep_latest_artifact),
                    last_activity_at: Ok(value.last_activity_at),
                    lfs_enabled: Ok(value.lfs_enabled),
                    license: Ok(value.license),
                    license_url: Ok(value.license_url),
                    links: Ok(value.links),
                    marked_for_deletion_at: Ok(value.marked_for_deletion_at),
                    marked_for_deletion_on: Ok(value.marked_for_deletion_on),
                    max_artifacts_size: Ok(value.max_artifacts_size),
                    merge_commit_template: Ok(value.merge_commit_template),
                    merge_method: Ok(value.merge_method),
                    merge_pipelines_enabled: Ok(value.merge_pipelines_enabled),
                    merge_requests_access_level: Ok(value.merge_requests_access_level),
                    merge_requests_enabled: Ok(value.merge_requests_enabled),
                    merge_requests_template: Ok(value.merge_requests_template),
                    merge_trains_enabled: Ok(value.merge_trains_enabled),
                    merge_trains_skip_train_allowed: Ok(value.merge_trains_skip_train_allowed),
                    mirror: Ok(value.mirror),
                    mirror_overwrites_diverged_branches: Ok(
                        value.mirror_overwrites_diverged_branches
                    ),
                    mirror_trigger_builds: Ok(value.mirror_trigger_builds),
                    mirror_user_id: Ok(value.mirror_user_id),
                    model_experiments_access_level: Ok(value.model_experiments_access_level),
                    model_registry_access_level: Ok(value.model_registry_access_level),
                    monitor_access_level: Ok(value.monitor_access_level),
                    mr_default_target_self: Ok(value.mr_default_target_self),
                    name: Ok(value.name),
                    name_with_namespace: Ok(value.name_with_namespace),
                    namespace: Ok(value.namespace),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(
                        value.only_allow_merge_if_all_discussions_are_resolved
                    ),
                    only_allow_merge_if_all_status_checks_passed: Ok(
                        value.only_allow_merge_if_all_status_checks_passed
                    ),
                    only_allow_merge_if_pipeline_succeeds: Ok(
                        value.only_allow_merge_if_pipeline_succeeds
                    ),
                    only_mirror_protected_branches: Ok(value.only_mirror_protected_branches),
                    open_issues_count: Ok(value.open_issues_count),
                    owner: Ok(value.owner),
                    packages_enabled: Ok(value.packages_enabled),
                    pages_access_level: Ok(value.pages_access_level),
                    path: Ok(value.path),
                    path_with_namespace: Ok(value.path_with_namespace),
                    permissions: Ok(value.permissions),
                    pre_receive_secret_detection_enabled: Ok(
                        value.pre_receive_secret_detection_enabled
                    ),
                    prevent_merge_without_jira_issue: Ok(value.prevent_merge_without_jira_issue),
                    printing_merge_request_link_enabled: Ok(
                        value.printing_merge_request_link_enabled
                    ),
                    public_jobs: Ok(value.public_jobs),
                    readme_url: Ok(value.readme_url),
                    releases_access_level: Ok(value.releases_access_level),
                    remove_source_branch_after_merge: Ok(value.remove_source_branch_after_merge),
                    repository_access_level: Ok(value.repository_access_level),
                    repository_object_format: Ok(value.repository_object_format),
                    repository_storage: Ok(value.repository_storage),
                    request_access_enabled: Ok(value.request_access_enabled),
                    requirements_access_level: Ok(value.requirements_access_level),
                    requirements_enabled: Ok(value.requirements_enabled),
                    resolve_outdated_diff_discussions: Ok(value.resolve_outdated_diff_discussions),
                    restrict_user_defined_variables: Ok(value.restrict_user_defined_variables),
                    runner_token_expiration_interval: Ok(value.runner_token_expiration_interval),
                    runners_token: Ok(value.runners_token),
                    secret_push_protection_enabled: Ok(value.secret_push_protection_enabled),
                    security_and_compliance_access_level: Ok(
                        value.security_and_compliance_access_level
                    ),
                    security_and_compliance_enabled: Ok(value.security_and_compliance_enabled),
                    service_desk_address: Ok(value.service_desk_address),
                    service_desk_enabled: Ok(value.service_desk_enabled),
                    shared_runners_enabled: Ok(value.shared_runners_enabled),
                    shared_with_groups: Ok(value.shared_with_groups),
                    snippets_access_level: Ok(value.snippets_access_level),
                    snippets_enabled: Ok(value.snippets_enabled),
                    squash_commit_template: Ok(value.squash_commit_template),
                    squash_option: Ok(value.squash_option),
                    ssh_url_to_repo: Ok(value.ssh_url_to_repo),
                    star_count: Ok(value.star_count),
                    statistics: Ok(value.statistics),
                    suggestion_commit_message: Ok(value.suggestion_commit_message),
                    tag_list: Ok(value.tag_list),
                    topics: Ok(value.topics),
                    updated_at: Ok(value.updated_at),
                    visibility: Ok(value.visibility),
                    warn_about_potentially_unwanted_characters: Ok(
                        value.warn_about_potentially_unwanted_characters
                    ),
                    web_url: Ok(value.web_url),
                    wiki_access_level: Ok(value.wiki_access_level),
                    wiki_enabled: Ok(value.wiki_enabled),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectWithAccessLinks {
            cluster_agents: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            events: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            members: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_requests: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            repo_branches: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            self_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesProjectWithAccessLinks {
            fn default() -> Self {
                Self {
                    cluster_agents: Ok(Default::default()),
                    events: Ok(Default::default()),
                    issues: Ok(Default::default()),
                    labels: Ok(Default::default()),
                    members: Ok(Default::default()),
                    merge_requests: Ok(Default::default()),
                    repo_branches: Ok(Default::default()),
                    self_: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectWithAccessLinks {
            pub fn cluster_agents<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cluster_agents = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cluster_agents: {}", e)
                });
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn issues<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for issues: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn members<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.members = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for members: {}", e));
                self
            }
            pub fn merge_requests<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests = value.try_into().map_err(|e| {
                    format!("error converting supplied value for merge_requests: {}", e)
                });
                self
            }
            pub fn repo_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repo_branches = value.try_into().map_err(|e| {
                    format!("error converting supplied value for repo_branches: {}", e)
                });
                self
            }
            pub fn self_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.self_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for self_: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectWithAccessLinks>
            for super::ApiEntitiesProjectWithAccessLinks
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectWithAccessLinks,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cluster_agents: value.cluster_agents?,
                    events: value.events?,
                    issues: value.issues?,
                    labels: value.labels?,
                    members: value.members?,
                    merge_requests: value.merge_requests?,
                    repo_branches: value.repo_branches?,
                    self_: value.self_?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectWithAccessLinks>
            for ApiEntitiesProjectWithAccessLinks
        {
            fn from(value: super::ApiEntitiesProjectWithAccessLinks) -> Self {
                Self {
                    cluster_agents: Ok(value.cluster_agents),
                    events: Ok(value.events),
                    issues: Ok(value.issues),
                    labels: Ok(value.labels),
                    members: Ok(value.members),
                    merge_requests: Ok(value.merge_requests),
                    repo_branches: Ok(value.repo_branches),
                    self_: Ok(value.self_),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesProjectWithAccessPermissions {
            group_access: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesGroupAccess>,
                ::std::string::String,
            >,
            project_access: ::std::result::Result<
                ::std::option::Option<super::ApiEntitiesProjectAccess>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesProjectWithAccessPermissions {
            fn default() -> Self {
                Self {
                    group_access: Ok(Default::default()),
                    project_access: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesProjectWithAccessPermissions {
            pub fn group_access<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesGroupAccess>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_access = value.try_into().map_err(|e| {
                    format!("error converting supplied value for group_access: {}", e)
                });
                self
            }
            pub fn project_access<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiEntitiesProjectAccess>>,
                T::Error: ::std::fmt::Display,
            {
                self.project_access = value.try_into().map_err(|e| {
                    format!("error converting supplied value for project_access: {}", e)
                });
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesProjectWithAccessPermissions>
            for super::ApiEntitiesProjectWithAccessPermissions
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesProjectWithAccessPermissions,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    group_access: value.group_access?,
                    project_access: value.project_access?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesProjectWithAccessPermissions>
            for ApiEntitiesProjectWithAccessPermissions
        {
            fn from(value: super::ApiEntitiesProjectWithAccessPermissions) -> Self {
                Self {
                    group_access: Ok(value.group_access),
                    project_access: Ok(value.project_access),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesPushEventPayload {
            action: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            commit_from: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit_title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit_to: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ref_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ref_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ref_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesPushEventPayload {
            fn default() -> Self {
                Self {
                    action: Ok(Default::default()),
                    commit_count: Ok(Default::default()),
                    commit_from: Ok(Default::default()),
                    commit_title: Ok(Default::default()),
                    commit_to: Ok(Default::default()),
                    ref_: Ok(Default::default()),
                    ref_count: Ok(Default::default()),
                    ref_type: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesPushEventPayload {
            pub fn action<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action: {}", e));
                self
            }
            pub fn commit_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_count = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commit_count: {}", e)
                });
                self
            }
            pub fn commit_from<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_from = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit_from: {}", e));
                self
            }
            pub fn commit_title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_title = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commit_title: {}", e)
                });
                self
            }
            pub fn commit_to<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_to = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit_to: {}", e));
                self
            }
            pub fn ref_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ref_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_: {}", e));
                self
            }
            pub fn ref_count<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ref_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_count: {}", e));
                self
            }
            pub fn ref_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ref_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_type: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesPushEventPayload> for super::ApiEntitiesPushEventPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesPushEventPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    action: value.action?,
                    commit_count: value.commit_count?,
                    commit_from: value.commit_from?,
                    commit_title: value.commit_title?,
                    commit_to: value.commit_to?,
                    ref_: value.ref_?,
                    ref_count: value.ref_count?,
                    ref_type: value.ref_type?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesPushEventPayload> for ApiEntitiesPushEventPayload {
            fn from(value: super::ApiEntitiesPushEventPayload) -> Self {
                Self {
                    action: Ok(value.action),
                    commit_count: Ok(value.commit_count),
                    commit_from: Ok(value.commit_from),
                    commit_title: Ok(value.commit_title),
                    commit_to: Ok(value.commit_to),
                    ref_: Ok(value.ref_),
                    ref_count: Ok(value.ref_count),
                    ref_type: Ok(value.ref_type),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesUserBasic {
            avatar_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            avatar_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            custom_attributes: ::std::result::Result<
                ::std::vec::Vec<super::ApiEntitiesCustomAttribute>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            locked: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            username: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            web_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesUserBasic {
            fn default() -> Self {
                Self {
                    avatar_path: Ok(Default::default()),
                    avatar_url: Ok(Default::default()),
                    custom_attributes: Ok(Default::default()),
                    id: Ok(Default::default()),
                    locked: Ok(Default::default()),
                    name: Ok(Default::default()),
                    state: Ok(Default::default()),
                    username: Ok(Default::default()),
                    web_url: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesUserBasic {
            pub fn avatar_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_path: {}", e));
                self
            }
            pub fn avatar_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar_url: {}", e));
                self
            }
            pub fn custom_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ApiEntitiesCustomAttribute>>,
                T::Error: ::std::fmt::Display,
            {
                self.custom_attributes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for custom_attributes: {}",
                        e
                    )
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn locked<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.locked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for locked: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn username<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.username = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for username: {}", e));
                self
            }
            pub fn web_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.web_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_url: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesUserBasic> for super::ApiEntitiesUserBasic {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesUserBasic,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    avatar_path: value.avatar_path?,
                    avatar_url: value.avatar_url?,
                    custom_attributes: value.custom_attributes?,
                    id: value.id?,
                    locked: value.locked?,
                    name: value.name?,
                    state: value.state?,
                    username: value.username?,
                    web_url: value.web_url?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesUserBasic> for ApiEntitiesUserBasic {
            fn from(value: super::ApiEntitiesUserBasic) -> Self {
                Self {
                    avatar_path: Ok(value.avatar_path),
                    avatar_url: Ok(value.avatar_url),
                    custom_attributes: Ok(value.custom_attributes),
                    id: Ok(value.id),
                    locked: Ok(value.locked),
                    name: Ok(value.name),
                    state: Ok(value.state),
                    username: Ok(value.username),
                    web_url: Ok(value.web_url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiEntitiesWikiPageBasic {
            format: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            slug: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ApiEntitiesWikiPageBasic {
            fn default() -> Self {
                Self {
                    format: Ok(Default::default()),
                    slug: Ok(Default::default()),
                    title: Ok(Default::default()),
                }
            }
        }
        impl ApiEntitiesWikiPageBasic {
            pub fn format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.format = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for format: {}", e));
                self
            }
            pub fn slug<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.slug = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for slug: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiEntitiesWikiPageBasic> for super::ApiEntitiesWikiPageBasic {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiEntitiesWikiPageBasic,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    format: value.format?,
                    slug: value.slug?,
                    title: value.title?,
                })
            }
        }
        impl ::std::convert::From<super::ApiEntitiesWikiPageBasic> for ApiEntitiesWikiPageBasic {
            fn from(value: super::ApiEntitiesWikiPageBasic) -> Self {
                Self {
                    format: Ok(value.format),
                    slug: Ok(value.slug),
                    title: Ok(value.title),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PostApiV4ProjectsIdRepositoryCommits {
            actions: ::std::result::Result<
                ::std::vec::Vec<super::PostApiV4ProjectsIdRepositoryCommitsActionsItem>,
                ::std::string::String,
            >,
            author_email: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            author_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            commit_message: ::std::result::Result<::std::string::String, ::std::string::String>,
            force: ::std::result::Result<bool, ::std::string::String>,
            start_branch: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            start_project: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            start_sha: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            stats: ::std::result::Result<bool, ::std::string::String>,
        }
        impl ::std::default::Default for PostApiV4ProjectsIdRepositoryCommits {
            fn default() -> Self {
                Self {
                    actions: Err("no value supplied for actions".to_string()),
                    author_email: Ok(Default::default()),
                    author_name: Ok(Default::default()),
                    branch: Err("no value supplied for branch".to_string()),
                    commit_message: Err("no value supplied for commit_message".to_string()),
                    force: Ok(Default::default()),
                    start_branch: Ok(Default::default()),
                    start_project: Ok(Default::default()),
                    start_sha: Ok(Default::default()),
                    stats: Ok(super::defaults::default_bool::<true>()),
                }
            }
        }
        impl PostApiV4ProjectsIdRepositoryCommits {
            pub fn actions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::PostApiV4ProjectsIdRepositoryCommitsActionsItem>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.actions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for actions: {}", e));
                self
            }
            pub fn author_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_email = value.try_into().map_err(|e| {
                    format!("error converting supplied value for author_email: {}", e)
                });
                self
            }
            pub fn author_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.author_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for author_name: {}", e));
                self
            }
            pub fn branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.branch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branch: {}", e));
                self
            }
            pub fn commit_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.commit_message = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commit_message: {}", e)
                });
                self
            }
            pub fn force<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.force = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for force: {}", e));
                self
            }
            pub fn start_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.start_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for start_branch: {}", e)
                });
                self
            }
            pub fn start_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.start_project = value.try_into().map_err(|e| {
                    format!("error converting supplied value for start_project: {}", e)
                });
                self
            }
            pub fn start_sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.start_sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_sha: {}", e));
                self
            }
            pub fn stats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.stats = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stats: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<PostApiV4ProjectsIdRepositoryCommits>
            for super::PostApiV4ProjectsIdRepositoryCommits
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostApiV4ProjectsIdRepositoryCommits,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    actions: value.actions?,
                    author_email: value.author_email?,
                    author_name: value.author_name?,
                    branch: value.branch?,
                    commit_message: value.commit_message?,
                    force: value.force?,
                    start_branch: value.start_branch?,
                    start_project: value.start_project?,
                    start_sha: value.start_sha?,
                    stats: value.stats?,
                })
            }
        }
        impl ::std::convert::From<super::PostApiV4ProjectsIdRepositoryCommits>
            for PostApiV4ProjectsIdRepositoryCommits
        {
            fn from(value: super::PostApiV4ProjectsIdRepositoryCommits) -> Self {
                Self {
                    actions: Ok(value.actions),
                    author_email: Ok(value.author_email),
                    author_name: Ok(value.author_name),
                    branch: Ok(value.branch),
                    commit_message: Ok(value.commit_message),
                    force: Ok(value.force),
                    start_branch: Ok(value.start_branch),
                    start_project: Ok(value.start_project),
                    start_sha: Ok(value.start_sha),
                    stats: Ok(value.stats),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PostApiV4ProjectsIdRepositoryCommitsActionsItem {
            action: ::std::result::Result<
                super::PostApiV4ProjectsIdRepositoryCommitsActionsItemAction,
                ::std::string::String,
            >,
            content: ::std::result::Result<::std::string::String, ::std::string::String>,
            encoding: ::std::result::Result<
                super::PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding,
                ::std::string::String,
            >,
            execute_filemode: ::std::result::Result<bool, ::std::string::String>,
            file_path: ::std::result::Result<::std::string::String, ::std::string::String>,
            last_commit_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            previous_path: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for PostApiV4ProjectsIdRepositoryCommitsActionsItem {
            fn default() -> Self {
                Self {
                    action: Err("no value supplied for action".to_string()),
                    content: Err("no value supplied for content".to_string()),
                    encoding: Ok(
                        super::defaults::post_api_v4_projects_id_repository_commits_actions_item_encoding(),
                    ),
                    execute_filemode: Err(
                        "no value supplied for execute_filemode".to_string(),
                    ),
                    file_path: Err("no value supplied for file_path".to_string()),
                    last_commit_id: Ok(Default::default()),
                    previous_path: Err("no value supplied for previous_path".to_string()),
                }
            }
        }
        impl PostApiV4ProjectsIdRepositoryCommitsActionsItem {
            pub fn action<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PostApiV4ProjectsIdRepositoryCommitsActionsItemAction,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action: {}", e));
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn encoding<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.encoding = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for encoding: {}", e));
                self
            }
            pub fn execute_filemode<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.execute_filemode = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for execute_filemode: {}",
                        e
                    )
                });
                self
            }
            pub fn file_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.file_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for file_path: {}", e));
                self
            }
            pub fn last_commit_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_commit_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_commit_id: {}", e)
                });
                self
            }
            pub fn previous_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_path = value.try_into().map_err(|e| {
                    format!("error converting supplied value for previous_path: {}", e)
                });
                self
            }
        }
        impl ::std::convert::TryFrom<PostApiV4ProjectsIdRepositoryCommitsActionsItem>
            for super::PostApiV4ProjectsIdRepositoryCommitsActionsItem
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PostApiV4ProjectsIdRepositoryCommitsActionsItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    action: value.action?,
                    content: value.content?,
                    encoding: value.encoding?,
                    execute_filemode: value.execute_filemode?,
                    file_path: value.file_path?,
                    last_commit_id: value.last_commit_id?,
                    previous_path: value.previous_path?,
                })
            }
        }
        impl ::std::convert::From<super::PostApiV4ProjectsIdRepositoryCommitsActionsItem>
            for PostApiV4ProjectsIdRepositoryCommitsActionsItem
        {
            fn from(value: super::PostApiV4ProjectsIdRepositoryCommitsActionsItem) -> Self {
                Self {
                    action: Ok(value.action),
                    content: Ok(value.content),
                    encoding: Ok(value.encoding),
                    execute_filemode: Ok(value.execute_filemode),
                    file_path: Ok(value.file_path),
                    last_commit_id: Ok(value.last_commit_id),
                    previous_path: Ok(value.previous_path),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PutApiV4ProjectsId {
            allow_merge_on_skipped_pipeline:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            allow_pipeline_trigger_approve_deployment:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            analytics_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdAnalyticsAccessLevel>,
                ::std::string::String,
            >,
            approvals_before_merge:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            auto_cancel_pending_pipelines: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdAutoCancelPendingPipelines>,
                ::std::string::String,
            >,
            auto_devops_deploy_strategy: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdAutoDevopsDeployStrategy>,
                ::std::string::String,
            >,
            auto_devops_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            autoclose_referenced_issues:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            avatar: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            build_git_strategy: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdBuildGitStrategy>,
                ::std::string::String,
            >,
            build_timeout: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            builds_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdBuildsAccessLevel>,
                ::std::string::String,
            >,
            ci_allow_fork_pipelines_to_run_in_parent_project:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_config_path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_default_git_depth:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_delete_pipelines_in_seconds:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            ci_forward_deployment_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_forward_deployment_rollback_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_id_token_sub_claim_components: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            ci_pipeline_variables_minimum_override_role: ::std::result::Result<
                ::std::option::Option<
                    super::PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole,
                >,
                ::std::string::String,
            >,
            ci_push_repository_for_job_token_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ci_restrict_pipeline_cancellation_role: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ci_separated_caches:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            container_expiration_policy_attributes: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdContainerExpirationPolicyAttributes>,
                ::std::string::String,
            >,
            container_registry_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdContainerRegistryAccessLevel>,
                ::std::string::String,
            >,
            container_registry_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            default_branch: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            emails_disabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            emails_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            enforce_auth_checks_on_uploads:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            environments_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdEnvironmentsAccessLevel>,
                ::std::string::String,
            >,
            external_authorization_classification_label: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fallback_approvals_required:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            feature_flags_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdFeatureFlagsAccessLevel>,
                ::std::string::String,
            >,
            forking_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdForkingAccessLevel>,
                ::std::string::String,
            >,
            group_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            import_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            infrastructure_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdInfrastructureAccessLevel>,
                ::std::string::String,
            >,
            issue_branch_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            issues_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdIssuesAccessLevel>,
                ::std::string::String,
            >,
            issues_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            issues_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            jobs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            keep_latest_artifact:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            lfs_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            max_artifacts_size:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            merge_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_method: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdMergeMethod>,
                ::std::string::String,
            >,
            merge_pipelines_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdMergeRequestsAccessLevel>,
                ::std::string::String,
            >,
            merge_requests_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_requests_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            merge_trains_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            merge_trains_skip_train_allowed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror_branch_regex: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mirror_overwrites_diverged_branches:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror_trigger_builds:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            mirror_user_id:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            model_experiments_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdModelExperimentsAccessLevel>,
                ::std::string::String,
            >,
            model_registry_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdModelRegistryAccessLevel>,
                ::std::string::String,
            >,
            monitor_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdMonitorAccessLevel>,
                ::std::string::String,
            >,
            mr_default_target_self:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            only_allow_merge_if_all_discussions_are_resolved:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_all_status_checks_passed:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_allow_merge_if_pipeline_succeeds:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            only_mirror_protected_branches:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            packages_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            pages_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdPagesAccessLevel>,
                ::std::string::String,
            >,
            path: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            prevent_merge_without_jira_issue:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            printing_merge_request_link_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            public_builds:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            public_jobs: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            releases_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdReleasesAccessLevel>,
                ::std::string::String,
            >,
            remove_source_branch_after_merge:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            repository_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdRepositoryAccessLevel>,
                ::std::string::String,
            >,
            repository_storage: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            request_access_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            requirements_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdRequirementsAccessLevel>,
                ::std::string::String,
            >,
            resolve_outdated_diff_discussions:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            restrict_user_defined_variables:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            security_and_compliance_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdSecurityAndComplianceAccessLevel>,
                ::std::string::String,
            >,
            service_desk_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            shared_runners_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            show_default_award_emojis:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            show_diff_preview_in_email:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            snippets_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdSnippetsAccessLevel>,
                ::std::string::String,
            >,
            snippets_enabled:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            squash_commit_template: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            squash_option: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdSquashOption>,
                ::std::string::String,
            >,
            suggestion_commit_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tag_list: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            topics: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdVisibility>,
                ::std::string::String,
            >,
            warn_about_potentially_unwanted_characters:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            wiki_access_level: ::std::result::Result<
                ::std::option::Option<super::PutApiV4ProjectsIdWikiAccessLevel>,
                ::std::string::String,
            >,
            wiki_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        }
        impl ::std::default::Default for PutApiV4ProjectsId {
            fn default() -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(Default::default()),
                    allow_pipeline_trigger_approve_deployment: Ok(Default::default()),
                    analytics_access_level: Ok(Default::default()),
                    approvals_before_merge: Ok(Default::default()),
                    auto_cancel_pending_pipelines: Ok(Default::default()),
                    auto_devops_deploy_strategy: Ok(Default::default()),
                    auto_devops_enabled: Ok(Default::default()),
                    autoclose_referenced_issues: Ok(Default::default()),
                    avatar: Ok(Default::default()),
                    build_git_strategy: Ok(Default::default()),
                    build_timeout: Ok(Default::default()),
                    builds_access_level: Ok(Default::default()),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(Default::default()),
                    ci_config_path: Ok(Default::default()),
                    ci_default_git_depth: Ok(Default::default()),
                    ci_delete_pipelines_in_seconds: Ok(Default::default()),
                    ci_forward_deployment_enabled: Ok(Default::default()),
                    ci_forward_deployment_rollback_allowed: Ok(Default::default()),
                    ci_id_token_sub_claim_components: Ok(Default::default()),
                    ci_pipeline_variables_minimum_override_role: Ok(Default::default()),
                    ci_push_repository_for_job_token_allowed: Ok(Default::default()),
                    ci_restrict_pipeline_cancellation_role: Ok(Default::default()),
                    ci_separated_caches: Ok(Default::default()),
                    container_expiration_policy_attributes: Ok(Default::default()),
                    container_registry_access_level: Ok(Default::default()),
                    container_registry_enabled: Ok(Default::default()),
                    default_branch: Ok(Default::default()),
                    description: Ok(Default::default()),
                    emails_disabled: Ok(Default::default()),
                    emails_enabled: Ok(Default::default()),
                    enforce_auth_checks_on_uploads: Ok(Default::default()),
                    environments_access_level: Ok(Default::default()),
                    external_authorization_classification_label: Ok(Default::default()),
                    fallback_approvals_required: Ok(Default::default()),
                    feature_flags_access_level: Ok(Default::default()),
                    forking_access_level: Ok(Default::default()),
                    group_runners_enabled: Ok(Default::default()),
                    import_url: Ok(Default::default()),
                    infrastructure_access_level: Ok(Default::default()),
                    issue_branch_template: Ok(Default::default()),
                    issues_access_level: Ok(Default::default()),
                    issues_enabled: Ok(Default::default()),
                    issues_template: Ok(Default::default()),
                    jobs_enabled: Ok(Default::default()),
                    keep_latest_artifact: Ok(Default::default()),
                    lfs_enabled: Ok(Default::default()),
                    max_artifacts_size: Ok(Default::default()),
                    merge_commit_template: Ok(Default::default()),
                    merge_method: Ok(Default::default()),
                    merge_pipelines_enabled: Ok(Default::default()),
                    merge_requests_access_level: Ok(Default::default()),
                    merge_requests_enabled: Ok(Default::default()),
                    merge_requests_template: Ok(Default::default()),
                    merge_trains_enabled: Ok(Default::default()),
                    merge_trains_skip_train_allowed: Ok(Default::default()),
                    mirror: Ok(Default::default()),
                    mirror_branch_regex: Ok(Default::default()),
                    mirror_overwrites_diverged_branches: Ok(Default::default()),
                    mirror_trigger_builds: Ok(Default::default()),
                    mirror_user_id: Ok(Default::default()),
                    model_experiments_access_level: Ok(Default::default()),
                    model_registry_access_level: Ok(Default::default()),
                    monitor_access_level: Ok(Default::default()),
                    mr_default_target_self: Ok(Default::default()),
                    name: Ok(Default::default()),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(Default::default()),
                    only_allow_merge_if_all_status_checks_passed: Ok(Default::default()),
                    only_allow_merge_if_pipeline_succeeds: Ok(Default::default()),
                    only_mirror_protected_branches: Ok(Default::default()),
                    packages_enabled: Ok(Default::default()),
                    pages_access_level: Ok(Default::default()),
                    path: Ok(Default::default()),
                    prevent_merge_without_jira_issue: Ok(Default::default()),
                    printing_merge_request_link_enabled: Ok(Default::default()),
                    public_builds: Ok(Default::default()),
                    public_jobs: Ok(Default::default()),
                    releases_access_level: Ok(Default::default()),
                    remove_source_branch_after_merge: Ok(Default::default()),
                    repository_access_level: Ok(Default::default()),
                    repository_storage: Ok(Default::default()),
                    request_access_enabled: Ok(Default::default()),
                    requirements_access_level: Ok(Default::default()),
                    resolve_outdated_diff_discussions: Ok(Default::default()),
                    restrict_user_defined_variables: Ok(Default::default()),
                    security_and_compliance_access_level: Ok(Default::default()),
                    service_desk_enabled: Ok(Default::default()),
                    shared_runners_enabled: Ok(Default::default()),
                    show_default_award_emojis: Ok(Default::default()),
                    show_diff_preview_in_email: Ok(Default::default()),
                    snippets_access_level: Ok(Default::default()),
                    snippets_enabled: Ok(Default::default()),
                    squash_commit_template: Ok(Default::default()),
                    squash_option: Ok(Default::default()),
                    suggestion_commit_message: Ok(Default::default()),
                    tag_list: Ok(Default::default()),
                    topics: Ok(Default::default()),
                    visibility: Ok(Default::default()),
                    warn_about_potentially_unwanted_characters: Ok(Default::default()),
                    wiki_access_level: Ok(Default::default()),
                    wiki_enabled: Ok(Default::default()),
                }
            }
        }
        impl PutApiV4ProjectsId {
            pub fn allow_merge_on_skipped_pipeline<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_merge_on_skipped_pipeline = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for allow_merge_on_skipped_pipeline: {}",
                        e
                    )
                });
                self
            }
            pub fn allow_pipeline_trigger_approve_deployment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.allow_pipeline_trigger_approve_deployment = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for allow_pipeline_trigger_approve_deployment: {}",
                            e
                        )
                    });
                self
            }
            pub fn analytics_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdAnalyticsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.analytics_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for analytics_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn approvals_before_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.approvals_before_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for approvals_before_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_cancel_pending_pipelines<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdAutoCancelPendingPipelines>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.auto_cancel_pending_pipelines = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_cancel_pending_pipelines: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_deploy_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdAutoDevopsDeployStrategy>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_deploy_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_deploy_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_devops_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_devops_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_devops_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn autoclose_referenced_issues<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.autoclose_referenced_issues = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for autoclose_referenced_issues: {}",
                        e
                    )
                });
                self
            }
            pub fn avatar<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.avatar = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for avatar: {}", e));
                self
            }
            pub fn build_git_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdBuildGitStrategy>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.build_git_strategy = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for build_git_strategy: {}",
                        e
                    )
                });
                self
            }
            pub fn build_timeout<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.build_timeout = value.try_into().map_err(|e| {
                    format!("error converting supplied value for build_timeout: {}", e)
                });
                self
            }
            pub fn builds_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdBuildsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.builds_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for builds_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_allow_fork_pipelines_to_run_in_parent_project<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_allow_fork_pipelines_to_run_in_parent_project = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_allow_fork_pipelines_to_run_in_parent_project: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_config_path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_config_path = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ci_config_path: {}", e)
                });
                self
            }
            pub fn ci_default_git_depth<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_default_git_depth = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_default_git_depth: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_delete_pipelines_in_seconds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_delete_pipelines_in_seconds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_delete_pipelines_in_seconds: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_forward_deployment_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_forward_deployment_rollback_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_forward_deployment_rollback_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_forward_deployment_rollback_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_id_token_sub_claim_components<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_id_token_sub_claim_components = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_id_token_sub_claim_components: {}",
                        e
                    )
                });
                self
            }
            pub fn ci_pipeline_variables_minimum_override_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<
                        super::PutApiV4ProjectsIdCiPipelineVariablesMinimumOverrideRole,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.ci_pipeline_variables_minimum_override_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_pipeline_variables_minimum_override_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_push_repository_for_job_token_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_push_repository_for_job_token_allowed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_push_repository_for_job_token_allowed: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_restrict_pipeline_cancellation_role<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_restrict_pipeline_cancellation_role = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for ci_restrict_pipeline_cancellation_role: {}",
                            e
                        )
                    });
                self
            }
            pub fn ci_separated_caches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.ci_separated_caches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ci_separated_caches: {}",
                        e
                    )
                });
                self
            }
            pub fn container_expiration_policy_attributes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<
                        super::PutApiV4ProjectsIdContainerExpirationPolicyAttributes,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.container_expiration_policy_attributes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for container_expiration_policy_attributes: {}",
                            e
                        )
                    });
                self
            }
            pub fn container_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdContainerRegistryAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn container_registry_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.container_registry_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for container_registry_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn emails_disabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_disabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_disabled: {}", e)
                });
                self
            }
            pub fn emails_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for emails_enabled: {}", e)
                });
                self
            }
            pub fn enforce_auth_checks_on_uploads<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.enforce_auth_checks_on_uploads = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for enforce_auth_checks_on_uploads: {}",
                        e
                    )
                });
                self
            }
            pub fn environments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdEnvironmentsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.environments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for environments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn external_authorization_classification_label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.external_authorization_classification_label = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for external_authorization_classification_label: {}",
                            e
                        )
                    });
                self
            }
            pub fn fallback_approvals_required<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.fallback_approvals_required = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for fallback_approvals_required: {}",
                        e
                    )
                });
                self
            }
            pub fn feature_flags_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdFeatureFlagsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.feature_flags_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for feature_flags_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn forking_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdForkingAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.forking_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for forking_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn group_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for group_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn import_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.import_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for import_url: {}", e));
                self
            }
            pub fn infrastructure_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdInfrastructureAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.infrastructure_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for infrastructure_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issue_branch_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issue_branch_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issue_branch_template: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdIssuesAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.issues_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for issues_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn issues_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_enabled: {}", e)
                });
                self
            }
            pub fn issues_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.issues_template = value.try_into().map_err(|e| {
                    format!("error converting supplied value for issues_template: {}", e)
                });
                self
            }
            pub fn jobs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.jobs_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for jobs_enabled: {}", e)
                });
                self
            }
            pub fn keep_latest_artifact<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.keep_latest_artifact = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for keep_latest_artifact: {}",
                        e
                    )
                });
                self
            }
            pub fn lfs_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.lfs_enabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lfs_enabled: {}", e));
                self
            }
            pub fn max_artifacts_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_artifacts_size = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for max_artifacts_size: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdMergeMethod>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.merge_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for merge_method: {}", e)
                });
                self
            }
            pub fn merge_pipelines_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_pipelines_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_pipelines_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdMergeRequestsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_requests_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_requests_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_requests_template: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn merge_trains_skip_train_allowed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.merge_trains_skip_train_allowed = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for merge_trains_skip_train_allowed: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mirror: {}", e));
                self
            }
            pub fn mirror_branch_regex<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_branch_regex = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mirror_branch_regex: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror_overwrites_diverged_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_overwrites_diverged_branches = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mirror_overwrites_diverged_branches: {}",
                            e
                        )
                    });
                self
            }
            pub fn mirror_trigger_builds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_trigger_builds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mirror_trigger_builds: {}",
                        e
                    )
                });
                self
            }
            pub fn mirror_user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mirror_user_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mirror_user_id: {}", e)
                });
                self
            }
            pub fn model_experiments_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdModelExperimentsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.model_experiments_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_experiments_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn model_registry_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdModelRegistryAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.model_registry_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for model_registry_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn monitor_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdMonitorAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.monitor_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for monitor_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn mr_default_target_self<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.mr_default_target_self = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for mr_default_target_self: {}",
                        e
                    )
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn only_allow_merge_if_all_discussions_are_resolved<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_discussions_are_resolved = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_discussions_are_resolved: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_all_status_checks_passed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_all_status_checks_passed = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_all_status_checks_passed: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_allow_merge_if_pipeline_succeeds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_allow_merge_if_pipeline_succeeds = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for only_allow_merge_if_pipeline_succeeds: {}",
                            e
                        )
                    });
                self
            }
            pub fn only_mirror_protected_branches<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.only_mirror_protected_branches = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for only_mirror_protected_branches: {}",
                        e
                    )
                });
                self
            }
            pub fn packages_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.packages_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for packages_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn pages_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdPagesAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.pages_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for pages_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn prevent_merge_without_jira_issue<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.prevent_merge_without_jira_issue = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for prevent_merge_without_jira_issue: {}",
                        e
                    )
                });
                self
            }
            pub fn printing_merge_request_link_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.printing_merge_request_link_enabled = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for printing_merge_request_link_enabled: {}",
                            e
                        )
                    });
                self
            }
            pub fn public_builds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.public_builds = value.try_into().map_err(|e| {
                    format!("error converting supplied value for public_builds: {}", e)
                });
                self
            }
            pub fn public_jobs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.public_jobs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for public_jobs: {}", e));
                self
            }
            pub fn releases_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdReleasesAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.releases_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for releases_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn remove_source_branch_after_merge<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.remove_source_branch_after_merge = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for remove_source_branch_after_merge: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdRepositoryAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.repository_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn repository_storage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.repository_storage = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for repository_storage: {}",
                        e
                    )
                });
                self
            }
            pub fn request_access_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.request_access_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for request_access_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn requirements_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdRequirementsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.requirements_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for requirements_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn resolve_outdated_diff_discussions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.resolve_outdated_diff_discussions = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for resolve_outdated_diff_discussions: {}",
                        e
                    )
                });
                self
            }
            pub fn restrict_user_defined_variables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.restrict_user_defined_variables = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for restrict_user_defined_variables: {}",
                        e
                    )
                });
                self
            }
            pub fn security_and_compliance_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<
                        super::PutApiV4ProjectsIdSecurityAndComplianceAccessLevel,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.security_and_compliance_access_level = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for security_and_compliance_access_level: {}",
                            e
                        )
                    });
                self
            }
            pub fn service_desk_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.service_desk_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for service_desk_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn shared_runners_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.shared_runners_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for shared_runners_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn show_default_award_emojis<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.show_default_award_emojis = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for show_default_award_emojis: {}",
                        e
                    )
                });
                self
            }
            pub fn show_diff_preview_in_email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.show_diff_preview_in_email = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for show_diff_preview_in_email: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdSnippetsAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn snippets_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.snippets_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snippets_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_commit_template<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.squash_commit_template = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for squash_commit_template: {}",
                        e
                    )
                });
                self
            }
            pub fn squash_option<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdSquashOption>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.squash_option = value.try_into().map_err(|e| {
                    format!("error converting supplied value for squash_option: {}", e)
                });
                self
            }
            pub fn suggestion_commit_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suggestion_commit_message = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for suggestion_commit_message: {}",
                        e
                    )
                });
                self
            }
            pub fn tag_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tag_list = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag_list: {}", e));
                self
            }
            pub fn topics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.topics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for topics: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdVisibility>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
            pub fn warn_about_potentially_unwanted_characters<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.warn_about_potentially_unwanted_characters = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for warn_about_potentially_unwanted_characters: {}",
                            e
                        )
                    });
                self
            }
            pub fn wiki_access_level<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PutApiV4ProjectsIdWikiAccessLevel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_access_level = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for wiki_access_level: {}",
                        e
                    )
                });
                self
            }
            pub fn wiki_enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.wiki_enabled = value.try_into().map_err(|e| {
                    format!("error converting supplied value for wiki_enabled: {}", e)
                });
                self
            }
        }
        impl ::std::convert::TryFrom<PutApiV4ProjectsId> for super::PutApiV4ProjectsId {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PutApiV4ProjectsId,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    allow_merge_on_skipped_pipeline: value.allow_merge_on_skipped_pipeline?,
                    allow_pipeline_trigger_approve_deployment: value
                        .allow_pipeline_trigger_approve_deployment?,
                    analytics_access_level: value.analytics_access_level?,
                    approvals_before_merge: value.approvals_before_merge?,
                    auto_cancel_pending_pipelines: value.auto_cancel_pending_pipelines?,
                    auto_devops_deploy_strategy: value.auto_devops_deploy_strategy?,
                    auto_devops_enabled: value.auto_devops_enabled?,
                    autoclose_referenced_issues: value.autoclose_referenced_issues?,
                    avatar: value.avatar?,
                    build_git_strategy: value.build_git_strategy?,
                    build_timeout: value.build_timeout?,
                    builds_access_level: value.builds_access_level?,
                    ci_allow_fork_pipelines_to_run_in_parent_project: value
                        .ci_allow_fork_pipelines_to_run_in_parent_project?,
                    ci_config_path: value.ci_config_path?,
                    ci_default_git_depth: value.ci_default_git_depth?,
                    ci_delete_pipelines_in_seconds: value.ci_delete_pipelines_in_seconds?,
                    ci_forward_deployment_enabled: value.ci_forward_deployment_enabled?,
                    ci_forward_deployment_rollback_allowed: value
                        .ci_forward_deployment_rollback_allowed?,
                    ci_id_token_sub_claim_components: value.ci_id_token_sub_claim_components?,
                    ci_pipeline_variables_minimum_override_role: value
                        .ci_pipeline_variables_minimum_override_role?,
                    ci_push_repository_for_job_token_allowed: value
                        .ci_push_repository_for_job_token_allowed?,
                    ci_restrict_pipeline_cancellation_role: value
                        .ci_restrict_pipeline_cancellation_role?,
                    ci_separated_caches: value.ci_separated_caches?,
                    container_expiration_policy_attributes: value
                        .container_expiration_policy_attributes?,
                    container_registry_access_level: value.container_registry_access_level?,
                    container_registry_enabled: value.container_registry_enabled?,
                    default_branch: value.default_branch?,
                    description: value.description?,
                    emails_disabled: value.emails_disabled?,
                    emails_enabled: value.emails_enabled?,
                    enforce_auth_checks_on_uploads: value.enforce_auth_checks_on_uploads?,
                    environments_access_level: value.environments_access_level?,
                    external_authorization_classification_label: value
                        .external_authorization_classification_label?,
                    fallback_approvals_required: value.fallback_approvals_required?,
                    feature_flags_access_level: value.feature_flags_access_level?,
                    forking_access_level: value.forking_access_level?,
                    group_runners_enabled: value.group_runners_enabled?,
                    import_url: value.import_url?,
                    infrastructure_access_level: value.infrastructure_access_level?,
                    issue_branch_template: value.issue_branch_template?,
                    issues_access_level: value.issues_access_level?,
                    issues_enabled: value.issues_enabled?,
                    issues_template: value.issues_template?,
                    jobs_enabled: value.jobs_enabled?,
                    keep_latest_artifact: value.keep_latest_artifact?,
                    lfs_enabled: value.lfs_enabled?,
                    max_artifacts_size: value.max_artifacts_size?,
                    merge_commit_template: value.merge_commit_template?,
                    merge_method: value.merge_method?,
                    merge_pipelines_enabled: value.merge_pipelines_enabled?,
                    merge_requests_access_level: value.merge_requests_access_level?,
                    merge_requests_enabled: value.merge_requests_enabled?,
                    merge_requests_template: value.merge_requests_template?,
                    merge_trains_enabled: value.merge_trains_enabled?,
                    merge_trains_skip_train_allowed: value.merge_trains_skip_train_allowed?,
                    mirror: value.mirror?,
                    mirror_branch_regex: value.mirror_branch_regex?,
                    mirror_overwrites_diverged_branches: value
                        .mirror_overwrites_diverged_branches?,
                    mirror_trigger_builds: value.mirror_trigger_builds?,
                    mirror_user_id: value.mirror_user_id?,
                    model_experiments_access_level: value.model_experiments_access_level?,
                    model_registry_access_level: value.model_registry_access_level?,
                    monitor_access_level: value.monitor_access_level?,
                    mr_default_target_self: value.mr_default_target_self?,
                    name: value.name?,
                    only_allow_merge_if_all_discussions_are_resolved: value
                        .only_allow_merge_if_all_discussions_are_resolved?,
                    only_allow_merge_if_all_status_checks_passed: value
                        .only_allow_merge_if_all_status_checks_passed?,
                    only_allow_merge_if_pipeline_succeeds: value
                        .only_allow_merge_if_pipeline_succeeds?,
                    only_mirror_protected_branches: value.only_mirror_protected_branches?,
                    packages_enabled: value.packages_enabled?,
                    pages_access_level: value.pages_access_level?,
                    path: value.path?,
                    prevent_merge_without_jira_issue: value.prevent_merge_without_jira_issue?,
                    printing_merge_request_link_enabled: value
                        .printing_merge_request_link_enabled?,
                    public_builds: value.public_builds?,
                    public_jobs: value.public_jobs?,
                    releases_access_level: value.releases_access_level?,
                    remove_source_branch_after_merge: value.remove_source_branch_after_merge?,
                    repository_access_level: value.repository_access_level?,
                    repository_storage: value.repository_storage?,
                    request_access_enabled: value.request_access_enabled?,
                    requirements_access_level: value.requirements_access_level?,
                    resolve_outdated_diff_discussions: value.resolve_outdated_diff_discussions?,
                    restrict_user_defined_variables: value.restrict_user_defined_variables?,
                    security_and_compliance_access_level: value
                        .security_and_compliance_access_level?,
                    service_desk_enabled: value.service_desk_enabled?,
                    shared_runners_enabled: value.shared_runners_enabled?,
                    show_default_award_emojis: value.show_default_award_emojis?,
                    show_diff_preview_in_email: value.show_diff_preview_in_email?,
                    snippets_access_level: value.snippets_access_level?,
                    snippets_enabled: value.snippets_enabled?,
                    squash_commit_template: value.squash_commit_template?,
                    squash_option: value.squash_option?,
                    suggestion_commit_message: value.suggestion_commit_message?,
                    tag_list: value.tag_list?,
                    topics: value.topics?,
                    visibility: value.visibility?,
                    warn_about_potentially_unwanted_characters: value
                        .warn_about_potentially_unwanted_characters?,
                    wiki_access_level: value.wiki_access_level?,
                    wiki_enabled: value.wiki_enabled?,
                })
            }
        }
        impl ::std::convert::From<super::PutApiV4ProjectsId> for PutApiV4ProjectsId {
            fn from(value: super::PutApiV4ProjectsId) -> Self {
                Self {
                    allow_merge_on_skipped_pipeline: Ok(value.allow_merge_on_skipped_pipeline),
                    allow_pipeline_trigger_approve_deployment: Ok(
                        value.allow_pipeline_trigger_approve_deployment
                    ),
                    analytics_access_level: Ok(value.analytics_access_level),
                    approvals_before_merge: Ok(value.approvals_before_merge),
                    auto_cancel_pending_pipelines: Ok(value.auto_cancel_pending_pipelines),
                    auto_devops_deploy_strategy: Ok(value.auto_devops_deploy_strategy),
                    auto_devops_enabled: Ok(value.auto_devops_enabled),
                    autoclose_referenced_issues: Ok(value.autoclose_referenced_issues),
                    avatar: Ok(value.avatar),
                    build_git_strategy: Ok(value.build_git_strategy),
                    build_timeout: Ok(value.build_timeout),
                    builds_access_level: Ok(value.builds_access_level),
                    ci_allow_fork_pipelines_to_run_in_parent_project: Ok(
                        value.ci_allow_fork_pipelines_to_run_in_parent_project
                    ),
                    ci_config_path: Ok(value.ci_config_path),
                    ci_default_git_depth: Ok(value.ci_default_git_depth),
                    ci_delete_pipelines_in_seconds: Ok(value.ci_delete_pipelines_in_seconds),
                    ci_forward_deployment_enabled: Ok(value.ci_forward_deployment_enabled),
                    ci_forward_deployment_rollback_allowed: Ok(
                        value.ci_forward_deployment_rollback_allowed
                    ),
                    ci_id_token_sub_claim_components: Ok(value.ci_id_token_sub_claim_components),
                    ci_pipeline_variables_minimum_override_role: Ok(
                        value.ci_pipeline_variables_minimum_override_role
                    ),
                    ci_push_repository_for_job_token_allowed: Ok(
                        value.ci_push_repository_for_job_token_allowed
                    ),
                    ci_restrict_pipeline_cancellation_role: Ok(
                        value.ci_restrict_pipeline_cancellation_role
                    ),
                    ci_separated_caches: Ok(value.ci_separated_caches),
                    container_expiration_policy_attributes: Ok(
                        value.container_expiration_policy_attributes
                    ),
                    container_registry_access_level: Ok(value.container_registry_access_level),
                    container_registry_enabled: Ok(value.container_registry_enabled),
                    default_branch: Ok(value.default_branch),
                    description: Ok(value.description),
                    emails_disabled: Ok(value.emails_disabled),
                    emails_enabled: Ok(value.emails_enabled),
                    enforce_auth_checks_on_uploads: Ok(value.enforce_auth_checks_on_uploads),
                    environments_access_level: Ok(value.environments_access_level),
                    external_authorization_classification_label: Ok(
                        value.external_authorization_classification_label
                    ),
                    fallback_approvals_required: Ok(value.fallback_approvals_required),
                    feature_flags_access_level: Ok(value.feature_flags_access_level),
                    forking_access_level: Ok(value.forking_access_level),
                    group_runners_enabled: Ok(value.group_runners_enabled),
                    import_url: Ok(value.import_url),
                    infrastructure_access_level: Ok(value.infrastructure_access_level),
                    issue_branch_template: Ok(value.issue_branch_template),
                    issues_access_level: Ok(value.issues_access_level),
                    issues_enabled: Ok(value.issues_enabled),
                    issues_template: Ok(value.issues_template),
                    jobs_enabled: Ok(value.jobs_enabled),
                    keep_latest_artifact: Ok(value.keep_latest_artifact),
                    lfs_enabled: Ok(value.lfs_enabled),
                    max_artifacts_size: Ok(value.max_artifacts_size),
                    merge_commit_template: Ok(value.merge_commit_template),
                    merge_method: Ok(value.merge_method),
                    merge_pipelines_enabled: Ok(value.merge_pipelines_enabled),
                    merge_requests_access_level: Ok(value.merge_requests_access_level),
                    merge_requests_enabled: Ok(value.merge_requests_enabled),
                    merge_requests_template: Ok(value.merge_requests_template),
                    merge_trains_enabled: Ok(value.merge_trains_enabled),
                    merge_trains_skip_train_allowed: Ok(value.merge_trains_skip_train_allowed),
                    mirror: Ok(value.mirror),
                    mirror_branch_regex: Ok(value.mirror_branch_regex),
                    mirror_overwrites_diverged_branches: Ok(
                        value.mirror_overwrites_diverged_branches
                    ),
                    mirror_trigger_builds: Ok(value.mirror_trigger_builds),
                    mirror_user_id: Ok(value.mirror_user_id),
                    model_experiments_access_level: Ok(value.model_experiments_access_level),
                    model_registry_access_level: Ok(value.model_registry_access_level),
                    monitor_access_level: Ok(value.monitor_access_level),
                    mr_default_target_self: Ok(value.mr_default_target_self),
                    name: Ok(value.name),
                    only_allow_merge_if_all_discussions_are_resolved: Ok(
                        value.only_allow_merge_if_all_discussions_are_resolved
                    ),
                    only_allow_merge_if_all_status_checks_passed: Ok(
                        value.only_allow_merge_if_all_status_checks_passed
                    ),
                    only_allow_merge_if_pipeline_succeeds: Ok(
                        value.only_allow_merge_if_pipeline_succeeds
                    ),
                    only_mirror_protected_branches: Ok(value.only_mirror_protected_branches),
                    packages_enabled: Ok(value.packages_enabled),
                    pages_access_level: Ok(value.pages_access_level),
                    path: Ok(value.path),
                    prevent_merge_without_jira_issue: Ok(value.prevent_merge_without_jira_issue),
                    printing_merge_request_link_enabled: Ok(
                        value.printing_merge_request_link_enabled
                    ),
                    public_builds: Ok(value.public_builds),
                    public_jobs: Ok(value.public_jobs),
                    releases_access_level: Ok(value.releases_access_level),
                    remove_source_branch_after_merge: Ok(value.remove_source_branch_after_merge),
                    repository_access_level: Ok(value.repository_access_level),
                    repository_storage: Ok(value.repository_storage),
                    request_access_enabled: Ok(value.request_access_enabled),
                    requirements_access_level: Ok(value.requirements_access_level),
                    resolve_outdated_diff_discussions: Ok(value.resolve_outdated_diff_discussions),
                    restrict_user_defined_variables: Ok(value.restrict_user_defined_variables),
                    security_and_compliance_access_level: Ok(
                        value.security_and_compliance_access_level
                    ),
                    service_desk_enabled: Ok(value.service_desk_enabled),
                    shared_runners_enabled: Ok(value.shared_runners_enabled),
                    show_default_award_emojis: Ok(value.show_default_award_emojis),
                    show_diff_preview_in_email: Ok(value.show_diff_preview_in_email),
                    snippets_access_level: Ok(value.snippets_access_level),
                    snippets_enabled: Ok(value.snippets_enabled),
                    squash_commit_template: Ok(value.squash_commit_template),
                    squash_option: Ok(value.squash_option),
                    suggestion_commit_message: Ok(value.suggestion_commit_message),
                    tag_list: Ok(value.tag_list),
                    topics: Ok(value.topics),
                    visibility: Ok(value.visibility),
                    warn_about_potentially_unwanted_characters: Ok(
                        value.warn_about_potentially_unwanted_characters
                    ),
                    wiki_access_level: Ok(value.wiki_access_level),
                    wiki_enabled: Ok(value.wiki_enabled),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
            cadence: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            keep_n: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            name_regex: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            name_regex_keep: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            older_than: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
            fn default() -> Self {
                Self {
                    cadence: Ok(Default::default()),
                    enabled: Ok(Default::default()),
                    keep_n: Ok(Default::default()),
                    name_regex: Ok(Default::default()),
                    name_regex_keep: Ok(Default::default()),
                    older_than: Ok(Default::default()),
                }
            }
        }
        impl PutApiV4ProjectsIdContainerExpirationPolicyAttributes {
            pub fn cadence<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cadence = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cadence: {}", e));
                self
            }
            pub fn enabled<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.enabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for enabled: {}", e));
                self
            }
            pub fn keep_n<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.keep_n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for keep_n: {}", e));
                self
            }
            pub fn name_regex<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_regex = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name_regex: {}", e));
                self
            }
            pub fn name_regex_keep<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.name_regex_keep = value.try_into().map_err(|e| {
                    format!("error converting supplied value for name_regex_keep: {}", e)
                });
                self
            }
            pub fn older_than<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.older_than = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for older_than: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<PutApiV4ProjectsIdContainerExpirationPolicyAttributes>
            for super::PutApiV4ProjectsIdContainerExpirationPolicyAttributes
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PutApiV4ProjectsIdContainerExpirationPolicyAttributes,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cadence: value.cadence?,
                    enabled: value.enabled?,
                    keep_n: value.keep_n?,
                    name_regex: value.name_regex?,
                    name_regex_keep: value.name_regex_keep?,
                    older_than: value.older_than?,
                })
            }
        }
        impl ::std::convert::From<super::PutApiV4ProjectsIdContainerExpirationPolicyAttributes>
            for PutApiV4ProjectsIdContainerExpirationPolicyAttributes
        {
            fn from(value: super::PutApiV4ProjectsIdContainerExpirationPolicyAttributes) -> Self {
                Self {
                    cadence: Ok(value.cadence),
                    enabled: Ok(value.enabled),
                    keep_n: Ok(value.keep_n),
                    name_regex: Ok(value.name_regex),
                    name_regex_keep: Ok(value.name_regex_keep),
                    older_than: Ok(value.older_than),
                }
            }
        }
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }
        pub(super) fn post_api_v4_projects_id_repository_commits_actions_item_encoding(
        ) -> super::PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding {
            super::PostApiV4ProjectsIdRepositoryCommitsActionsItemEncoding::Text
        }
    }
}
#[derive(Clone, Debug)]
/**Client for GitLab API

Version: v4*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "v4"
    }
}
impl Client {
    /**List currently authenticated user's events

    This feature was introduced in GitLab 9.3.

    Sends a `GET` request to `/api/v4/events`

    Arguments:
    - `action`: Event action to filter on
    - `after`: Include only events created after this date
    - `before`: Include only events created before this date
    - `page`: Current page number
    - `per_page`: Number of items per page
    - `scope`: Include all events across a user’s projects
    - `sort`: Return events sorted in ascending and descending order
    - `target_type`: Event target type to filter on
    ```ignore
    let response = client.get_api_v4_events()
        .action(action)
        .after(after)
        .before(before)
        .page(page)
        .per_page(per_page)
        .scope(scope)
        .sort(sort)
        .target_type(target_type)
        .send()
        .await;
    ```*/
    pub fn get_api_v4_events(&self) -> builder::GetApiV4Events {
        builder::GetApiV4Events::new(self)
    }
    /**Get a single project

    Sends a `GET` request to `/api/v4/projects/{id}`

    Arguments:
    - `id`: The ID or URL-encoded path of the project
    - `license`: Include project license data
    - `statistics`: Include project statistics
    - `with_custom_attributes`: Include custom attributes in the response
    ```ignore
    let response = client.get_api_v4_projects_id()
        .id(id)
        .license(license)
        .statistics(statistics)
        .with_custom_attributes(with_custom_attributes)
        .send()
        .await;
    ```*/
    pub fn get_api_v4_projects_id(&self) -> builder::GetApiV4ProjectsId {
        builder::GetApiV4ProjectsId::new(self)
    }
    /**Update an existing project

    Sends a `PUT` request to `/api/v4/projects/{id}`

    Arguments:
    - `id`: The ID or URL-encoded path of the project
    - `body`
    ```ignore
    let response = client.put_api_v4_projects_id()
        .id(id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn put_api_v4_projects_id(&self) -> builder::PutApiV4ProjectsId {
        builder::PutApiV4ProjectsId::new(self)
    }
    /**Delete a project

    Sends a `DELETE` request to `/api/v4/projects/{id}`

    Arguments:
    - `id`: The ID or URL-encoded path of the project
    ```ignore
    let response = client.delete_api_v4_projects_id()
        .id(id)
        .send()
        .await;
    ```*/
    pub fn delete_api_v4_projects_id(&self) -> builder::DeleteApiV4ProjectsId {
        builder::DeleteApiV4ProjectsId::new(self)
    }
    /**Get a project repository commits

    Sends a `GET` request to `/api/v4/projects/{id}/repository/commits`

    Arguments:
    - `id`: The ID or URL-encoded path of the project
    - `all`: Every commit will be returned
    - `author`: Search commits by commit author
    - `first_parent`: Only include the first parent of merges
    - `order`: List commits in order
    - `page`: Current page number
    - `path`: The file path
    - `per_page`: Number of items per page
    - `ref_name`: The name of a repository branch or tag, if not given the default branch is used
    - `since`: Only commits after or on this date will be returned
    - `trailers`: Parse and include Git trailers for every commit
    - `until`: Only commits before or on this date will be returned
    - `with_stats`: Stats about each commit will be added to the response
    ```ignore
    let response = client.get_api_v4_projects_id_repository_commits()
        .id(id)
        .all(all)
        .author(author)
        .first_parent(first_parent)
        .order(order)
        .page(page)
        .path(path)
        .per_page(per_page)
        .ref_name(ref_name)
        .since(since)
        .trailers(trailers)
        .until(until)
        .with_stats(with_stats)
        .send()
        .await;
    ```*/
    pub fn get_api_v4_projects_id_repository_commits(
        &self,
    ) -> builder::GetApiV4ProjectsIdRepositoryCommits {
        builder::GetApiV4ProjectsIdRepositoryCommits::new(self)
    }
    /**Commit multiple file changes as one commit

    This feature was introduced in GitLab 8.13

    Sends a `POST` request to `/api/v4/projects/{id}/repository/commits`

    Arguments:
    - `id`: The ID or URL-encoded path of the project
    - `body`
    ```ignore
    let response = client.post_api_v4_projects_id_repository_commits()
        .id(id)
        .body(body)
        .send()
        .await;
    ```*/
    pub fn post_api_v4_projects_id_repository_commits(
        &self,
    ) -> builder::PostApiV4ProjectsIdRepositoryCommits {
        builder::PostApiV4ProjectsIdRepositoryCommits::new(self)
    }
}
/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`Client::get_api_v4_events`]

    [`Client::get_api_v4_events`]: super::Client::get_api_v4_events*/
    #[derive(Debug, Clone)]
    pub struct GetApiV4Events<'a> {
        client: &'a super::Client,
        action: Result<Option<::std::string::String>, String>,
        after: Result<Option<chrono::naive::NaiveDate>, String>,
        before: Result<Option<chrono::naive::NaiveDate>, String>,
        page: Result<Option<i32>, String>,
        per_page: Result<Option<i32>, String>,
        scope: Result<Option<::std::string::String>, String>,
        sort: Result<Option<types::GetApiV4EventsSort>, String>,
        target_type: Result<Option<types::GetApiV4EventsTargetType>, String>,
    }
    impl<'a> GetApiV4Events<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                action: Ok(None),
                after: Ok(None),
                before: Ok(None),
                page: Ok(None),
                per_page: Ok(None),
                scope: Ok(None),
                sort: Ok(None),
                target_type: Ok(None),
            }
        }
        pub fn action<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.action = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for action failed".to_string()
            });
            self
        }
        pub fn after<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<chrono::naive::NaiveDate>,
        {
            self.after = value.try_into().map(Some).map_err(|_| {
                "conversion to `chrono :: naive :: NaiveDate` for after failed".to_string()
            });
            self
        }
        pub fn before<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<chrono::naive::NaiveDate>,
        {
            self.before = value.try_into().map(Some).map_err(|_| {
                "conversion to `chrono :: naive :: NaiveDate` for before failed".to_string()
            });
            self
        }
        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for page failed".to_string());
            self
        }
        pub fn per_page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.per_page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for per_page failed".to_string());
            self
        }
        pub fn scope<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.scope = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for scope failed".to_string()
            });
            self
        }
        pub fn sort<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetApiV4EventsSort>,
        {
            self.sort = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetApiV4EventsSort` for sort failed".to_string());
            self
        }
        pub fn target_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetApiV4EventsTargetType>,
        {
            self.target_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetApiV4EventsTargetType` for target_type failed".to_string()
            });
            self
        }
        ///Sends a `GET` request to `/api/v4/events`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::ApiEntitiesEvent>>, Error<()>> {
            let Self {
                client,
                action,
                after,
                before,
                page,
                per_page,
                scope,
                sort,
                target_type,
            } = self;
            let action = action.map_err(Error::InvalidRequest)?;
            let after = after.map_err(Error::InvalidRequest)?;
            let before = before.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let per_page = per_page.map_err(Error::InvalidRequest)?;
            let scope = scope.map_err(Error::InvalidRequest)?;
            let sort = sort.map_err(Error::InvalidRequest)?;
            let target_type = target_type.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v4/events", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("action", &action))
                .query(&progenitor_client::QueryParam::new("after", &after))
                .query(&progenitor_client::QueryParam::new("before", &before))
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("per_page", &per_page))
                .query(&progenitor_client::QueryParam::new("scope", &scope))
                .query(&progenitor_client::QueryParam::new("sort", &sort))
                .query(&progenitor_client::QueryParam::new(
                    "target_type",
                    &target_type,
                ))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_api_v4_projects_id`]

    [`Client::get_api_v4_projects_id`]: super::Client::get_api_v4_projects_id*/
    #[derive(Debug, Clone)]
    pub struct GetApiV4ProjectsId<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        license: Result<Option<bool>, String>,
        statistics: Result<Option<bool>, String>,
        with_custom_attributes: Result<Option<bool>, String>,
    }
    impl<'a> GetApiV4ProjectsId<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                license: Ok(None),
                statistics: Ok(None),
                with_custom_attributes: Ok(None),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }
        pub fn license<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.license = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for license failed".to_string());
            self
        }
        pub fn statistics<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.statistics = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for statistics failed".to_string());
            self
        }
        pub fn with_custom_attributes<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.with_custom_attributes = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for with_custom_attributes failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v4/projects/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiEntitiesProjectWithAccess>, Error<()>> {
            let Self {
                client,
                id,
                license,
                statistics,
                with_custom_attributes,
            } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let license = license.map_err(Error::InvalidRequest)?;
            let statistics = statistics.map_err(Error::InvalidRequest)?;
            let with_custom_attributes = with_custom_attributes.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v4/projects/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("license", &license))
                .query(&progenitor_client::QueryParam::new(
                    "statistics",
                    &statistics,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "with_custom_attributes",
                    &with_custom_attributes,
                ))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::put_api_v4_projects_id`]

    [`Client::put_api_v4_projects_id`]: super::Client::put_api_v4_projects_id*/
    #[derive(Debug, Clone)]
    pub struct PutApiV4ProjectsId<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        body: Result<types::builder::PutApiV4ProjectsId, String>,
    }
    impl<'a> PutApiV4ProjectsId<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PutApiV4ProjectsId>,
            <V as std::convert::TryInto<types::PutApiV4ProjectsId>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `PutApiV4ProjectsId` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PutApiV4ProjectsId,
            ) -> types::builder::PutApiV4ProjectsId,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `PUT` request to `/api/v4/projects/{id}`
        pub async fn send(self) -> Result<ResponseValue<types::ApiEntitiesProject>, Error<()>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::PutApiV4ProjectsId::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v4/projects/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::delete_api_v4_projects_id`]

    [`Client::delete_api_v4_projects_id`]: super::Client::delete_api_v4_projects_id*/
    #[derive(Debug, Clone)]
    pub struct DeleteApiV4ProjectsId<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
    }
    impl<'a> DeleteApiV4ProjectsId<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }
        ///Sends a `DELETE` request to `/api/v4/projects/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v4/projects/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                202u16 => Ok(ResponseValue::empty(response)),
                403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_api_v4_projects_id_repository_commits`]

    [`Client::get_api_v4_projects_id_repository_commits`]: super::Client::get_api_v4_projects_id_repository_commits*/
    #[derive(Debug, Clone)]
    pub struct GetApiV4ProjectsIdRepositoryCommits<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        all: Result<Option<bool>, String>,
        author: Result<Option<::std::string::String>, String>,
        first_parent: Result<Option<bool>, String>,
        order: Result<Option<types::GetApiV4ProjectsIdRepositoryCommitsOrder>, String>,
        page: Result<Option<i32>, String>,
        path: Result<Option<::std::string::String>, String>,
        per_page: Result<Option<i32>, String>,
        ref_name: Result<Option<::std::string::String>, String>,
        since: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        trailers: Result<Option<bool>, String>,
        until: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        with_stats: Result<Option<bool>, String>,
    }
    impl<'a> GetApiV4ProjectsIdRepositoryCommits<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                all: Ok(None),
                author: Ok(None),
                first_parent: Ok(None),
                order: Ok(None),
                page: Ok(None),
                path: Ok(None),
                per_page: Ok(None),
                ref_name: Ok(None),
                since: Ok(None),
                trailers: Ok(None),
                until: Ok(None),
                with_stats: Ok(None),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }
        pub fn all<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.all = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for all failed".to_string());
            self
        }
        pub fn author<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.author = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for author failed".to_string()
            });
            self
        }
        pub fn first_parent<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.first_parent = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for first_parent failed".to_string());
            self
        }
        pub fn order<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetApiV4ProjectsIdRepositoryCommitsOrder>,
        {
            self.order = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetApiV4ProjectsIdRepositoryCommitsOrder` for order failed"
                    .to_string()
            });
            self
        }
        pub fn page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for page failed".to_string());
            self
        }
        pub fn path<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.path = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for path failed".to_string()
            });
            self
        }
        pub fn per_page<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.per_page = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for per_page failed".to_string());
            self
        }
        pub fn ref_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.ref_name = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for ref_name failed".to_string()
            });
            self
        }
        pub fn since<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
        {
            self.since = value.try_into().map(Some).map_err(|_| {
                "conversion to `chrono :: DateTime < chrono :: offset :: Utc >` for since failed"
                    .to_string()
            });
            self
        }
        pub fn trailers<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.trailers = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for trailers failed".to_string());
            self
        }
        pub fn until<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
        {
            self.until = value.try_into().map(Some).map_err(|_| {
                "conversion to `chrono :: DateTime < chrono :: offset :: Utc >` for until failed"
                    .to_string()
            });
            self
        }
        pub fn with_stats<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.with_stats = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for with_stats failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v4/projects/{id}/repository/commits`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::ApiEntitiesCommit>>, Error<()>> {
            let Self {
                client,
                id,
                all,
                author,
                first_parent,
                order,
                page,
                path,
                per_page,
                ref_name,
                since,
                trailers,
                until,
                with_stats,
            } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let all = all.map_err(Error::InvalidRequest)?;
            let author = author.map_err(Error::InvalidRequest)?;
            let first_parent = first_parent.map_err(Error::InvalidRequest)?;
            let order = order.map_err(Error::InvalidRequest)?;
            let page = page.map_err(Error::InvalidRequest)?;
            let path = path.map_err(Error::InvalidRequest)?;
            let per_page = per_page.map_err(Error::InvalidRequest)?;
            let ref_name = ref_name.map_err(Error::InvalidRequest)?;
            let since = since.map_err(Error::InvalidRequest)?;
            let trailers = trailers.map_err(Error::InvalidRequest)?;
            let until = until.map_err(Error::InvalidRequest)?;
            let with_stats = with_stats.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v4/projects/{}/repository/commits",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("all", &all))
                .query(&progenitor_client::QueryParam::new("author", &author))
                .query(&progenitor_client::QueryParam::new(
                    "first_parent",
                    &first_parent,
                ))
                .query(&progenitor_client::QueryParam::new("order", &order))
                .query(&progenitor_client::QueryParam::new("page", &page))
                .query(&progenitor_client::QueryParam::new("path", &path))
                .query(&progenitor_client::QueryParam::new("per_page", &per_page))
                .query(&progenitor_client::QueryParam::new("ref_name", &ref_name))
                .query(&progenitor_client::QueryParam::new("since", &since))
                .query(&progenitor_client::QueryParam::new("trailers", &trailers))
                .query(&progenitor_client::QueryParam::new("until", &until))
                .query(&progenitor_client::QueryParam::new(
                    "with_stats",
                    &with_stats,
                ))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::post_api_v4_projects_id_repository_commits`]

    [`Client::post_api_v4_projects_id_repository_commits`]: super::Client::post_api_v4_projects_id_repository_commits*/
    #[derive(Debug, Clone)]
    pub struct PostApiV4ProjectsIdRepositoryCommits<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        body: Result<types::builder::PostApiV4ProjectsIdRepositoryCommits, String>,
    }
    impl<'a> PostApiV4ProjectsIdRepositoryCommits<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostApiV4ProjectsIdRepositoryCommits>,
            <V as std::convert::TryInto<types::PostApiV4ProjectsIdRepositoryCommits>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `PostApiV4ProjectsIdRepositoryCommits` for body failed: {}",
                    s
                )
            });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostApiV4ProjectsIdRepositoryCommits,
            )
                -> types::builder::PostApiV4ProjectsIdRepositoryCommits,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v4/projects/{id}/repository/commits`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiEntitiesCommitDetail>, Error<()>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::PostApiV4ProjectsIdRepositoryCommits::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v4/projects/{}/repository/commits",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    
}
