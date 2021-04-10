#[cfg(feature = "database")]
use crate::schema::student;

use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "database", derive(Insertable), table_name = "student")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub gpa: Option<f32>,
}

impl NewStudent {
    pub fn from_strings(name: String, s_gpa: String) -> Option<NewStudent> { 
        if name.is_empty() {
            return None;
        }

        if s_gpa.is_empty() {
            return Some(NewStudent { name, gpa: None })
        }

        match s_gpa.parse::<f32>() {
            Ok(gpa) => Some(NewStudent { name, gpa: Some(gpa) }),
            Err(_) => None,
        }
    }

    pub fn with_id(self, id: i32) -> Student {
        Student {
            id,
            name: self.name,
            gpa: self.gpa,
        }
    }
}

#[cfg_attr(feature = "database", derive(Queryable, Identifiable, AsChangeset), table_name = "student")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub gpa: Option<f32>,
}
