# Protect Joe
A tower defense game and minimal game engine written in Rust!

## Game play: 
The object of the game is to protect our main character Joe. You are playing as Joe's immune system in an attempt to prevent the viruses, bacterias, fungus, and other dangerous outside forces from getting to Joe's heart. 

Some ideas are to add real time elements to the game as well during the rounds to stop stab wounds and bullets. 

## Libraries used: 
- https://github.com/PistonDevelopers/glfw-rs (GLFW bindings and other window creation things)
- https://github.com/image-rs/image-png (handling png things)
- https://github.com/redox-os/rusttype (for fonts and typing to the screen)
- https://github.com/rustgd/cgmath (for game vector math)
- ~~https://github.com/Rust-SDL2/rust-sdl2~~ (ended up not going this route)
- ~~https://github.com/brendanzab/gl-rs/tree/master/gl_generator~~ (ended up not going this route)


## Dependencies: 
- ~~cmake needs to be installed for sdl2 (https://cmake.org/download/)~~ I ended up not going this path and used GLFW instead which did not require external dependencies on the system. 

## References for helping me understand concepts: 
- https://www.rust-lang.org/ (official rust documentation)
- https://doc.rust-lang.org/stable/book/title-page.html (Official FREE Rust textbook)
- https://michaelfairley.com/blog/i-made-a-game-in-rust/ (Helped me build a list of libraries to use)
- https://www.youtube.com/playlist?list=PLlrATfBNZ98foTJPJ_Ev03o2oq3-GGOS2 (What ultimately taught me OpenGL. I cannot recommend this tutorial or channel enough.)
- http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html (Another resource I found for helping me get started, ~~decided not to go this path~~ This was invaluable and incredibly elegant and I ended up using much of this guy's suggestions for how to handle OpenGL in Rust)
- once I pulled all the dependencies I was able to run ```cargo doc --open``` which built all the docs in a searchable offline html format which was very helpful for learning how the API's worked
