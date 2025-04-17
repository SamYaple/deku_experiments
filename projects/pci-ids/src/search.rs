use crate::{PciEntity, PciIds};

// All code below was produced by an LLM. TODO review it a little more beyond just hammering it with tests.

/// Atomic pieces you can filter on
#[derive(Debug, PartialEq)]
pub enum QueryTerm<'a> {
    /// substring‐match against any name field
    Name(&'a str),

    /// exact‐match against any u8 ID (class, subclass, prog_if)
    Id8(u8),

    /// exact‐match against any u16 ID (vendor, device, subvendor, subdevice)
    Id16(u16),

    /// match subsystem pairs (subvendor:subdevice), e.g. "dead:beef"
    Subsystem { vendor: u16, device: u16 },
}

/// How to combine multiple terms
#[derive(Debug, PartialEq)]
pub enum MatchMode {
    /// every term must match (AND)
    All,
    /// any term may match (OR)
    Any,
}

/// A full search request
#[derive(Debug, PartialEq)]
pub struct SearchQuery<'a> {
    pub terms: Vec<QueryTerm<'a>>,
    pub mode: MatchMode,
}

impl<'a> SearchQuery<'a> {
    /// simple builder: split on whitespace, parse hex‐colon pairs as Subsystem,
    /// 0x... or decimal into Id8/Id16, otherwise Name
    pub fn parse(s: &'a str) -> Self {
        let mut terms = Vec::new();
        for token in s.split_whitespace() {
            if let Some((a, b)) = token.split_once(':') {
                // hex‐colon pair → Subsystem
                if let (Ok(v), Ok(d)) = (
                    u16::from_str_radix(a.trim_start_matches("0x"), 16),
                    u16::from_str_radix(b.trim_start_matches("0x"), 16),
                ) {
                    terms.push(QueryTerm::Subsystem {
                        vendor: v,
                        device: d,
                    });
                    continue;
                }
            }
            // try u8
            if let Ok(x) = token.parse::<u8>() {
                terms.push(QueryTerm::Id8(x));
                continue;
            }
            // try hex or decimal u16
            if let Ok(x) = token
                .parse::<u16>()
                .or_else(|_| u16::from_str_radix(token.trim_start_matches("0x"), 16))
            {
                terms.push(QueryTerm::Id16(x));
                continue;
            }
            // fallback: name
            terms.push(QueryTerm::Name(token));
        }
        // default to AND semantics; you could make this configurable too
        SearchQuery {
            terms,
            mode: MatchMode::All,
        }
    }
}

impl<'a> PciIds<'a> {
    /// a unified search over everything, using a `SearchQuery`
    pub fn search(&'a self, s: &str) -> Vec<PciEntity<'a>> {
        let q = SearchQuery::parse(s);
        let mut found = Vec::new();

        // helper to test one entity against all terms
        let matches = |name: &str, ids_u8: &[u8], ids_u16: &[u16]| {
            q.terms
                .iter()
                .map(|term| {
                    let hit = match term {
                        QueryTerm::Name(sub) => name.to_lowercase().contains(&sub.to_lowercase()),

                        QueryTerm::Id8(x) => ids_u8.iter().any(|id| id == x),

                        QueryTerm::Id16(x) => ids_u16.iter().any(|id| id == x),

                        QueryTerm::Subsystem { vendor, device } => {
                            // only relevant for Subsystem entities,
                            // so we’ll handle that case at call‐site
                            false
                        }
                    };
                    if let MatchMode::Any = q.mode {
                        // short‐circuit on first true
                        if hit {
                            return true;
                        }
                    }
                    hit
                })
                .collect::<Vec<_>>()
        };

        // walk the whole PCI tree
        for class in &self.classes {
            let class_hits = matches(class.name, &[class.id], &[]);
            if q.mode == MatchMode::Any && class_hits.iter().any(|&h| h)
                || q.mode == MatchMode::All && class_hits.iter().all(|&h| h)
            {
                found.push(PciEntity::Class(class));
            }
            for sub in &class.subclasses {
                let sub_hits = matches(sub.name, &[sub.id], &[]);
                if (q.mode == MatchMode::Any && sub_hits.iter().any(|&h| h))
                    || (q.mode == MatchMode::All && sub_hits.iter().all(|&h| h))
                {
                    found.push(PciEntity::SubClass(sub));
                }
                for pi in &sub.prog_ifs {
                    let pi_hits = matches(pi.name, &[pi.id], &[]);
                    if (q.mode == MatchMode::Any && pi_hits.iter().any(|&h| h))
                        || (q.mode == MatchMode::All && pi_hits.iter().all(|&h| h))
                    {
                        found.push(PciEntity::ProgIf(pi));
                    }
                }
            }
        }

        for vendor in &self.vendors {
            let vend_hits = matches(vendor.name, &[], &[vendor.id]);
            if (q.mode == MatchMode::Any && vend_hits.iter().any(|&h| h))
                || (q.mode == MatchMode::All && vend_hits.iter().all(|&h| h))
            {
                found.push(PciEntity::Vendor(vendor));
            }
            for dev in &vendor.devices {
                let dev_hits = matches(dev.name, &[], &[dev.id]);
                if (q.mode == MatchMode::Any && dev_hits.iter().any(|&h| h))
                    || (q.mode == MatchMode::All && dev_hits.iter().all(|&h| h))
                {
                    found.push(PciEntity::Device(dev));
                }
                for sub in &dev.subsystems {
                    // handle Subsystem matching specially
                    let mut hits = Vec::new();
                    for term in &q.terms {
                        let hit = match term {
                            QueryTerm::Name(subs) => {
                                sub.name.to_lowercase().contains(&subs.to_lowercase())
                            }

                            QueryTerm::Id16(x) => x == &sub.subvendor_id || x == &sub.subdevice_id,

                            QueryTerm::Subsystem { vendor, device } => {
                                vendor == &sub.subvendor_id && device == &sub.subdevice_id
                            }

                            _ => false,
                        };
                        if let MatchMode::Any = q.mode {
                            if hit {
                                hits.push(true);
                                break;
                            }
                        } else {
                            hits.push(hit);
                        }
                    }

                    let passes = match q.mode {
                        MatchMode::Any => hits.iter().any(|&h| h),
                        MatchMode::All => q.terms.len() == hits.len() && hits.iter().all(|&h| h),
                    };

                    if passes {
                        found.push(PciEntity::Subsystem(sub));
                    }
                }
            }
        }

        found
    }
}
