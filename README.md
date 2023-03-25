# fish-lang

A simple interpreter for a simple language.
To build use `cargo install`, then run `fish-lan code.txt` inside your console to try it out!


Example programs:
```
print("This is name_checker_1000");
print("What is your name?");
input name;
if (name == "Fish") {
  print("You are the best!")
} else {
  print("You are not the best, "+name+"!")
};
```

```
print("Hello World");

if ((1+5) >= (2*2)) {
  print("1+5 is greater than or equal to 2*2");
};

index = 0;
end = 10;
while ((index +=1) < end) {
  print("Currently at:");
  print(index);
}
```
