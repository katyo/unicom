use std::{
    slice::Iter,
    iter::{Filter, Chain, FromIterator},
};

pub use std::net::IpAddr;

/// Address selection rules
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum AddrSelect {
    /// Select any available IP address (default)
    AnyType,
    /// Select any address but prefer IPv4 if available
    PreferV4,
    /// Select any address but prefer IPv6 if available
    PreferV6,
    /// Select IPv4 address only
    OnlyV4,
    /// Select IPv6 address only
    OnlyV6,
}

impl Default for AddrSelect {
    fn default() -> Self {
        Self::AnyType
    }
}

impl AddrSelect {
    /// Is IPv6 address allowed
    pub fn has_v6(&self) -> bool {
        self != &Self::OnlyV4
    }

    /// Is IPv4 address allowed
    pub fn has_v4(&self) -> bool {
        self != &Self::OnlyV6
    }
}

pub struct IpAddrIter<'a>(IterVariant<'a>);

type AddrFn = fn(&&IpAddr) -> bool;
type IterAddr<'a> = Iter<'a, IpAddr>;
type FilterAddr<'a> = Filter<IterAddr<'a>, AddrFn>;

enum IterVariant<'a> {
    Any(IterAddr<'a>),
    Only(FilterAddr<'a>),
    Prefer(Chain<FilterAddr<'a>, FilterAddr<'a>>),
}

impl<'a> Iterator for IpAddrIter<'a> {
    type Item = &'a IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        use IterVariant::*;
        match &mut self.0 {
            Any(iter) => iter.next(),
            Only(iter) => iter.next(),
            Prefer(iter) => iter.next(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpAddrs(Vec<IpAddr>);

impl From<Vec<IpAddr>> for IpAddrs {
    fn from(addrs: Vec<IpAddr>) -> Self {
        Self(addrs)
    }
}

impl FromIterator<IpAddr> for IpAddrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = IpAddr>,
    {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl IpAddrs {
    pub fn iter(&self, rule: AddrSelect) -> IpAddrIter<'_> {
        use AddrSelect::*;
        use IterVariant::*;
        IpAddrIter(match rule {
            AnyType => Any(self.0.iter()),
            OnlyV4 => Only(self.0.iter().filter(is_v4 as AddrFn)),
            OnlyV6 => Only(self.0.iter().filter(is_v6 as AddrFn)),
            PreferV4 => Prefer(self.0.iter().filter(is_v4 as AddrFn)
                               .chain(self.0.iter().filter(is_v6 as AddrFn))),
            PreferV6 => Prefer(self.0.iter().filter(is_v6 as AddrFn)
                               .chain(self.0.iter().filter(is_v4 as AddrFn))),
        })
    }
}

fn is_v4(addr: &&IpAddr) -> bool {
    addr.is_ipv4()
}

fn is_v6(addr: &&IpAddr) -> bool {
    addr.is_ipv6()
}
