use poem_openapi::Object;
use crate::entity;

/// Chef scheme.
#[derive(Object)]
#[oai(example = "chef_example")]
pub struct Chef {
    pub id: i32,
    pub name: String,
    pub short_info: String,
    pub description: Option<String>,
    pub season: i32,
    pub source: Source,
    pub profile_key: Option<String>,
    pub created_at: String,
}


#[derive(poem_openapi::Enum)]
pub enum Source {
    White,
    Black,
}

fn chef_example() -> Chef {
    Chef {
        id: 1,
        name: "최현석".to_string(),
        short_info: "대한민국 대표 스타 셰프".to_string(),
        description: Some("".to_string()),
        season: 1,
        source: Source::Black,
        profile_key: Some("gordon_ramsay_profile".to_string()),
        created_at: "2024-01-01T12:00:00Z".to_string(),
    }
}

impl From<entity::chefs::Model> for Chef {
    fn from(model: entity::chefs::Model) -> Self {
        let source = match model.source.as_str() {
            "W" => Source::White,
            _ => Source::Black,
        };
        Chef {
            id: model.id,
            name: model.name,
            short_info: model.short_info,
            description: model.description,
            season: model.season,
            source,
            profile_key: model.profile_key,
            created_at: model.created_at.to_string(),
        }
    }

}