use std::fmt::Display;

#[derive(Eq, PartialEq, Copy, Clone,Debug)]
pub struct Location {
    pub row: u32,
    pub col: u32,
}

impl Location {
    pub fn to_string(&self) -> String {
        return format!("{},{}",&self.col, &self.row);
    }
    pub fn from_str(input: &str) -> Result<Self, String> {
        let parts = input.split(",").filter_map(|s|  s.parse::<u32>().ok()).collect::<Vec<u32>>();
        if parts.len() != 2 {
            return Err(format!("Expected 2 , separated values but found {}", parts.len()));
        }
        return Ok(Location{ row: parts[1], col: parts[0] })
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::result::Result<(), ::std::fmt::Error> {
        f.write_str(format!("({}, {})", &self.col, &self.row).as_str())
    }
}

pub fn locations_to_string(locations: &Vec<Location>) -> String {
    return locations.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(";");
}

pub fn locations_from_string(serialized: &str) -> Result<Vec<Location>,String> {
    let locations: Vec<Location> = serialized
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map( | input: &&str | -> Option<Location> {
            let result = Location::from_str(input);
            return if result.is_ok() {
                Some(result.unwrap())
            } else {
                None
            }
        }).collect();

    let expected_count = serialized
        .split(";").count();

    if expected_count != locations.len() {
        return Err(format!("Expected to deserialize {} locations but only deserialized {}", expected_count, locations.len()));
    }
    return Ok(locations);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::location;
    use crate::location::Location;

    #[test]
    fn to_string() {
        let location = Location { row: 5, col: 7 };
        let serialized = location.to_string();
        assert_eq!(serialized, "7,5");
    }
    #[test]
    fn from_string() {
        let serialized = "9,11";
        let location = Location::from_str(serialized);
        assert!(location.is_ok());
        assert!(location.unwrap().eq(&Location {row: 11, col: 9}));
    }
    #[test]
    fn locations_to_string() {
        let locations: Vec<Location> = vec![
            Location{row: 0, col: 0},
            Location{row: 1, col: 2},
            Location{row: 3, col: 4},
        ];
        let serialized = location::locations_to_string(&locations);
        assert_eq!(serialized, "0,0;2,1;4,3")
    }
    #[test]
    fn locations_from_string() {
        let serialized = "5,6;7,8;9,10";
        let locations = location::locations_from_string(serialized);
        let expected: Vec<Location> = vec![
            Location{row: 6, col: 5},
            Location{row: 8, col: 7},
            Location{row: 10, col: 9},
        ];
        assert!(locations.is_ok());
        assert!(locations.unwrap().eq(&expected));
    }
}