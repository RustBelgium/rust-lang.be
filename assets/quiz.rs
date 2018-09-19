// This code comes from a quiz presented by Pieter Penninckx on the
// 3rd Belgian Rust user meetup on September 4, 2018 in Brussels, Belgium.
// This source code contains the questions of the quiz.
// You can compile the code and run the result to check your answers.
// The subject of the quiz is the `Drop` trait.
// This quiz is only meaningful if you know about the `Drop` trait.

// Troughout the quiz, we will be using the following data structure.
struct Droppable {
    message: &'static str
}

impl Droppable {
    fn new(message: &'static str) -> Self {
        Droppable{message}
    }
    fn get_message(&self) -> &'static str {
        self.message
    }
    fn get_bool(&self, b: bool) -> bool {
        b
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("{}", self.message);
    }
}

// This macro is just to make this code compile.
macro_rules! question {
    (fn $fname:ident() $body:block) => {
    	  println!("______________________");
    	  println!();
          fn $fname() $body
          let a = stringify!($body);
          println!("fn {}()", stringify!($fname));
          print!("{}", beautify(a));
          println!();
          println!();
          println!("\x1B[1mResult:");
          println!("\x1B[31m");
          $fname();
          println!("\x1B[0m");
          println!();
    }
}

fn main() {
	question!(
		fn question_1() {
			let a = Droppable::new("A");
			println!("B");
		}
	);

	question!(
		fn question_2() {
			let _a = Droppable::new("A");
			println!("B");
		}
	);

	question!(
		fn question_3() {
			let _ = Droppable::new("A");
			println!("B");
		}
	);

	question!(
		fn question_4() {
			let a = &Droppable::new("A");
			println!("B");
		}
	);

	question!(
		fn question_5() {
			{
				let a = Droppable::new("A");
			}
			
			println!("B");
		}
	);

	question!(
		fn question_6() {
			let a = { Droppable::new("A") };
			println!("B");
		}
	);

	question!(
		fn question_7() {
			let a = Droppable::new("A");
			let b = Droppable::new("B");
		}
	);
	
	question!(
		fn question_8() {
			let mut d = Droppable::new("A");
			d = Droppable::new("B");
		}
	);	

	question!(
		fn question_9() {
			struct S {
				f1: Droppable,
				f2: Droppable
			}
	
			let s = S {
				f2: Droppable::new("A"), f1: Droppable::new("B") 
			};
		}
	);
	
	struct DroppableContainer {
		message: &'static str,
		contained: Option<Droppable>
	}
	
	impl Drop for DroppableContainer {
		fn drop(&mut self) {
			println!("{}", self.message);
		}
	}

	question!(
		fn question_10() {
			let mut container = DroppableContainer{
				message: "B",
				contained: None
			};
			container.contained = Some(Droppable::new("A"));
		}
	);

	question!(
		fn question_11() {
			let a = Droppable::new("A").get_bool(true);
			println!("B");
		}
	);

	question!(
		fn question_12() {
			if Droppable::new("A").get_bool(true) {
				println!("B");
			}
		}
	);

	question!(
		fn question_13() {
			match Droppable::new("A").get_bool(true) {
				true => { println!("B"); },
				false => {unreachable!();}
			}
		}
	);

	question!(
		fn question_14() {
		    let mut has_executed: bool = false;
			while Droppable::new("A").get_bool(!has_executed) {
				println!("B");
				has_executed = true;
			}
		}
	);
}

// Below are some helper methods. I hacked this together in about half an hour,
// so it has many rough edges.

fn beautify(s: &'static str) -> String {
	fn add_newline(s: &mut String, indent: i32) {
		s.push('\n');
		for _ in 0..(indent*4)-1 {
			s.push(' ');
		}
	}
	let mut result = String::new();
	let mut indent = 0;
	let mut must_add_newline = false;
	
	for c in s.chars() {
		// Before printing character.
		if must_add_newline || c == '}' {
			match c {
				' ' | '\n' => {},
				'}' => {
					indent -= 1;
					must_add_newline = false;
				}
				_ => {
					must_add_newline = false;
				}
			}
			if ! must_add_newline {
				result.push('\n');
				for _ in 0..indent*4 {
					result.push(' ');
				}
				result.push(c);
			}
		}
		else {
			result.push(c);
		}
		// Determine next
		match c {
			'{' => {
				indent += 1;
				must_add_newline = true;
			},
			';' => {
				must_add_newline = true;
			},
			_ => {}
		}
	}
	return result;
}
