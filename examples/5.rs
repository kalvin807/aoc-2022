use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
};

type ContainerStack = Vec<char>;
type State = HashMap<u32, ContainerStack>;

struct Action {
    from: u32,
    to: u32,
    times: u32,
}

fn extract_container(sub_str: String) -> Option<char> {
    // can be either [X]  or 3 spaces (no container)
    // [A-Z], remove bracket get the middle char
    match sub_str.chars().nth(1).unwrap() {
        ' ' => None,
        c => Some(c),
    }
}

fn parse_graph(mut graph: Vec<String>) -> State {
    // Strictly following this format:
    // [X] [Y] ... [Z] <- containers' tower, container always look like "[A-Z]"
    // ...
    // 1 2 3 ... n as tower index
    let mut state = HashMap::new();
    // from bottom to top, as it is stack
    let idxs: Vec<u32> = graph
        .pop()
        .unwrap()
        .split_whitespace()
        .filter(|str| !str.trim().is_empty())
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    for i in &idxs {
        state.insert(*i, vec![]);
    }

    graph.reverse();
    for ln in graph {
        // Each row will have exact chars for all towers (aka 3 char * idx + 1 spacer * idx - 1)
        let mut row = ln.chars().clone();
        for i in &idxs {
            // take 3 chars
            let container = row.by_ref().take(3).collect();
            print!("{}", container);
            if let Some(c) = extract_container(container) {
                state.get_mut(i).unwrap().push(c)
            }
            // advance one space
            row.next();
        }
        println!();
    }
    println!("{:#?}", state);
    state
}

fn parse_action(str: String) -> Action {
    let splitted = str.split_whitespace().into_iter().collect::<Vec<&str>>();
    Action {
        times: splitted[1].parse::<u32>().unwrap(),
        from: splitted[3].parse::<u32>().unwrap(),
        to: splitted[5].parse::<u32>().unwrap(),
    }
}

// move one container once a time, as it is stack
fn move_one_container(mut state: State, action: &Action) -> State {
    for _ in 0..action.times {
        // 300 iq compiler will check if one mut ref is used, it will not allow another mut ref
        // Rust rules on mut ref
        // See https://stackoverflow.com/questions/64311263/why-does-order-of-mutable-borrows-matter-in-rust
        let from = state.get_mut(&action.from).unwrap();
        if !from.is_empty() {
            let top = from.pop().unwrap();
            // here, from is out of scope so compiler let us borrow another mut again
            let to = state.get_mut(&action.to).unwrap();
            to.push(top);
        }
    }
    state
}

// move multi containers once a time, so container retain their order
fn move_multi_container(mut state: State, action: &Action) -> State {
    // dirty hack from og method, double the time complexity...
    let mut buffer = vec![];
    for _ in 0..action.times {
        let from = state.get_mut(&action.from).unwrap();
        if !from.is_empty() {
            let top = from.pop().unwrap();
            buffer.push(top);
        }
    }
    for _ in 0..action.times {
        let to = state.get_mut(&action.to).unwrap();
        if !buffer.is_empty() {
            let top = buffer.pop().unwrap();
            to.push(top);
        }
    }
    state
}

fn run_actions(mut state: State, actions_str: Vec<String>) -> State {
    let actions: Vec<Action> = actions_str.into_iter().map(parse_action).collect();

    for action in actions {
        state = move_multi_container(state, &action);
    }

    state
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("examples/5-input");
    println!("reading path: {}", path.display());
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(&file).lines().flatten();
    let mut graph: Vec<String> = Vec::new();
    let mut actions: Vec<String> = Vec::new();
    let mut is_graph_end = false;
    for ln in lines {
        if ln.is_empty() {
            is_graph_end = true;
            continue;
        }
        if is_graph_end {
            actions.push(ln);
        } else {
            graph.push(ln);
        }
    }

    let mut state = parse_graph(graph);
    state = run_actions(state, actions);
    let mut keys = state.keys().collect::<Vec<_>>();
    keys.sort();
    for idx in &keys {
        print!("{} : ", idx);
        for c in state.get(idx).unwrap() {
            print!("[{}] ", c);
        }
        println!();
    }
    for idx in &keys {
        print!("{}", state.get(idx).unwrap().last().unwrap());
    }
    println!();
}
