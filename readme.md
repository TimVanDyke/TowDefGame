# Protect Joe
A tower defense game ~~and minimal game engine~~ written with code reuse for future games in mind executed in Rust!

## Game play: 
The object of the game is to protect our main character Joe. You are playing as Joe's immune system in an attempt to prevent the viruses, bacterias, fungus, and other dangerous outside forces from getting to Joe's heart. 

~~Some ideas are to add real time elements to the game as well during the rounds to stop stab wounds and bullets~~. Maybe in a future release after Beta.

## Libraries used: 
- https://github.com/Rust-SDL2/rust-sdl2 (For window and OpenGL context creation, inputs, and sound)
- https://github.com/brendanzab/gl-rs/tree/master/gl_generator (for creating my "personal" opengl crate and letting me create a pointer to OpenGL)
- https://github.com/rustsim/nalgebra (For doing all the vector math)
- https://github.com/image-rs/image-png (handling png things)
- https://github.com/redox-os/rusttype (for fonts and typing to the screen)
- ~~https://github.com/rustgd/cgmath~~ (Most people are using nalgebra now)
- ~~https://github.com/PistonDevelopers/glfw-rs~~ (Turns out https://github.com/Nercury/rust-and-opengl-lessons was may more elegant than I thought after learning more about OpenGL and I went with his general project suggestions)
- others: See cargo.toml as well as the inner cargo.toml for the other libraries I've pulled in from https://github.com/Nercury/rust-and-opengl-lessons . He's done a truly excellent job laying out safe OpenGL in Rust and I can't thank him enough for helping me with this project. 

## Dependencies: 
- cmake needs to be installed for sdl2 (https://cmake.org/download/) 

## References for helping me understand concepts: 
### Rust: 
- https://www.rust-lang.org/ (official rust documentation)
- https://doc.rust-lang.org/stable/book/title-page.html (Official FREE Rust textbook)
- once I pulled all the dependencies I was able to run ```cargo doc --open``` which built all the docs in a searchable offline html format which was very helpful for learning how the API's worked
### OpenGL: 
- The Cherno's OpenGL series! https://www.youtube.com/watch?v=W3gAzLwfIP0&list=PLlrATfBNZ98foTJPJ_Ev03o2oq3-GGOS2 (This was by far the best OpenGL tutorial I have ever seen ever. It is in CPP. So all the OpenGL is relevant, but doing it in Rust is quite different as far as project setup goes.)
### OpenGL in Rust: 
- http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html (Another resource I found for helping me get started, ~~decided not to go this path~~ This was invaluable and incredibly elegant and I ended up using much of this guy's suggestions for how to handle OpenGL in Rust. Without this I would never have been able to finish this project even close to ontime. )
- ~~https://michaelfairley.com/blog/i-made-a-game-in-rust/~~ (Helped me get started, but I don't recommend his crates list)