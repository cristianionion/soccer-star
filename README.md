# Soccer Star
## Cristian Ion

Soccer Star is a 2D game written in rust using Bevy. The user gets to take a penalty shot at a soccer goal. The user gets to select the shot's direction and power by pressing buttons on the keyboard. These actions will decide the x-axis, and y-axis destination of the shot. The challenges of this game are to select the right actions from a rapidly moving arrow and power bar. Based on the shot the game will display a message of the outcome. Outcomes are Goal, and misses by Hitting the post, missing wide, missing short, and missing high.

The game is ran by running "cargo run" in terminal. The game is played by taking three actions that determine the shot. First, while the arrow bounces left/right, the user pressed the "x" key to select shot's x-axis result. Second, while the arrow is bounding up/down, the user presses the "y" key to select the shot's height. Lastly, while the powerbar is bounding up/down, the user pressed the "z" key to select the shot's power. The shot's y-axis result is based on a combination of the "y" and "z" selections. The dependency is build from the Cargo.toml file. Testing was done through user testing,
attempting to use bevy_testing broke the program. The program works as desired. Some issues that I came across came from bevy being a lot harder to understand than initially expected, especially since there were more resources from older versions that had functionality that wouldn't work with bevy 0.15.0. bevy_rapier also broke code when trying to implement collisions between the keeper and the ball. There was one cargo clippy error that I wasn't able to resolve where it recommended to simplify the ParamSet that was input to shot_selection(). Future work would be to successfully implement the keeper+ball collision system and to find a way to make bevy_testing work. Future work might include refactoring to an older version of bevy.

(green-grass-background.png and
ball.png are designed by freepic 
https://www.freepik.com/)

![Initial State](initial_state.png)

![GOAL](goal.png)

![Short Miss](missed_short.png)

![Hit Post](hit_post.png)

![Wide Miss](wide_miss.png)

![High Miss](high_miss.png)


MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.