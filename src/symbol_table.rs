use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

use itertools::Itertools;

use crate::parsers::text_symt::parsed_text_symt::ParsedTextSymt;
use crate::{Label, Result, Symbol, EPS_SYMBOL};

/// A symbol table stores a bidirectional mapping between arc labels and "symbols" (strings).
#[derive(PartialEq, Debug, Clone, Default)]
pub struct SymbolTable {
    label_to_symbol: HashMap<Label, Symbol>,
    symbol_to_label: HashMap<Symbol, Label>,
    num_symbols: usize,
}

macro_rules! write_symt_text {
    ($symt:expr, $f:expr) => {
        for (label, symbol) in $symt.iter().sorted_by_key(|k| k.0) {
            writeln!($f, "{}\t{}", symbol, label)?;
        }
    };
}

impl SymbolTable {
    /// Creates a `SymbolTable` with a single element in it: the pair (`EPS_LABEL`, `EPS_SYMBOL`).
    ///
    /// # Examples
    /// ```rust
    /// # use rustfst::SymbolTable;
    /// let mut symt = SymbolTable::new();
    /// ```
    pub fn new() -> Self {
        let mut symt = SymbolTable {
            label_to_symbol: HashMap::new(),
            symbol_to_label: HashMap::new(),
            num_symbols: 0,
        };

        symt.add_symbol(EPS_SYMBOL.to_string());

        symt
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds a symbol to the symbol table. The corresponding label is returned.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let mut symt = symt!["a", "b"];
    ///
    /// // Elements in the table : `<eps>`, `a`, `b`
    /// assert_eq!(symt.len(), 3);
    ///
    /// // Add a single symbol
    /// symt.add_symbol("c");
    ///
    /// // Elements in the table : `<eps>`, `a`, `b`, `c`
    /// assert_eq!(symt.len(), 4);
    /// # }
    /// ```
    pub fn add_symbol<S: Into<String>>(&mut self, sym: S) -> Label {
        let label = self.num_symbols;
        let sym = sym.into();

        self.symbol_to_label.entry(sym.clone()).or_insert(label);
        self.label_to_symbol.entry(label).or_insert(sym);

        self.num_symbols += 1;
        label
    }

    /// Returns the number of symbols stored in the symbol table.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let mut symt = symt!["a", "b"];
    /// assert_eq!(symt.len(), 3);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.num_symbols
    }

    /// Given a symbol, returns the label corresponding.
    /// If the symbol is not stored in the table then `None` is returned.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let mut symt = symt!["a", "b"];
    /// let label = symt.add_symbol("c");
    /// assert_eq!(symt.get_label("c"), Some(label));
    /// assert_eq!(symt.get_label("d"), None);
    /// # }
    /// ```
    pub fn get_label<S: Into<String>>(&self, sym: S) -> Option<Label> {
        self.symbol_to_label.get(&sym.into()).cloned()
    }

    /// Given a label, returns the symbol corresponding.
    /// If no there is no symbol with this label in the table then `None` is returned.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let mut symt = symt!["a", "b"];
    /// let label = symt.add_symbol("c");
    /// assert_eq!(symt.get_symbol(label), Some("c"));
    /// assert_eq!(symt.get_symbol(label + 1), None);
    /// # }
    /// ```
    pub fn get_symbol(&self, label: Label) -> Option<&str> {
        self.label_to_symbol.get(&label).map(|v| v.as_str())
    }

    /// Given a symbol, returns whether it is present in the table.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let symt = symt!["a", "b"];
    /// assert!(symt.contains_symbol("a"));
    /// # }
    /// ```
    pub fn contains_symbol<S: Into<String>>(&self, sym: S) -> bool {
        self.get_label(sym.into()).is_some()
    }

    /// Given a label, returns whether it is present in the table.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let mut symt = symt!["a", "b"];
    /// let label = symt.add_symbol("c");
    /// assert!(symt.contains_label(label));
    /// assert!(!symt.contains_label(label+1));
    /// # }
    pub fn contains_label(&self, label: Label) -> bool {
        self.get_symbol(label).is_some()
    }

    /// Reserves capacity for at least additional more elements to be inserted in the `SymbolTable`.
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.label_to_symbol.reserve(additional);
        self.symbol_to_label.reserve(additional);
    }

    /// An iterator on all the labels stored in the `SymbolTable`.
    /// The iterator element is `&'a Label`.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let symt = symt!["a", "b"];
    /// let mut iterator = symt.labels();
    ///
    /// # }
    /// ```
    pub fn labels(&self) -> Keys<Label, Symbol> {
        self.label_to_symbol.keys()
    }

    /// An iterator on all the symbols stored in the `SymbolTable`.
    /// The iterator element is `&'a Symbol`.
    ///
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate rustfst; fn main() {
    /// # use rustfst::SymbolTable;
    /// let symt = symt!["a", "b"];
    /// let mut iterator = symt.symbols();
    ///
    /// for symbol in symt.symbols() {
    ///     println!("Symbol : {:?}", symbol);
    /// }
    /// # }
    /// ```
    pub fn symbols(&self) -> Keys<Symbol, Label> {
        self.symbol_to_label.keys()
    }

    /// An iterator on all the labels stored in the `SymbolTable`.
    /// The iterator element is `(&'a Label, &'a Symbol)`.
    pub fn iter(&self) -> Iter<Label, Symbol> {
        self.label_to_symbol.iter()
    }

    /// Adds another SymbolTable to this table.
    pub fn add_table(&mut self, other: &SymbolTable) {
        for symbol in other.symbols() {
            self.add_symbol(symbol.as_str());
        }
    }

    fn from_parsed_symt_text(parsed_symt_text: ParsedTextSymt) -> Result<Self> {
        let num_symbols = parsed_symt_text.pairs.len();
        let mut label_to_symbol: HashMap<Label, Symbol> = HashMap::new();
        let mut symbol_to_label: HashMap<Symbol, Label> = HashMap::new();
        for (symbol, label) in parsed_symt_text.pairs.into_iter() {
            label_to_symbol.insert(label, symbol.clone());
            symbol_to_label.insert(symbol, label);
        }

        Ok(SymbolTable {
            num_symbols,
            symbol_to_label,
            label_to_symbol,
        })
    }

    pub fn from_text_string(symt_string: &str) -> Result<Self> {
        let parsed_symt = ParsedTextSymt::from_string(symt_string)?;
        Self::from_parsed_symt_text(parsed_symt)
    }

    pub fn read_text<P: AsRef<Path>>(&self, path_text_symt: P) -> Result<Self> {
        let parsed_symt = ParsedTextSymt::from_path(path_text_symt)?;
        Self::from_parsed_symt_text(parsed_symt)
    }

    pub fn write_text<P: AsRef<Path>>(&self, path_output: P) -> Result<()> {
        let buffer = File::create(path_output.as_ref())?;
        let mut line_writer = LineWriter::new(buffer);

        write_symt_text!(self, line_writer);

        Ok(())
    }

    /// Writes the text_fst representation of the symbol table into a String.
    pub fn text(&self) -> Result<String> {
        let buffer = Vec::<u8>::new();
        let mut line_writer = LineWriter::new(buffer);
        write_symt_text!(self, line_writer);
        Ok(String::from_utf8(line_writer.into_inner()?)?)
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_symt_text!(self, f);
        Ok(())
    }
}

/// Creates a `SymbolTable` containing the arguments.
/// ```
/// # #[macro_use] extern crate rustfst; fn main() {
/// # use rustfst::SymbolTable;
/// let symt = symt!["a", "b"];
/// assert_eq!(symt.len(), 3);
/// # }
/// ```
#[macro_export]
macro_rules! symt {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = SymbolTable::new();
            $(
                temp_vec.add_symbol($x.to_string());
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symt_write() -> Result<()> {
        let s = symt!("a", "b");
        //        s.write_text("a/symt.txt")?;
        println!("symt = \n{}", s);
        Ok(())
    }
}