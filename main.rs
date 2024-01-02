use std::vec;
use std::fmt;
use std::collections::HashMap;

// use nalgebra;
// use nalgebra;
use nalgebra::DMatrix;
extern crate nalgebra as na;

#[derive(Clone, Debug, PartialEq)]
struct Node{
    loc:na::Vector2<f64>,
    id: usize,
    line_ids: Vec<usize>,
    force:na::Vector2<f64>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {}: ({}, {}) | Lines ({:?})", self.id, self.loc.x, self.loc.y,self.line_ids)
    }
}


#[derive(Clone, Debug, PartialEq)]
struct Nodes{
    items: HashMap<usize,Node>,
    next_id: usize,
}

impl Nodes {
    fn new() -> Self {
        Nodes {
            items: HashMap::new(),
            next_id: 0,
        }
    }

    fn from_locations(locations: Vec<(f64, f64)>, forces: Vec<(usize,f64,f64)>) -> Self {
        let mut nodes = Nodes::new();
        for (x, y) in locations {
            nodes.add_node(na::Vector2::new(x, y));
        }
        for (id,fx,fy) in forces{
            nodes.items.get_mut(&id).unwrap().force=na::Vector2::new(fx,fy)
        }
        nodes
    }

    fn add_node(&mut self, loc: na::Vector2<f64>) {
        let id = self.next_id;
        self.next_id += 1;
        let node = Node { loc, id, line_ids:Vec::new(),force: na::Vector2::new(0.0,0.0) };
        self.items.insert(id, node);
    }
    

}

// impl fmt::Display for Nodes {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\n")?;
//         for (id, node) in &self.items {
//             write!(f, "Node {}: ({}, {})\n", id, node.loc.x, node.loc.y)?;
//         }
//         Ok(())
//     }
// }

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        let mut ids: Vec<&usize> = self.items.keys().collect();
        ids.sort();
        for id in ids {
            let node = &self.items[id];
            write!(f, "Node {}: ({}, {})\n", id, node.loc.x, node.loc.y)?;
        }
        Ok(())
    }
}

 

#[derive(Clone, Debug, PartialEq)]
struct Line{
    nodes: [Node; 2],
    direction: na::Vector2<f64>,
    force: f64,
    // colour: [usize; 3],
    id: usize
}

impl Line {
    fn new(nodes: [Node; 2], id: usize) -> Self {
        // //println!("node1: {}, node2: {}",nodes[0].loc,nodes[1].loc);
        // let distance=(nodes[1].loc-nodes[0].loc).normalize();
        // //println!("{}",distance);
        let nodes0=nodes[0].clone();
        let nodes1=nodes[1].clone();

        Line {
            nodes,
            direction: (nodes1.loc-nodes0.clone().loc).normalize(),
            force: 0.0,
            id,
        }
    }
}



impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Node 1: ({}, {}), Node 2: ({}, {}), Force: {}",
            self.id,
            self.nodes[0].loc.x, self.nodes[0].loc.y,
            self.nodes[1].loc.x, self.nodes[1].loc.y,
            self.force)
    }
}


#[derive(Clone, Debug, PartialEq)]
struct Lines{
    items: HashMap<usize,Line>,
    next_id: usize
}

impl Lines {
    fn new() -> Self {
        Lines {
            items: HashMap::new(),
            next_id: 0,
        }
    }

    fn add_line(&mut self,all_nodes: &mut Nodes, line: Line) {
        let id = self.next_id;
        self.next_id += 1;
        all_nodes.items.get_mut(&line.nodes[0].id).unwrap().line_ids.push(id);
        all_nodes.items.get_mut(&line.nodes[1].id).unwrap().line_ids.push(id);
        self.items.insert(id, line);
    }

    fn from_locations(nodes:&mut Nodes,locations:Vec<(usize,usize)>)->Self{
        let mut lines = Lines::new();
        // //println!("{}",nodes);
        for (id1, id2) in locations {
            // //println!("{:?}, {:?}",nodes.items.get(&id1), nodes.items.get(&id2));
            if let (Some(node1), Some(node2)) = (nodes.items.get(&id1), nodes.items.get(&id2)) {
                // Clone the nodes if you intend to store them separately
                let node1 = node1.clone();
                // //println!("{}",node1);
                let node2 = node2.clone();
                // //println!("{}",node2);
                lines.add_line(nodes,Line::new([node1, node2], lines.next_id));
            } 
            else {
                //println!("Node IDs {} or {} not found", id1, id2);
            }
        }
        lines
    }

}


// impl fmt::Display for Lines {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\n")?;
//         for (_id, line) in &self.items {
//             write!(f, "{}\n", line)?;
//         }
//         Ok(())
//     }
// }

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        let mut ids: Vec<&usize> = self.items.keys().collect();
        ids.sort();
        for id in ids {
            let line = &self.items[id];
            write!(
                f,
                "Line {}: Node {}, Node {}, Force: {:.3}\n",
                id, char::from_u32((line.nodes[0].id+65) as u32).unwrap(), char::from_u32((line.nodes[1].id+65) as u32).unwrap(), line.force
            )?;
        }
        Ok(())
    }
}

// struct Support{
//     node: Node,
//     force: na::Vector2<f64>,
//     direction: char
// }



fn main(){
    let now = std::time::Instant::now();
    let locations:Vec<(f64, f64)>=vec![(0.0,0.0),(0.0,1.0),(1.0,1.0),(2.0,1.0),(3.0,1.0),(4.0,1.0),(4.0,0.0),(3.0,0.0),(2.0,0.0),(1.0,0.0)];
    let node_locations:Vec<(usize,usize)>=vec![(0,1),(0,9),(1,2),(1,9),(2,3),(2,8),(2,9),(3,4),(3,8),(4,5),(4,7),(4,8),(5,6),(5,7),(6,7),(7,8),(8,9)];
    let forces:Vec<(usize,f64,f64)>=vec![(2,0.0,-1.0),(0,0.0,0.75),(6,0.0,0.25)];
    // let supports:Vec<(usize,char)>=vec![(0,'B'),(3,'V')];
    let mut all_nodes=Nodes::from_locations(locations,forces);
    let mut all_lines=Lines::from_locations(&mut all_nodes,node_locations);
    
    //println!("Nodes: {}", all_nodes);
    //println!("Lines: {}", all_lines);


    let rows = 2*all_nodes.items.len();
    let columns = all_lines.items.len();

    let mut matrix :DMatrix<f64>= DMatrix::zeros(rows, columns);
    let mut vector :DMatrix<f64>=DMatrix::zeros(rows,1);



    for i in 0..(rows/2){

        let node = &all_nodes.items[&i];
        let row = 2*i;

        // println!("{}",node.id);


        //println!("{}",node);
        vector[(row,0)]=-node.force.x;
        vector[(row+1,0)]=-node.force.y;


        
        for column in 0..columns{
            let line = &all_lines.items[&column];
            // if row != 99{
            //     println!("line:{}",id);
            // }
            if node.line_ids.contains(&column){
                if line.nodes[0].id==node.id{
                    matrix[(row,column)]=line.direction.x;
                    matrix[(row+1,column)]=line.direction.y;
                }
                else if line.nodes[1].id==node.id{
                    matrix[(row,column)]=-line.direction.x;
                    matrix[(row+1,column)]=-line.direction.y;
                }
                
            }
        }
        
        
    }
    

    // println!("{}",matrix);
    // println!("{}",vector);

    let forces: DMatrix<f64>;

    if let Some(a_t_a_inv) = (matrix.transpose() * &matrix).try_inverse() {
        forces = a_t_a_inv * matrix.transpose() * vector;
        // println!("Solution: {:?}", forces);
        for (n,force) in forces.iter().enumerate(){
            if let Some(line) = all_lines.items.get_mut(&n) {

                line.force = *force;

                
            }
        }
    } else {
        println!("Error forming least squares solution");
    }
    


   

    //let forces=matrix.least_squares(&vector).unwrap();
    //println!("{}",forces);

    // println!("Nodes: {}", all_nodes);
    println!("Lines: {}", all_lines);

















    let elapsed_time = now.elapsed();
    println!("Runtime of {} microseconds.", elapsed_time.as_micros());

}


