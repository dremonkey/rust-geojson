// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson, Object};
use Poly;

/// Polygon
///
/// [GeoJSON Format Specification § 2.1.6](http://geojson.org/geojson-spec.html#polygon)
#[derive(RustcEncodable, Clone)]
pub struct Polygon {
    pub coordinates: Poly
}

impl ToJson for Polygon {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Polygon".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

impl Polygon {
    pub fn from_json(json_geometry: &Object) -> Polygon {
        let json_poly = json_geometry.get("coordinates").unwrap();
        let coordinates = Poly::from_json(json_poly.as_array().unwrap());
        return Polygon{coordinates: coordinates};
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::{Json, ToJson};
    use {Polygon, Poly, Pos, Ring};

    #[test]
    fn test_polygon_to_json() {
        let polygon = Polygon{coordinates: Poly(vec![
            Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
            Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
            ])};
        let json_string = format!("{}", polygon.to_json());
        assert_eq!("{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"Polygon\"}", json_string);
    }

    #[test]
    fn test_polygon_from_json() {
        let json_string = "{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"Polygon\"}";
        let json_doc = Json::from_str(json_string).unwrap();
        let polygon = Polygon::from_json(json_doc.as_object().unwrap());
        assert_eq!(json_string, format!("{}", polygon.to_json()));
    }
}