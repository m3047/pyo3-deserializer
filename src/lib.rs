// Copyright (c) 2020 Fred Morris, Tacoma WA 98445 USA
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use pyo3::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

// Tests are written in the Python test wrapper.
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

/// This module is implemented in Rust.
#[pymodule]
fn wtrack_base(_py: Python, m: &PyModule) -> PyResult<()> {
    // The `_py` argument represents that we're holding the GIL.
    m.add_class::<BaseDevice>()?;

    Ok(())
}

const DATA_TIMESTAMP:   usize   = 0;
const DATA_FREQUENCY:   usize   = 1;
const DATA_SIGLEVEL:    usize   = 2;
const DATA_PAKTYPE:     usize   = 3;
const DATA_SUBTYPE:     usize   = 4;
const DATA_SOURCE:      usize   = 5;
const DATA_DEST:        usize   = 6;
const DATA_ATTRS:       usize   = 7;

const NO_SIGNAL:        i32     = 100;

const ATTR_STATION:     &str    = "0";

/// Base Device (observation) record class.
///
/// Represents one tab-separated line in a sensor observations file.
#[pyclass]
struct BaseDevice {
    #[pyo3(get)]
    record: String,
    #[pyo3(get)]
    fields: Vec<(usize,usize)>,
    #[pyo3(get)]
    attrs: HashMap<String, String>,
}

#[pymethods]
impl BaseDevice {
    #[new]
    fn new(record: String) -> Self {

        let mut start = 0;
        let mut end = 0;
        let mut fields = Vec::new();
        for c in record.chars() {
            if c == '\t' {
                fields.push( ( start, end )  );
                start = end + 1;
            }
            end += 1;
        }
        if record.ends_with('\n') {
            end -= 1;
        }
        fields.push( (start, end ) );
        
        let mut i = DATA_ATTRS;
        let end = fields.len();
        let mut attrs = HashMap::new();
        while i+1 < end {
            let k_bounds = fields[i];
            let v_bounds = fields[i+1];
            attrs.insert(record[k_bounds.0..k_bounds.1].to_string(),
                         record[v_bounds.0..v_bounds.1].to_string()
                        );
            i += 2;
        }
        
        let obj = BaseDevice { 
                        record: record,
                        fields: fields,
                        attrs:  attrs
            };
        obj
    }
    
    #[getter]
    fn get_timestamp(&self) -> f64 {
        let bounds = self.fields[DATA_TIMESTAMP];
        match f64::from_str(&self.record[bounds.0..bounds.1]) {
            Ok(v) => v,
            Err(_) => -1.0
        }
    }
    #[getter]
    fn get_frequency(&self) -> String {
        let bounds = self.fields[DATA_FREQUENCY];
        self.record[bounds.0..bounds.1].to_string()
    }
    #[getter]
    fn get_signal(&self) -> i32 {
        let bounds = self.fields[DATA_SIGLEVEL];
        let v = match i32::from_str(&self.record[bounds.0..bounds.1]) {
            Ok(v) => v,
            Err(_) => NO_SIGNAL
        };
        v.abs()
    }
    #[getter]
    fn get_type(&self) -> Option<u32> {
        let bounds = self.fields[DATA_PAKTYPE];
        match u32::from_str(&self.record[bounds.0..bounds.1].to_string()) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
    #[getter]
    fn get_subtype(&self) -> Option<u32> {
        let bounds = self.fields[DATA_SUBTYPE];
        match u32::from_str(&self.record[bounds.0..bounds.1].to_string()) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
    /// Returns a (type, subtype) tuple.
    #[getter]
    fn get_packet_type(&self) -> (Option<u32>, Option<u32>) {
        let type_bounds = self.fields[DATA_PAKTYPE];
        let subtype_bounds = self.fields[DATA_SUBTYPE];
        let ptype = match u32::from_str(&self.record[type_bounds.0..type_bounds.1].to_string()) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        let stype = match u32::from_str(&self.record[subtype_bounds.0..subtype_bounds.1].to_string()) {
            Ok(v) => Some(v),
            Err(_) => None
        };
        ( ptype, stype )
    }
    #[getter]
    fn get_src(&self) -> String {
        let bounds = self.fields[DATA_SOURCE];
        self.record[bounds.0..bounds.1].to_string()
    }
    #[getter]
    fn get_dest(&self) -> String {
        let bounds = self.fields[DATA_DEST];
        self.record[bounds.0..bounds.1].to_string()
    }
    
    /// Retrieve an Attribute.
    ///
    /// Expects the attribute specifier (as a string).
    /// If nothing is found returns None.
    fn attr(&self, attribute: &str) -> Option<&String> {
        self.attrs.get(attribute)
    }
    
    /// Report the number of fields as Rust sees it.
    fn fields_length(&self) -> usize {
        self.fields.len()
    }
    
    /// Is the record valid?
    fn valid(&self) -> bool {
        self.fields.len() >= DATA_ATTRS
    }
    
    #[getter]
    fn station(&self) -> String {
        let raw_station = match self.attrs.get(ATTR_STATION) {
            Some(v) => v,
            None => ""
        };
        lazy_static! {
            static ref CLEAN_STATION: Regex = Regex::new(r"[\\\\]x..").unwrap();
        }
        let cleaned = CLEAN_STATION.replace_all(raw_station,".");
        cleaned.trim().to_string()
    }
    
    #[getter]
    fn ap(&self) -> bool {
        let type_bounds = self.fields[DATA_PAKTYPE];
        let subtype_bounds = self.fields[DATA_SUBTYPE];
        &self.record[type_bounds.0..type_bounds.1] == "0" && &self.record[subtype_bounds.0..subtype_bounds.1] == "8"
    }
}
