use crate::location::Location;

#[derive(Eq, PartialEq, Clone)]
pub struct Shot {
    pub location: Location,
    pub hit: bool
}

impl Shot {
    fn to_string(&self) -> String {
        return format!("{}|{}",self.location.to_string(),self.hit);
    }

    fn from_str(str: &str) -> Result<Self, String> {
        let parts:Vec<String> = str.split("|").filter_map(|s| Option::from(String::from(s))).collect::<Vec<String>>();
        if parts.len() != 2 {
            return Err(format!("Expected to find 2 fields, found {}.", parts.len()));
        }
        let location = Location::from_str(parts[0].as_str())?;
        let hit = match parts[1].as_str() {
            "true" => true,
            "True" => true,
            _ => false
        };

        return Ok(Shot{location, hit})
    }
}

#[cfg(test)]
mod tests {
    use crate::location::Location;
    use crate::shot::Shot;

    #[test]
    fn shot_to_string() {
        let shot = Shot{ location: Location{row:3,col:5}, hit: false };
        let str = shot.to_string();
        assert_eq!(str, "5,3|false");
    }

    #[test]
    fn shot_from_string() {
        let shot = Shot::from_str("5,2|true").unwrap();
        assert!(Location{row:2,col:5}.eq(&shot.location));
        assert!(&shot.hit);
    }
}
