# fGrapher
fGrapher is a graphing application heavily inspired by https://www.desmos.com/calculator. It was written in rust using the EGUI crate for the front end and the RSC crate for parsing inputs. The program requires explicit decleration of the operations, e.g. y = 2x would have to be written y = 2*x. There are also currently issues rendering graphs with asymptotes, they are drawn as if they have a fixed height and are sometimes connected over the x-axis.

<img src="fgimg/Screenshot (39).png" alt="Image description">

I was at chapter 7 of the rust book when starting this project and this project was purley a way for me to get some hands on experience with rust. I'm sure there are much better ways to go about error handling and passing around data within the program but I have next to no experience on designing in an expandable and maintainable way. While the overall system design is lacking this project gave me a much better grasp of ownership in rust and I found myself quickly improving at small scale problems within the language. 

<img src="fgimg/Screenshot (41).png" alt="Image description">

fGrapher works by using the RSC crate to parse user input into a usable expression which is then used to generate a vector of points. using EGUI's draw line method these points are connected up to form the line for the function. This technique works, and was reasonably performant after caching these points but is far from perfect. A linear line for example could be represented with 2 points insetad the several thousand points the program generates. This system is also repsonible for the problems with asymptotes, the program currently only can identify these if an error is thrown while computing a point. If the points are just slightly after or before an asymptote it is missed which can cause extra lines on asymptotes where there should be none. 

<img src="fgimg/Screenshot (40).png" alt="Image description">
