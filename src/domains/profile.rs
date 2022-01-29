use failure::Error;

#[derive(Debug, Clone)]
pub struct Profile {
  pub id: u128,
  pub name: String,
  pub age: u8,
}

// ProfileRepository リポジトリ一覧
pub trait ProfileRepository {
  fn find_by_id(&self, _id: u128) -> String;
  fn update(&self, _profile: &Profile) -> Result<(), Error>;
}
