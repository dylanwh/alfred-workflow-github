use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<String>,

    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    matches: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,

    #[builder(setter(!strip_option))]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    variables: HashMap<String, String>,

    #[builder(default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mods: Option<Modifiers>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Icon {
    path: PathBuf,
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

    #[builder(setter(!strip_option))]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    variables: HashMap<String, String>,
}

impl Item {
    pub fn owner(&self) -> Option<String> {
        self.variables.get("owner").cloned()
    }
}
