# Mystery Box - Can You Survive 3 Out of 10

## Author
Yihui T.

Welcome to **Mystery Box - Can You Survive 3 Out of 10**, a dystopian game developed in Rust.

Are you lucky enough to choose 3 among 10 mystery boxes within one minute and still survive? 

The odds are stacked against you, let's see how lucky you are.

## Highlights

- **Character Movement:** You can guide your character in one of three directions: forward, backward left, or right.
- **Mystery Boxes:** The map have 10 hidden boxes, each with a concealed outcome. Choose wisely whether to open a box or pass it by, but remember, your choices are irreversible.
- **Limited Field of View:** You will only see the mystery box within the certain distance.
- **Randomized Outcomes:** Mystery boxes deliver unpredictable results. You might encounter peril, lose health points, gain healing, or experience nothing at all. The outcomes are entirely random.
- **Survival Challenge:** Your objective is to make the 'right' choices and open three boxes without succumbing to harm. Achieve a positive health balance to succeed.
- **Time Constraints:** Time is not your ally. You have just one minute to explore, select boxes, and make decisions. Fail to open three boxes within this timeframe, and the game ends with your character's demise.

## How to Play

- Use the arrow keys to move your character (forward, backward, left, or right).
- Approach a mystery box, press space to open it or don't.
- Make RANDOM choices to ensure your character survives with a positive health balance.

## How to Build and Run
1. Clone the repository: `git clone https://github.com/GIR7/Mystery-Box_Can-You-Survive-3-Out-of-10.git`
2. Navigate to the project directory: `cd Mystery-Box_Can-You-Survive-3-Out-of-10`
3. Build the project: `cargo build`
4. Run the game: `cargo run`

## Testing
To run tests, use the following command: `cargo test`

## Example
White square represents the player, green square represents the mystery box.
For a quick example of the game's operation, run the project and use arrow keys to move the player and the space bar to interact with boxes.

## What Worked, What Didn't, and Future Improvements
The game successfully implements basic functionality, such as player movement, box interactions, limited field of view and win/lose conditions. However, there would be room for improvement in terms of user interface, animation effect，sounds effect, additional game features, and overall polish.

I also found it is hard to have unit test in terms of the ggez game engine

## License

