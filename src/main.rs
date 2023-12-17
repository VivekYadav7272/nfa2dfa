mod FA {
    use std::collections::HashSet;
    use std::collections::{hash_map::DefaultHasher, HashMap};
    use std::hash::Hash;
    use std::hash::Hasher;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
    pub struct Node(pub usize);

    #[derive(Debug)]
    pub struct NFA {
        starting_node: Node,
        inp_alph: Vec<char>,
        trans_table: HashMap<Node, HashMap<char, Vec<Node>>>,
        final_nodes: Vec<Node>,
    }

    impl NFA {
        pub fn new(starting_node: Node, inp_alph: Vec<char>) -> Self {
            Self {
                starting_node,
                inp_alph,
                trans_table: HashMap::new(),
                final_nodes: vec![],
            }
        }

        pub fn add(&mut self, from: Node, inp: char, to: Node) {
            self.trans_table
                .entry(from)
                .or_default()
                .entry(inp)
                .or_default()
                .push(to);
        }

        pub fn add_final(&mut self, node: Node) {
            self.final_nodes.push(node);
        }

        pub fn to_dfa(&self) -> DFA {
            let mut dfa_trans_table: HashMap<Node, HashMap<char, Node>> = HashMap::new();

            // Doing this will also eliminate creation of any node not reachable via starting_node
            let mut dfa_visited_nodes = HashSet::new();
            let mut dfa_final_nodes = Vec::new();
            self.insert(
                [self.starting_node].as_slice(),
                &mut dfa_trans_table,
                &mut dfa_visited_nodes,
                &mut dfa_final_nodes,
            );

            DFA {
                starting_node: Node(hash(&[self.starting_node].as_slice())),
                trans_table: dfa_trans_table,
                final_nodes: dfa_final_nodes,
            }
        }

        fn insert(
            &self,
            element: &[Node],
            dfa_transition_table: &mut HashMap<Node, HashMap<char, Node>>,
            visited_nodes: &mut HashSet<Node>,
            final_nodes: &mut Vec<Node>,
        ) {
            // for each element in nfa_trans_table, we go to its value, and
            // see that as one whole new element. We say that the element now points to hash(this vector)
            // For solving for hash(this vector), its answer is hash(vec[element_i in this vector])
            let new_element = Node(hash(&element));
            visited_nodes.insert(new_element);

            self.inp_alph.iter().for_each(|inp| {
                println!("For {element:?}, for transition {inp}...");
                let nil = vec![];
                let mut all_parts: Vec<Node> = element
                    .iter()
                    .flat_map(|part| {
                        self.trans_table
                            .get(part)
                            .and_then(|transitions| transitions.get(inp))
                            .unwrap_or(&nil)
                    })
                    .copied()
                    .collect();

                all_parts.sort_unstable();
                let all_parts = all_parts;

                println!("Working on {all_parts:?}..");

                let nodified = Node(hash(&all_parts));

                dfa_transition_table
                    .entry(new_element)
                    .or_default()
                    .insert(*inp, nodified);

                if all_parts.iter().any(|part| self.final_nodes.contains(part)) {
                    final_nodes.push(nodified);
                }

                if !visited_nodes.contains(&nodified) {
                    self.insert(
                        all_parts.as_slice(),
                        dfa_transition_table,
                        visited_nodes,
                        final_nodes,
                    );
                }
            });
        }
    }

    // Don't wanna spend time ensuring how to verify a user's given
    // DFA is a DFA or not, so I'll simply not let the user create a
    // DFA 😌. Instead, the only way to create a DFA is through an NFA,
    // which has a .to_dfa() function.
    #[derive(Debug)]
    pub struct DFA {
        starting_node: Node,
        trans_table: HashMap<Node, HashMap<char, Node>>, // Here, the nodes aren't the old nodes, they're Node(<hash of list of nodes>)
        final_nodes: Vec<Node>,                          // Same here
    }

    impl DFA {
        pub fn is_accepted(&self, text: &str) -> bool {
            let final_node = text.chars().fold(self.starting_node, |node, inp| {
                *self
                    .trans_table
                    .get(&node)
                    .and_then(|node_transitions| node_transitions.get(&inp))
                    .unwrap_or(&Node(usize::MAX))
            });

            self.final_nodes.contains(&final_node)
        }
    }

    fn hash<T: Hash>(t: &T) -> usize {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish() as usize
    }
}
fn main() {
    use FA::*;
    let mut nfa = NFA::new(Node(0), vec!['0', '1']);
    nfa.add(Node(0), '0', Node(1));
    nfa.add(Node(0), '1', Node(1));
    nfa.add(Node(0), '1', Node(0));
    nfa.add_final(Node(1));
    println!("{nfa:?}");

    let dfa = nfa.to_dfa();
    println!("{dfa:?}");
    println!("{}", dfa.is_accepted("1110"));
    println!("{}", dfa.is_accepted("1111"));
    println!("{}", dfa.is_accepted("1011"));
}
