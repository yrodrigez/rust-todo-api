use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn is_auth () -> impl Filter<Extract = (UserContext,) , Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(HEADER_XAUTH))
        .and_then(|xauth: String| async move {
            if !xauth.ends_with(".exp.signature") {
                return Err(warp::reject::custom(FailAuth));
            }
            
            if let Some(user_id) = xauth
                .splitn(2, ".")
                .next()
                .and_then(|v| v.parse::<i64>().ok()) {
                Ok::<UserContext, warp::Rejection>(UserContext {user_id})
            }
            else { 
                Err(warp::reject::custom(FailAuth))
            }
        })
}

pub struct UserContext {
    pub user_id: i64,
}


#[derive(Debug)]
pub struct FailAuth;

impl warp::reject::Reject for FailAuth{}