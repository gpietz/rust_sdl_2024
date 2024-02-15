# RustSdl2024

In recent years, I've experimented with the SDL2 library in conjunction with Rust and OpenGL. This year, I aim to revisit and further develop the source codes I've accumulated on this topic.

## Project status 13.02.2024
Today, I delved into basic transformations, with a significant focus on mastering cgmath, which was the most time-consuming part. The initial transformation test produced unexpected results, highlighting that basing rotation degrees solely on seconds leads to choppy animations - 
a method I've realized is not optimal. However, the provided screenshots serve as evidence that, when executed correctly, colored vertices, multitexturing and transformations are indeed possible with OpenGL. It's all about getting the technique right. :)

![screenshot_20240213](https://github.com/gpietz/rust_sdl_2024/assets/77841571/8f88484a-2a95-46db-8147-59f7022f50b0)

## Project status 11.02.2024
Today, I successfully completed the multitexturing topic and can now focus on transformations. To simplify multitexturing in the future, I need to come up with a solution. The way it's solved now requires considering too much.

![screenshot_20240211](https://github.com/gpietz/rust_sdl_2024/assets/77841571/ce659fec-04eb-42e4-9cec-cab2f6ee5656)


## Project status 01.02.2024
Over the past few days, I've been wrestling with shaders, which initially led to a few unsightly bugs, but now everything 
is smooth sailing, and I've also managed to implement uniforms pretty swiftly. Thanks to ChatGPT, I always have a buddy 
to explain the connections, making it fun to bring ideas to life. 
However, with each new insight, my awe for the necessary data wrangling grows. But hey, it's fun, so I'm sticking with it! 

![screenshot_m2_20240201](https://github.com/gpietz/rust_sdl_2024/assets/77841571/8876fad5-2219-4db2-b7a1-61d7240fa2c6)  
 

## Project status 28.01.2024
I am currently very satisfied with and confident in the project's progress. The newly structured BufferObject is highly flexible, capable of managing various data formats. 
I have successfully ported two drawables from the old project to this new structure. These can be toggled using the F1 and F2 keys. I plan to continue developing the project further.

![screenshot_m1_20240205](https://github.com/gpietz/rust_sdl_2024/assets/77841571/69ede581-c57f-492c-a663-180d265f6d08)
