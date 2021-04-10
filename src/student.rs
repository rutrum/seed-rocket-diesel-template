#[cfg(feature = "database")]
use crate::schema::student;

use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "database", derive(Insertable), table_name = "student")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub gpa: Option<f32>,
}

#[cfg_attr(feature = "database", derive(Queryable, Identifiable, AsChangeset), table_name = "student")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub gpa: Option<f32>,
}
