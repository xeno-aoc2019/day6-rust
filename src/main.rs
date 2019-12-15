use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;


#[derive(Clone, Hash, Eq)]
struct SpaceObject {
    name: String,
    orbits: String,
}

impl fmt::Display for SpaceObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for SpaceObject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// impl Eq for SpaceObject {}

fn main() {
    println!("Hello, world!");
    let space_objects = read_deps();
    let COM = SpaceObject { name: "COM".into(), orbits: "COM".into() };
    let by_name = map_by_name(&space_objects);
    let by_satellite = map_by_satellite(&space_objects, &by_name);

    let o1 = SpaceObject { name: "foo".into(), orbits: "bar".into() };
    let o2 = SpaceObject { name: "foo".into(), orbits: "bar".into() };

    println!("{} == {} => {}", o1, o2, o1 == o2);

    for key in by_satellite.keys() {
        println!("{} ( {}", key, by_satellite.get(key).unwrap());
    }
    let by_center = map_by_center(&space_objects);


    let solution_1 = sum_orbits(&COM, 0, &by_center);
    println!("Task 1: {}", solution_1)
}


fn sum_orbits(current: &SpaceObject, current_orbits: i32, by_center: &HashMap<&SpaceObject, Vec<&SpaceObject>>) -> i32 {
    let mut sum = current_orbits;
    for satellite in by_center.get(current).unwrap() {
        sum += sum_orbits(&satellite, current_orbits+1, &by_center);
    }
    sum
}

fn map_by_name(objects: &Vec<SpaceObject>) -> HashMap<String, SpaceObject> {
    let mut map = HashMap::new();
    for o in objects {
        map.insert(o.name.clone(), o.clone());
    }
    map
}

fn map_by_center(objects: &Vec<SpaceObject>) -> HashMap<&SpaceObject, Vec<&SpaceObject>> {
    let mut map = HashMap::new();
    for center in objects {
        let mut satellites = vec!();
        for other in objects {
            if other.orbits == center.name && other.name != center.name {
                satellites.push(other);
            }
        }
        map.insert(center, satellites);
    }
    map
}

fn map_by_satellite(objects: &Vec<SpaceObject>, by_name: &HashMap<String, SpaceObject>) -> HashMap<SpaceObject, SpaceObject> {
    let mut map = HashMap::new();
    for o in objects {
        let orbits = by_name.get(&o.orbits);
        match orbits {
            Some(center) => {
                println!("found {}", center);
                map.insert(o.clone(), center.clone());
            }
            None => {
                println!("Not found for {}", o.orbits);
            }
        }
    }
    map
}

fn read_deps() -> Vec<SpaceObject> {
    let mut space_objects = vec!();
    if let Ok(lines) = read_lines("input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let v: Vec<&str> = line.split(')').collect();
                space_objects.push(SpaceObject { name: v[1].into(), orbits: v[0].into() })
            }
        }
    }
    space_objects.push(SpaceObject { name: "COM".into(), orbits: "COM".into() });
    space_objects
}

fn read_lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}