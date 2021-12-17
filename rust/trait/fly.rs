struct Duck;
struct Pig;

trait Fly {
	fn fly(&self) -> bool;
}

impl Fly for Duck {
	fn fly(&self) -> bool {
		return true;
	}
}

impl Fly for Pig {
	fn fly(&self) -> bool {
		return false;
	}
}

fn fly_static<T: Fly>(s: T) -> bool {
	s.fly()
}

fn fly_dyn(s: &dyn Fly) -> bool {
	s.fly()
}

fn main() {
	let pig = Pig;
	println!("pig={}", fly_static::<Pig>(pig));
	println!("pig={}", fly_dyn(&Pig));

	let duck = Duck;
	println!("duck={}", fly_dyn(&duck));
	println!("duck={}", fly_static::<Duck>(duck));
}
