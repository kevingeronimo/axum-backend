use cookie::Cookie;
use http::{header::COOKIE, HeaderValue, Request};

pub fn parse_cookie<'a, 'b, B>(request: &'a Request<B>, id: &'b str) -> Option<Cookie<'a>> {
    let cookie = request
        .headers()
        .get(COOKIE)
        .map(HeaderValue::to_str)
        .map(|result| {
            result.map(|cookies| {
                cookies
                    .split(';')
                    .filter(|cookie| cookie.contains(id))
                    .map(Cookie::parse)
                    .next()
            })
        });

    if let Some(Ok(Some(Ok(cookie)))) = cookie {
        Some(cookie)
    } else {
        None
    }
}
