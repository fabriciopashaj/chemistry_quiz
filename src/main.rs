use std::io::{self, Write, BufRead};
use std::mem;
use rand::Rng;

#[derive(Debug)]
struct Element {
    pub name:          &'static str,
    pub symbol:        &'static str,
    pub atomic_number: u8
}

#[derive(Debug)]
enum Question {
    SYMBOL,
    NAME
}

#[derive(Debug)]
struct Answer<'a> {
    pub question: Question,
    pub answer: String,
    pub element: &'a Element
}

impl Question {
    #[inline]
    pub fn from(q: u32) -> Self {
        if q == 0 { Question::SYMBOL } else { Question::NAME }
    }
}
impl<'a> Answer<'a> {
    pub fn new(
        i: u32,
        question: Question,
        element: &'a Element
    ) -> Self {
        print!("[Question No.{}]: ", i + 1);
        match question {
            Question::SYMBOL => {
                print!(
                    "What is the symbol of the element called {}? ",
                    element.name
                );
            }
            Question::NAME => {
                print!(
                    "What is called the element with the symbol {}? ",
                    element.symbol
                );
            }
        }
        io::stdout().flush().unwrap();
        Answer{
            question,
            element,
            answer: io::stdin().lock().lines().next().unwrap().unwrap()
        }
    }
    pub fn correct(&self) -> &'static str {
        match self.question {
            Question::SYMBOL => self.element.symbol,
            Question::NAME   => self.element.name,
        }
    }
    pub fn is_correct(&self) -> bool {
        self.correct() == self.answer
    }
}

fn main() {
    let elements = [
        Element {
            name: "Hydrogen",
            symbol: "H",
            atomic_number: 1
        },
        Element {
            name: "Helium",
            symbol: "He",
            atomic_number: 2
        },
        Element {
            name: "Lithium",
            symbol: "Li",
            atomic_number: 3
        },
        Element {
            name: "Berillium",
            symbol: "Be",
            atomic_number: 4
        },
        Element {
            name: "Borium",
            symbol: "B",
            atomic_number: 5
        },
        Element {
            name: "Carbon",
            symbol: "C",
            atomic_number: 6
        },
        Element {
            name: "Nytrogen",
            symbol: "N",
            atomic_number: 7
        },
        Element {
            name: "Oxygen",
            symbol: "O",
            atomic_number: 8
        },
        Element {
            name: "Fluorium",
            symbol: "F",
            atomic_number: 9
        },
        Element {
            name: "Neon",
            symbol: "Ne",
            atomic_number: 10
        },
        Element {
            name: "Sodium",
            symbol: "Na",
            atomic_number: 11
        },
    ];
    println!("{}", mem::size_of::<[Element; 11]>());
    let length = elements.len();
    // let mut rng = rand::thread_rng();
    let mut question = rand::thread_rng();
    let mut answers = Vec::with_capacity(length);
    let mut wrong_answers = 0;
    for i in 0..length {
        let element = &elements[rand::thread_rng().gen_range(0..length)];
        answers.push(
            Answer::new(
                i as u32,
                Question::from(question.gen_range(0..2)),
                element
            )
        );
    }
    for i in 0..length {
        if !answers[i].is_correct() {
            println!("------------------------------------------------");
            println!("Question No.{} wasn't answered correctly", i + 1);
            println!("Expected: {}", answers[i].correct());
            println!("Got: {}", answers[i].answer);
            wrong_answers += 1;
        }
    }
    println!("======================================================");
    println!("{}% of questions were answered correctly",
             (length - wrong_answers) * 100 / length);
    match wrong_answers * 100 / length {
        0       => println!("Perfect score! Well done!"),
        1..=20  => println!("Good, but more careful next time."),
        21..=40 => println!("You need to read more."),
        41..=60 => println!("You won't pass class like this."),
        _       => println!("I'm disappointed. You have failed me.")
    }
    println!("======================================================");
}
