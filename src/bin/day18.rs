use std::fmt::Display;

const INPUT: &str = include_str!("../../input/day18.txt");

const TEST_INPUT: &str = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
";

const TEST_A: &str = "[[[[[9,8],1],2],3],4]";
const TEST_B: &str = "[7,[6,[5,[4,[3,2]]]]]";
const TEST_C: &str = "[[6,[5,[4,[3,2]]]],1]";
const TEST_D: &str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

const SUM_INPUT_A: &str = "[1,1]
[2,2]
[3,3]
[4,4]";

const SUM_INPUT_B: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

const MAGNITUDE_TEST: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

fn main() {
    println!("day 18");
    let numbers: Vec<_> = INPUT
        .lines()
        .map(|line| {
            ParseState::new(line.chars().rev().collect())
                .parse()
                .unwrap()
        })
        .map(|mut n| {
            n.reduce();
            n
        })
        .collect();

    {
        let mut sum = numbers
            .iter()
            .cloned()
            .reduce(|a, b| {
                let mut new = Element::Pair(Box::new(a), Box::new(b));
                new.reduce();
                new
            })
            .unwrap();
        sum.reduce();
        let magnitude = sum.magnitude();
        println!("parta: {}", magnitude);
    }
    {
        let largest_mag = numbers
            .iter()
            .enumerate()
            .flat_map(|(i, number)| {
                std::iter::repeat(number)
                    .zip(numbers.iter().enumerate().filter(move |(j, _n)| *j != i))
                    .map(|(a, (j, b))| (a, b))
            })
            .map(|(a, b)| {
                let mut new = Element::Pair(Box::new(a.clone()), Box::new(b.clone()));
                new.reduce();
                new.magnitude()
            })
            .max()
            .unwrap();
        println!("partb: {}", largest_mag);
    }
}

#[derive(Debug)]
enum ParseState {
    Empty(String),
    First(String),
    Second(Element, String),
    Closed(Element, String),
}

impl ParseState {
    fn new(inner: String) -> ParseState {
        ParseState::Empty(inner)
    }
    fn unwrap(self) -> Element {
        use ParseState::*;
        // println!("finished parsing {:?}", self);
        match self {
            Closed(e, r) => {
                assert!(r.len() == 0);
                e
            }
            _ => panic!("expected parsing to be finished"),
        }
    }
    fn partial_unwrap(self) -> (Element, String) {
        use ParseState::*;
        match self {
            Closed(e, r) => {
                // assert!(r.len() != 0);
                (e, r)
            }
            _ => panic!("expected parsing to be finished"),
        }
    }
    fn parse(mut self) -> ParseState {
        while !self.is_closed() {
            self = self.parse_inner();
        }
        self
    }
    fn is_closed(&self) -> bool {
        use ParseState::*;
        match self {
            Closed(..) => true,
            _ => false,
        }
    }
    fn parse_inner(self) -> ParseState {
        // line.chars().fold(ParseState::Empty, |state, next| {
        // println!("recursing over {:?}", self);
        use ParseState::*;
        match self {
            Empty(mut rest) => {
                // expect open paren
                let next = rest.pop().unwrap();
                if next == '[' {
                    First(rest)
                } else {
                    panic!("unexpected input");
                }
            }
            First(mut rest) => {
                // look for number or open paren
                let next = rest.pop().unwrap();
                if next == '[' {
                    // println!("desceding on first element");
                    let (a, rest) = First(rest).parse().partial_unwrap();
                    // println!("asceding, got {:?}", a);
                    Second(a, rest)
                } else {
                    let digit = next.to_digit(10).map(|u| u as i64);
                    if let Some(num) = digit {
                        let e = Element::Number(num);
                        Second(e, rest)
                    } else {
                        panic!("unexpected input");
                    }
                }
            }
            Second(first, mut rest) => {
                // expect comma and then another element
                let next = rest.pop().unwrap();
                if next == ',' {
                    // println!("descending on second element");
                    let (second, rest) = ParseState::First(rest).parse().partial_unwrap();
                    // println!("ascending, got {:?}", second);
                    let e = Element::Pair(Box::new(first), Box::new(second));
                    Closed(e, rest)
                } else if next == ']' {
                    Closed(first, rest)
                } else {
                    panic!("unexpected input: {}", next);
                }
            }
            s @ Closed(_, _) => s,
        }

        // })
    }
}

#[derive(Debug, Clone)]
enum Element {
    Number(i64),
    Pair(Box<Element>, Box<Element>),
}

impl Element {
    fn magnitude(&self) -> i64 {
        use Element::*;
        match self {
            Number(n) => *n,
            Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
    fn reduce(&mut self) {
        while self.reduce_inner() {}
    }
    fn reduce_inner(&mut self) -> bool {
        use Element::*;
        if let (i, Some(e)) = self.deeply_nested(4) {
            let f = e.clone();
            *e = Element::Number(0);
            match f {
                Number(_) => false,
                Pair(a, b) => {
                    if i > 0 {
                        self.add(i - 1, a.unwrap_number());
                    }

                    self.add(i + 1, b.unwrap_number());
                    true
                }
            }
        } else if let Some(e) = self.high_value() {
            match e {
                Number(n) => {
                    let number = *n;
                    let half = number / 2;
                    let (high, low) = if number & 0b1 == 0 {
                        (half, half)
                    } else {
                        (half, half + 1)
                    };
                    let high = Box::new(Element::Number(high));
                    let low = Box::new(Element::Number(low));
                    *e = Pair(high, low);
                }
                Pair(..) => panic!("unexpected code path"),
            }
            true
        } else {
            false
        }
    }
    fn unwrap_number(&self) -> i64 {
        use Element::*;
        match self {
            Number(n) => *n,
            Pair(..) => panic!("called unwrap on a nested pair"),
        }
    }
    /// adds the provided number to the node at the given index. if this node was not found in this branch, the number of skipped leaves is returned instead. return None if we found the desired leaf
    fn add(&mut self, index: usize, add: i64) -> Option<usize> {
        use Element::*;
        match self {
            Number(n) => {
                if index == 0 {
                    *n += add;
                    None
                } else {
                    Some(1)
                }
            }
            Pair(a, b) => {
                if let Some(skipped) = a.add(index, add) {
                    if let Some(skipped_b) = b.add(index - skipped, add) {
                        Some(skipped + skipped_b)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
    /// returns a reference to the leftmost element which is nested 4 deep
    /// as well as the "index" of that element (counting from the left, disregarding depth)
    fn deeply_nested(&mut self, depth: usize) -> (usize, Option<&mut Element>) {
        use Element::*;
        if depth == 0 {
            match self {
                Number(_) => (1, None),
                Pair(..) => (0, Some(self)), //panic!("unexpected depth")
            }
        } else {
            match self {
                Number(_) => (1, None),
                Pair(a, b) => {
                    let (ai, a_deep) = a.deeply_nested(depth - 1);
                    if a_deep.is_some() {
                        (ai, a_deep)
                    } else {
                        let (bi, b_deep) = b.deeply_nested(depth - 1);
                        if b_deep.is_some() {
                            (ai + bi, b_deep)
                        } else {
                            (ai + bi, None)
                        }
                    }
                }
            }
        }
    }
    fn high_value(&mut self) -> Option<&mut Element> {
        use Element::*;
        match self {
            Number(n) if *n >= 10 => Some(self),
            Number(_) => None,
            Pair(a, b) => {
                let a_high = a.high_value();
                if a_high.is_some() {
                    a_high
                } else {
                    let b_high = b.high_value();
                    if b_high.is_some() {
                        b_high
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Element::*;
        match self {
            Number(n) => write!(f, "{}", n),
            Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}
