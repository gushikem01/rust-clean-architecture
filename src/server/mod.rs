mod handlers;

use crate::domains::profile::ProfileRepository;

pub fn run() {
  let p = handlers::get_profile(RequestContext::new(), 1);
  println!("{}", p.to_string());
}

pub struct RequestContext {}

impl RequestContext {
  pub fn new() -> RequestContext {
    RequestContext{}
  }
  pub fn profile_repository(&self) -> impl ProfileRepository {
    use crate::infrastructures::repository::profile::ProfileRepositoryImpl;
    ProfileRepositoryImpl {}
  }
}
