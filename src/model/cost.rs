use std::fmt;

use super::symbols::AsManaSymbol;
pub use super::symbols::ManaSymbol;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct ManaCost {
    pub cost: Vec<ManaSymbol>,
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest = String::with_capacity(5 * self.cost.len());
        for m in &self.cost {
            digest += &m.to_string();
        }
        write!(f, "{}", digest)
    }
}

impl ManaCost {
    #[inline]
    fn to_annoted_string(&self) -> String {
        format!("{{{}}}", self.to_string())
    }

    fn mana_value(&self) -> u8 {
        self.cost.iter().map(|s| s.mana_value()).sum()
    }
}
