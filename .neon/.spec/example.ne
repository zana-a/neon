enum Person =
  | Teacher
  | Director
  | Student;

func say_hello(person: Person): string =
  switch (person) {
    | Teacher => "Hey Teacher!"
    | Director => "Hey Director!"
    | Student => "Hey Student!"
  };

func say_hello(): unit =
    println("Hello, world!");

struct Car = {
  name: string,
  age: int,
};

let my_number: int = 23;
