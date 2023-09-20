use itertools::Itertools;
use octocrab::models::Author;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use super::ALFRED_WORKFLOW_DATA;

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn owners(&self) -> impl Iterator<Item = String> + '_ {
        self.items.iter().flat_map(|item| item.owner()).unique()
    }
}

impl FromIterator<Item> for Items {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Item {
    title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    subtitle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    uid: Option<String>,

    arg: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<String>,

    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    matches: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    icon: Option<Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    variables: Option<Value>,

    #[builder(default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mods: Option<Modifiers>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Icon {
    path: PathBuf,
}

// impl FromStr for Icon {
//     type Err = eyre::Error;

//     fn from_str(name: &str) -> Result<Self, Self::Err> {
//         Ok(Self {
//             path: ALFRED_WORKFLOW_DATA.join(format!("{}.png", name)),
//         })
//     }
// }

pub struct AuthorIcon(pub String);

impl From<&Author> for AuthorIcon {
    fn from(author: &Author) -> Self {
        Self(author.login.clone())
    }
}

impl From<Option<Author>> for AuthorIcon {
    fn from(author: Option<Author>) -> Self {
        Self(author.map(|a| a.login).unwrap_or_default())
    }
}

impl From<Option<Box<Author>>> for AuthorIcon {
    fn from(author: Option<Box<Author>>) -> Self {
        match author {
            Some(author) => Self(author.login.clone()),
            None => Self("".into()),
        }
    }
}

impl From<&Option<Box<Author>>> for AuthorIcon {
    fn from(author: &Option<Box<Author>>) -> Self {
        match author {
            Some(author) => Self(author.login.clone()),
            None => Self("".into()),
        }
    }
}

impl From<AuthorIcon> for Icon {
    fn from(author: AuthorIcon) -> Self {
        match author.0.as_str() {
            "" => Self {
                path: ALFRED_WORKFLOW_DATA.join("octocat.png"),
            },
            user => Self {
                path: ALFRED_WORKFLOW_DATA.join(format!("{}.png", user)),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Modifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<Modifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cmd: Option<Modifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ctrl: Option<Modifier>,

    #[serde(rename = "fn", skip_serializing_if = "Option::is_none")]
    fun: Option<Modifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shift: Option<Modifier>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<Value>,
}

impl Item {
    pub fn owner(&self) -> Option<String> {
        // self.variables.get("owner").cloned()
        Some(self.variables.as_ref()?.as_object()?.get("owner")?.to_string())
    }
}
