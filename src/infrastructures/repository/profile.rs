use crate::domains::profile::{Profile, ProfileRepository};
use failure::Error;

#[derive(Debug, Clone)]
pub struct ProfileEntity {
  pub id: u128,
  pub name: String,
  pub age: u8,
}

impl ProfileEntity {
  fn from(model: &Profile) -> ProfileEntity {
    ProfileEntity {
      id: model.id.to_owned(),
      name: model.name.to_owned(),
      age: model.age.to_owned(),
    }
  }
}

pub struct ProfileRepositoryImpl {}

impl ProfileRepository for ProfileRepositoryImpl {
  fn find_by_id(&self, _id: u128) -> String {
    if _id == 1 {
      let p = ProfileEntity {
        id: 1,
        name: "Bob".to_string(),
        age: 16,
      };
      return p.name.to_string();
    } else {
      return "".to_string();
    }
  }

  fn update(&self, _profile: &Profile) -> Result<(), Error> {
    let p = ProfileEntity::from(_profile);
    println!("update: id={}", p.id);
    Ok(())
  }
}
