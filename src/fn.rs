use rand::rng;
use rand::distr::Alphanumeric;
use rand::RngExt;

pub fn pipes<T, R>(val: T, f: impl Fn(T) -> R) -> R {
    f(val)
}

pub fn try_do<T, E>(f: impl Fn() -> Result<T, E>) -> Option<T> {
    f().ok()
}

pub fn when<T>(cond: bool, val: T) -> Option<T> {
    if cond { Some(val) } else { None }
}

pub fn unless<T>(cond: bool, val: T) -> Option<T> {
    if !cond { Some(val) } else { None }
}

pub fn default<T: Default>() -> T {
    T::default()
}

pub fn lazy<T>(f: impl Fn() -> T) -> T {
    f()
}

pub fn clone<T: Clone>(v: &T) -> T {
    v.clone()
}

pub fn to_string<T: ToString>(v: T) -> String {
    v.to_string()
}

pub fn parse<T: std::str::FromStr>(s: &str) -> Option<T> {
    s.parse().ok()
}

pub fn env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

pub fn rand(len: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn foreach<I, T, F>(iter: I, mut f: F)
where
    I: IntoIterator<Item = T>,
    F: FnMut(T),
{
    for item in iter {
        f(item);
    }
}

pub async fn all<T>(
    futures: Vec<impl std::future::Future<Output = T>>
) -> Vec<T> {
    futures::future::join_all(futures).await
}

pub fn ok<T, E>(res: Result<T, E>) -> Option<T> {
    res.ok()
}

pub fn err<T, E>(res: Result<T, E>) -> Option<E> {
    res.err()
}

pub fn or<T, E>(res: Result<T, E>, default: T) -> T {
    res.unwrap_or(default)
}

pub fn val<T>(opt: Option<T>) -> T {
    opt.unwrap()
}

pub fn val_or<T>(opt: Option<T>, default: T) -> T {
    opt.unwrap_or(default)
}

pub fn then<T, R>(val: T, f: impl Fn(T) -> R) -> R {
    f(val)
}

pub fn and_then_res<T, R, E>(
    res: Result<T, E>,
    f: impl Fn(T) -> Result<R, E>
) -> Result<R, E> {
    res.and_then(f)
}

pub fn and_then<T, R>(opt: Option<T>, f: impl Fn(T) -> Option<R>) -> Option<R> {
    opt.and_then(f)
}

pub fn tap<T>(val: T, f: impl Fn(&T)) -> T {
    f(&val);
    val
}

pub fn explode<'a>(delim: &'a str, s: &'a str) -> Vec<&'a str> {
    s.split(delim).collect()
}

pub fn strlen(s: &str) -> usize {
    s.len()
}

pub fn implode(delim: &str, v: Vec<&str>) -> String {
    v.join(delim)
}

pub fn array_push<T>(v: &mut Vec<T>, val: T) {
    v.push(val);
}

pub fn array_map<T, R, F>(v: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R,
{
    v.into_iter().map(f).collect()
}