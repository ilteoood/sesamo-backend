use actix_web::guard::GuardContext;

pub fn can_open(guard: &GuardContext) -> bool {
    true
}
