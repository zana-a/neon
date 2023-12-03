enum Species {
  Dog,
  Cat
}

type Animal {
  name: string,
  species: Species,
}

def create_dog(name: string) -> Animal {
  return Animal {
    name: name,
    species: Species::Dog
  }
}

let jerry_dog -> Animal = create_dog("Jerry")

#[
create_dog("Jerry")
  ->> @String.reverse()
  ->> @String.to_list()
  ->> @List.at(3)
  ->> @Func.predicate((s: string) -> s == "e")
  ->> @Func.consume((b: bool) -> {
    if b == true {
      @IO.puts("Hello, world")
    }
  })
#]

using Neon.String
using Neon.List
using Neon.Func
using Neon.IO

create_dog("Jerry")
  ->> @String.reverse()
  ->> @List.to_list()
  ->> @List.at(3)
  ->> @Func.predicate((s: string) -> s == "e")
  ->> @Func.consume((b: bool) -> {
    if b == true {
      @IO.puts("Hello, world")
    }
  })
