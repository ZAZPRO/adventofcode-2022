use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

// Struct to hold coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

// Struct to hold each point on the map.
#[derive(Debug, Clone, Copy, PartialEq)]
struct HeightPoint {
    id: u32,
    position: Point,
    height: u32,
}

impl HeightPoint {
    fn new(id: u32, position: Point, height: u32) -> Self {
        Self {
            id,
            position,
            height,
        }
    }
}

// Struct to hold each graph node.
#[derive(Debug, Clone, PartialEq)]
struct Node {
    // Node id.
    id: u32,
    // List of neighbors ids and heuristics of path to this neighbor id from current Node.
    neighbors: Vec<(u32, u32)>,
    // Node heuristics.
    heuristic: u32,
    // Node id of node heuristics value.
    origin: Option<u32>,
}

impl Node {
    fn new(id: u32, neighbors: Vec<(u32, u32)>, heuristic: u32, origin: Option<u32>) -> Self {
        Self {
            id,
            neighbors,
            heuristic,
            origin,
        }
    }
}

// Node Graph is a hashmap of <K, V> where K is Node id and Node is a Node stuct.
type Graph = HashMap<u32, Node>;

// Function to create a new graph out of the height map.
fn new_graph(height_map: Vec<HeightPoint>, x_max: u32, y_max: u32) -> Graph {
    // Node graph that will be returned.
    let mut graph: Graph = Graph::new();
    // Copy of the height map.
    let height_map_copy = height_map.clone();

    // For each point on the map.
    for point in height_map_copy {
        // Get current point position.
        let pos = point.position;
        // Variable to store all neighbor points. An option because point might not have all 4 neighbors.
        let mut neighbor_points: [Option<&HeightPoint>; 4] = [None; 4];

        // Check if up neighbor can exist.
        if pos.y > 0 {
            // Coordinates of that neighbor.
            let pos_check = Point::new(pos.x, pos.y - 1);
            // Find it in the List and store in in the array.
            neighbor_points[0] = height_map.iter().find(|p| p.position == pos_check);
        }

        // Check if down neighbor can exist.
        if pos.y < y_max {
            // Coordinates of that neighbor.
            let pos_check = Point::new(pos.x, pos.y + 1);
            // Find it in the List and store in in the array.
            neighbor_points[1] = height_map.iter().find(|p| p.position == pos_check);
        }

        // Check if left neighbor can exist.
        if pos.x > 0 {
            // Coordinates of that neighbor.
            let pos_check = Point::new(pos.x - 1, pos.y);
            // Find it in the List and store in in the array.
            neighbor_points[2] = height_map.iter().find(|p| p.position == pos_check);
        }

        // Check if right neighbor can exist.
        if pos.x < x_max {
            // Coordinates of that neighbor.
            let pos_check = Point::new(pos.x + 1, pos.y);
            // Find it in the List and store in in the array.
            neighbor_points[3] = height_map.iter().find(|p| p.position == pos_check);
        }

        // Variable to hold a pair of neighbors ids and neighbors heuristics.
        let mut neighbors: Vec<(u32, u32)> = Vec::new();

        // For each neighbor.
        for neighbor_point in neighbor_points {
            // Match to see if there is a number.
            match neighbor_point {
                Some(p) => {
                    // Default heuristic value. (Infinity like)
                    let mut heuristic: u32 = u32::MAX;

                    // If we can move from current point to neighbor_point -> set heuristic to 1.
                    if (point.height as i32 - p.height as i32) >= -1 {
                        heuristic = 1;
                    }

                    // Insert this neighbor to neighbor list.
                    neighbors.push((p.id, heuristic));
                }
                None => (),
            }
        }

        // Create new Node using all the data available.
        let node = Node::new(point.id, neighbors, u32::MAX, None);
        // Insert this node to the graph.
        graph.insert(node.id, node);
    }

    // Return graph
    graph
}

// Modify graph by setting start node heuristic to 0.
fn set_starting_point(graph: &mut Graph, starting_point_id: u32) {
    let starting_node = graph.get_mut(&starting_point_id).unwrap();
    starting_node.heuristic = 0;
}

// My implementation of Dijkstra's algorithm.
// Take a Node graph and Node id of the target.
fn dijkstra(graph: &mut Graph, target_id: u32) {
    // HashSet that holds all of the processed nodes.
    let mut processed: HashSet<u32> = HashSet::new();
    // Copy of the graph to use when we need so referencies.
    let graph_clone = graph.clone();
    // Dijkstra queue of nodes that holds node id and node heuristics.
    let mut queue: Vec<(u32, u32)> = Vec::new();
    // Insert each node from the graph to the queue.
    for v in graph.values() {
        queue.push((v.id, v.heuristic));
    }

    // Infinite loop.
    loop {
        // Sort queue by heuristics.
        queue.sort_by_key(|v| v.1);
        // Reverse list so shortest distance is last.
        queue.reverse();
        // Retrieve and remove last value from the list.
        let current_queue_item = queue.pop().unwrap();
        // If current_queue_item id is the same as target -> we are done.
        if current_queue_item.0 == target_id {
            break;
        }

        // Get current Node from the graph.
        let current_node = graph_clone.get(&current_queue_item.0).unwrap();

        // For each of current_node neighbors.
        for (neighbor_id, to_neighbor_heuristic) in current_node.neighbors.clone() {
            // If we can reach this neighbor from current node.
            // And if current node is not a dead_end.
            // And we are not done with this neighbor.
            if to_neighbor_heuristic != u32::MAX
                && current_queue_item.1 != u32::MAX
                && !processed.contains(&neighbor_id)
            {
                // Get current neighnor Node from the graph.
                let mut current_neighbor = graph.get_mut(&neighbor_id).unwrap();

                // Calculate new heuristics value.
                let new_heuristic = current_queue_item.1 + to_neighbor_heuristic;
                // If new value smaller than current one.
                if new_heuristic < current_neighbor.heuristic {
                    // Hey we found new shorter path!
                    // Update current_neighbor heuristics.
                    current_neighbor.heuristic = new_heuristic;
                    // Save where do we get this shortest path from (current node id).
                    current_neighbor.origin = Some(current_node.id);
                    // Remove old value from queue.
                    queue.retain(|v| v.0 != neighbor_id);
                    // Push new one.
                    queue.push((neighbor_id, new_heuristic));
                }
            }
        }

        // We have finished all business with current node. So we can add it to a processed HashSet.
        processed.insert(current_node.id);
    }
}

// Program entry point.
fn main() -> Result<(), Box<dyn Error>> {
    // List that will hold a whole map.
    let mut height_map: Vec<HeightPoint> = Vec::new();
    // Store HeightPoint of end point.
    let mut end_point: Option<HeightPoint> = None;

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Amount of lines is a total of y coordinates.
    let y_max = file.lines().count() as u32 - 1;
    // Amount of chars in fist line is a total of x coordinates.
    let x_max = file.lines().next().unwrap().chars().count() as u32 - 1;

    // Variable that will be used to give each point unique id.
    let mut point_id: u32 = 0;
    // For each new line inside of that string.
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // Treat each character as x and each line as y.
            let point = match c {
                // If it's a start.
                'S' => {
                    // Create new HeightPoint with 0 height.
                    HeightPoint::new(point_id, Point::new(x as u32, y as u32), 0)
                }
                // If it's end.
                'E' => {
                    // Create new HeightPoint with 25 height.
                    let p = HeightPoint::new(point_id, Point::new(x as u32, y as u32), 25);
                    // Save this point as ending one.
                    end_point = Some(p);
                    p
                }
                // If it's other letter.
                'a'..='z' => {
                    // Calculate current letter height.
                    let height: u32 = ((c as i32) - 97).abs() as u32;
                    // Create new HeightPoint with calculated height.
                    HeightPoint::new(point_id, Point::new(x as u32, y as u32), height as u32)
                }
                _ => todo!("Wrong input."),
            };

            // Insert a height into the list.
            height_map.push(point);

            // Increment point id.
            point_id += 1;
        }
    }

    // Calculate initial graph.
    let initial_graph = new_graph(height_map.clone(), x_max, y_max);

    // Find all points with zero height. Those are our starting points.
    // In my puzzle input I can filter those to first few rows of the map.
    // That's because any other zero height point is isolated in my case.
    // As I am not sure if it's true for other puzzles and alghoritm is fast enough.
    // I will just check every starting point. (Takes still one pass with dijkstra).
    let starting_points: Vec<&HeightPoint> = height_map.iter().filter(|n| n.height == 0).collect();

    // Copy this graph to use it in finding path to the end point.
    let mut graph = initial_graph.clone();

    // For each starting point.
    for starting_point in starting_points.iter() {
        // Set each starting point in the graph.
        set_starting_point(&mut graph, starting_point.id);
    }

    // Calculate shortest path to end the point.
    dijkstra(&mut graph, end_point.unwrap().id);

    // Get end node out of the processed tree.
    let end_node = graph.get(&end_point.unwrap().id).unwrap();

    // As all paths that are possible to take are 1.
    // And Heuristics is a sum of all points on the path before.
    // Final result is the heuristics result.
    println!("Result: {}", end_node.heuristic);

    Ok(())
}
