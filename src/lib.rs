#![crate_name = "fast_list"]

//! # A fast & thread-safe linked list.
//!
//! - On average ~2-3x faster than `std::collections::LinkedList` for all operations.
//! - On average ~2-3x faster than `Vec` & `VecDeque` for random insertions (random removals are about the same as of now)
//! - Only slightly slower than `Vec` & `VecDeque` for most other operations.
//! - Safe against [ABA problem] by using a [SlotMaps] internally, which means you can safely iterate & mutate the list across multiple threads. An advantage over just using a SlotMap is that the order when iterating is not arbitrary.
//! - Using indices into a stack allocated arena (slotmap) instead of pointers for improved cache locality.
//! - Written in 100% safe Rust.
//! 
//! # Structure
//! [Node] - Struct representing a node in the graph. Contains a [NodeID] which is a key to the node in the slotmap, which has a generic data field and a list of edges.
//!
//! [Edge] - Struct representing an edge in the graph. Contains an [EdgeID] which is a key to the edge in the slotmap, and two [NodeID]s which are the nodes the edge connects (from & to). An edge can also have "data", which could be anything or nothing; for example the weight of the connection or a struct or enum representing something else.
//!
//! [GraphInterface] - Trait defining methods to alter a graph, i.e. adding, removing, and editing nodes and edges.
//!
//!
//! [Graph] - The default graph struct which implements [GraphInterface]. It only contains two slotmaps, one for nodes and one for edges.
//!
//! [Categorized] - Trait that extends the [Graph] with category specific methods.
//!
//! [CategorizedGraph] - A graph with categories. Categories are normal nodes (which can contain edges & data), but the graph also contains a hashmap that maps category names to category nodes for easy access.
//!
//!
//! # Examples
//!
//! ## Simple [Graph] and the ABA problem.
//!
//! ```
//! use fast_graph::{Graph, Node, Edge};
//! /* We need to have this trait in scope: */
//! use fast_graph::{GraphInterface};
//!
//! #[derive(Debug, Clone)]
//! struct EdgeData(String);
//!
//! #[derive(Debug, Clone)]
//! struct NodeData(String);
//!
//! let mut graph: Graph<NodeData, EdgeData> = Graph::new();
//!
//! let node1 = graph.add_node(NodeData("Node 1".into()));
//! let node2 = graph.add_node(NodeData("Node 2".into()));
//! let edge1 = graph.add_edge(node1, node2, EdgeData("Edge 1".into()));
//!
//! assert_eq!(graph.node(node1).unwrap().id, node1);
//! assert_eq!(graph.edge(edge1).unwrap().id, edge1);
//!
//! graph.remove_node(node1).unwrap();
//!
//! // Since we just removed node 1, it should be None now.
//! assert!(graph.node(node1).is_err());
//! // And node 2 still points to node 2.
//! assert_eq!(graph.node(node2).unwrap().id, node2);
//!
//! println!("{:#?}", graph);
//!
//! ```
//!

mod basic_linked_list;
mod linked_list;
mod walker;
mod linked_list_cell;

pub use linked_list::*;

pub use basic_linked_list::LinkedList as BasicLinkedList;
pub use walker::*;
