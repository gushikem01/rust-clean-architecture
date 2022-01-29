use crate::domains::profile::{ProfileRepository};

pub fn get_profile(
  repository: impl ProfileRepository,
  _id: u128,
) -> String {
  return repository.find_by_id(1);
}
