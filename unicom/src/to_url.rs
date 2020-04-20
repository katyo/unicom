use crate::Result;

pub use url::Url;

/// Something which can represented as url
pub trait ToUrl {
    fn to_url(self) -> Result<Url>;
}

impl ToUrl for Url {
    fn to_url(self) -> Result<Url> {
        Ok(self)
    }
}

/*impl<T> ToUrl for T
where
    T: AsRef<str>,
{
    fn to_url(self) -> Result<Url> {
        Ok(self.as_ref().parse()?)
    }
}*/

impl<'a> ToUrl for &'a Url {
    fn to_url(self) -> Result<Url> {
        Ok(self.clone())
    }
}

impl<'a> ToUrl for &'a str {
    fn to_url(self) -> Result<Url> {
        Ok(self.parse()?)
    }
}

impl<'a> ToUrl for &'a String {
    fn to_url(self) -> Result<Url> {
        (self.as_ref() as &str).to_url()
    }
}
