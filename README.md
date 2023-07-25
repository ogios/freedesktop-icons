 # freedesktop-icons-greedy

A fork of [freedesktop-icons](https://github.com/oknozor/freedesktop-icons) that resolves icons even when the theme doesn't have an index. 

 ```rust
 use freedesktop_icons_greedy::lookup;

 let icon = lookup("firefox")
     .with_greed()
     .find();
```