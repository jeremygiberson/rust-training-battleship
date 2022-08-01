use std::str::FromStr;
use std::fmt::Display;
use crate::location;
use crate::location::Location;

#[derive(Eq,PartialEq,Copy,Clone)]
pub enum ShipType {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Display for ShipType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::result::Result<(), ::std::fmt::Error> {
        match *self {
            ShipType::Carrier => {f.write_str("Carrier")}
            ShipType::Battleship => {f.write_str("Battleship")}
            ShipType::Cruiser => {f.write_str("Cruiser")}
            ShipType::Submarine => {f.write_str("Submarine")}
            ShipType::Destroyer => {f.write_str("Destroyer")}
        }
    }
}

impl ShipType {
    pub fn size(&self) -> u32 {
        match *self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 2,
            ShipType::Destroyer => 2,
        }
    }
    fn from_str(input: &str) -> Result<Self, String> {
        match input {
            "Carrier"  => Ok(ShipType::Carrier),
            "Battleship"  => Ok(ShipType::Battleship),
            "Cruiser"  => Ok(ShipType::Cruiser),
            "Submarine" => Ok(ShipType::Submarine),
            "Destroyer" => Ok(ShipType::Destroyer),
            _      => Err(format!("Could not convert {} to ShipType", input)),
        }
    }
    fn to_string(&self) -> String {
        return match &self {
            ShipType::Carrier => String::from("Carrier"),
            ShipType::Battleship => String::from("Battleship"),
            ShipType::Cruiser => String::from("Cruiser"),
            ShipType::Submarine => String::from("Submarine"),
            ShipType::Destroyer => String::from("Destroyer"),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Ship {
    pub class: ShipType,
    pub locations: Vec<Location>,
    pub hits: u32,
}

impl Ship {
    pub fn new(class: ShipType) -> Self {
        return Ship {
            class,
            locations: Vec::new(),
            hits: 0,
        }
    }
    pub fn carrier() -> Self { Ship::new(ShipType::Carrier) }
    pub fn battleship() -> Self { Ship::new(ShipType::Battleship) }
    pub fn cruiser() -> Self { Ship::new(ShipType::Cruiser) }
    pub fn submarine() -> Self { Ship::new(ShipType::Submarine) }
    pub fn destroyer() -> Self { Ship::new(ShipType::Destroyer) }

    pub fn sunk(&self) -> bool {
        return &self.hits == &self.class.size();
    }

    pub fn to_string(&self) -> String {
        let str = format!(
            "{}|{}|{}",
            self.class.to_string(),
            self.hits,
            location::locations_to_string(&self.locations),
        );
        return str;
    }

    pub fn from_str(input: &str) -> Result<Self, String> {
        let parts = input.split("|").collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(format!("Expected serialized ship to contain 3 | separated values, found {}", parts.len()));
        }
        let class = ShipType::from_str(parts[0])?;
        let hits_result = parts[1].parse::<u32>();
        if hits_result.is_err() {
            return Err(format!("Could not parse hits, expected u32, found {}", parts[1]));
        }
        let hits = hits_result.unwrap();
        let locations = location::locations_from_string(parts[2])?;
        if locations.len() != class.size().try_into().unwrap() {
            return Err(format!("Expected {} serialized locations, found {}", class.size(), locations.len()));
        }

        Ok(Ship{class, hits, locations})
    }
}

pub fn ships_to_string(ships: &Vec<Ship>) -> String {
    return ships.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("&");
}

pub fn ships_from_str(serialized: &str) -> Result<Vec<Ship>,String> {
    let ships: Vec<Ship> = serialized
        .split("&")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map( | input: &&str | -> Option<Ship> {
            let result = Ship::from_str(input);
            return if result.is_ok() {
                Some(result.unwrap())
            } else {
                None
            }
        }).collect();

    let expected_count = serialized
        .split(";").count();

    if expected_count != ships.len() {
        return Err(format!("Expected to deserialize {} ships but only deserialized {}", expected_count, ships.len()));
    }
    return Ok(ships);
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::location::Location;
    use crate::ship::{Ship, ships_to_string, ShipType};

    #[test]
    fn ship_type_to_string() {
        assert_eq!(ShipType::Battleship.to_string(), String::from("Battleship"));
        assert_eq!(ShipType::Carrier.to_string(), String::from("Carrier"));
        assert_eq!(ShipType::Cruiser.to_string(), String::from("Cruiser"));
        assert_eq!(ShipType::Destroyer.to_string(), String::from("Destroyer"));
        assert_eq!(ShipType::Submarine.to_string(), String::from("Submarine"))
    }
    #[test]
    fn ship_type_from_string() {
        let battleship = ShipType::from_str(&String::from("Battleship"));
        let carrier = ShipType::from_str(&String::from("Carrier"));
        let cruiser = ShipType::from_str(&String::from("Cruiser"));
        let destroyer = ShipType::from_str(&String::from("Destroyer"));
        let submarine = ShipType::from_str(&String::from("Submarine"));
        assert!(battleship.is_ok());
        assert!(battleship.unwrap().eq(&ShipType::Battleship));

        assert!(carrier.is_ok());
        assert!(carrier.unwrap().eq(&ShipType::Carrier));

        assert!(cruiser.is_ok());
        assert!(cruiser.unwrap().eq(&ShipType::Cruiser));

        assert!(destroyer.is_ok());
        assert!(destroyer.unwrap().eq(&ShipType::Destroyer));

        assert!(submarine.is_ok());
        assert!(submarine.unwrap().eq(&ShipType::Submarine));
    }

    #[test]
    fn ship_to_string() {
        let ship = Ship{
            class: ShipType::Cruiser,
            hits: 1,
            locations: vec![
                Location{row: 0, col: 0},
                Location{row: 1, col: 2},
                Location{row: 3, col: 4},
            ]};
        let serialized = ship.to_string();
        assert_eq!(serialized, "Cruiser|1|0,0;2,1;4,3")
    }

    #[test]
    fn ship_from_string() {
        let serialized = "Destroyer|2|0,0;2,1";
        let ship = Ship::from_str(serialized);
        assert!(ship.is_ok());
        assert!(ship.unwrap().eq(&Ship{
            class: ShipType::Destroyer,
            hits: 2,
            locations: vec![
                Location{row: 0, col: 0},
                Location{row: 1, col: 2},
            ]
        }))
    }

    #[test]
    fn serialize_ships_to_string() {
        let ships: Vec<Ship> = vec![
            Ship{
                class: ShipType::Submarine,
                locations: vec![
                    Location{ row: 0, col: 0 },
                    Location{ row: 1, col: 0 }
                ],
                hits: 0
            },
            Ship{
                class: ShipType::Destroyer,
                locations: vec![
                    Location{ row: 0, col: 1 },
                    Location{ row: 1, col: 1 }
                ],
                hits: 0
            }
        ];
        let string = ships_to_string(&ships);
        assert_eq!(string, String::from("Submarine|0|0,0;0,1&Destroyer|0|1,0;1,1"));
    }
}