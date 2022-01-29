use crate::usecases;
use super::RequestContext;

pub fn get_profile(
  data: RequestContext,
  _id: u128,
) -> String {
  let p = usecases::profile::get_profile(data.profile_repository(), _id);
  return p.to_string();
}
