package hello_world

use stdlib.io.console
use stdlib.lang.string
use stdlib.util.range

struct Person do
  name: string,
  age: int
end

enum Pet do
  Dog,
  Cat
end

def main() do
  say_hello()
  say_hello(Person {name: "Zana", age: 24})
  say_hello(Pet.Dog)
  count_to(15)
end

def say_hello() do
  "!noeN ,olleH"
    ->> String.reverse()
    ->> Console.puts()
end

def say_hello(person: Person) do
  let {name, age} = person
  "Hey #{name}, you are #{age} years old"
    ->> Console.puts()
end

def say_hello(pet: Pet) do
  let pet: Pet = match pet do
    Dog -> "doggy",
    Cat -> "kittie"
  end

  "Aww... you are a cute #{pet}!"
    ->> Console.puts
end

def count_to(amount: int) do
  for i <- Range.inclusive(amount) do
    Console.puts(i)
  end
end
