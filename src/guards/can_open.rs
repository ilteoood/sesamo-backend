use actix_web::guard::GuardContext;

pub fn can_open(_guard: &GuardContext) -> bool {
    true
}
