pub fn get_loc(world: Vec<u32>, x: u32, y:u32)-> usize{ //Get the index
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut index: usize = 0;
    for (i, n) in world.iter().enumerate() {
        if col==x&&row==y {
            index = i;
            break;
        }
        if *n==0 {
            row+=1;
            col=0;
        } else {
            col+=1;
        }
        
        
    }
    index
}
pub fn get_loc_coll(world: Vec<u8>, x: u32, y:u32)-> usize{ //Get the index
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut index: usize = 0;
    for (i, n) in world.iter().enumerate() {
        if col==x&&row==y {
            index = i;
            break;
        }
        if *n==2 {
            row+=1;
            col=0;
        } else {
            col+=1;
        }
    }
    index
}
pub fn check_collision(x:usize, y:usize, collision_map: &Vec<u8>)-> bool{ //Check for collisions
    let line = get_collision_line(&collision_map, y as u32);
   if line[x]==0{
       true
   } else {
       false
   }
}
pub fn get_line_count(world: &Vec<u32>) -> u32 { //Get the lines of the map
    let mut count = 0;
    for i in world.iter() {
        if *i==0 {count+=1;}
    }
    count
}
pub fn get_collision_line(world: &Vec<u8>, line_number: u32)-> Vec<u8>{ //Get a Vector of a line in the collision_map
    let mut line_index: u32 = 0;
    let mut line: Vec<u8> = Vec::new();
    for i in world.iter() {
        if *i==2 { line_index+=1; } else if line_index==line_number {
            line.push(*i);
        }
    }
    line
}
pub fn get_line(world: Vec<u32>, line_number: u32)-> Vec<u32>{ //Get a Vector of a line in the world map
    let mut line_index: u32 = 0;
    let mut line: Vec<u32> = Vec::new();
    for i in world.iter() {
        if *i==0 { line_index+=1; } else if line_index==line_number {
            line.push(*i);
        }
    }
    line
}