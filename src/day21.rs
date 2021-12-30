pub fn day21(){
    //Player 1 starting position: 10
    //Player 2 starting position: 7
    let result = game(
                    Player::new(10),
                    Player::new(7),
                        DeterministicDice::new());
    println!("Day 21 , 1 : {}", &result);
    let result2 = game_quantum_dice(Player::new(10),
                                    Player::new(7));
    println!("Day 21 , 2 : {}", &result2);
}

fn game_quantum_dice (player1 : Player, player2:Player)-> u64{
    let mut player1_won = 0;
    let mut player2_won = 0;
    let mut universes : Vec<(Player, Player, u64)> = vec![(player1, player2, 1)];

    while !universes.is_empty() {
        let mut new_universes = Vec::new();
        for (player1, player2, u) in universes.iter() {
            for (steps, universes) in crate_universes() {
                let mut new_player :Player = player1.clone();
                new_player.move_steps(&steps);
                let new_universe_count = u * universes;
                if new_player.get_score() >= 21{
                    player1_won += new_universe_count;
                } else {
                    new_universes.push((new_player, player2.clone(), new_universe_count));
                }
            }
        }
        universes = new_universes;
        let mut new_universes = Vec::new();
        for (player1, player2, u) in universes.iter() {
            for (steps, universes) in crate_universes() {
                let mut new_player :Player = player2.clone();
                new_player.move_steps(&steps);
                let new_universe_count = u * universes;
                if new_player.get_score() >= 21{
                    player2_won += new_universe_count;
                } else {
                    new_universes.push((player1.clone(), new_player, new_universe_count));
                }
            }
        }
        universes = new_universes;
    }

    u64::max(player1_won, player2_won)
}

fn crate_universes() -> Vec<(u32, u64)> {
    let mut universes:Vec<(u32,u64)> = Vec::new();
    universes.push((3,1));  //(1/1/1)
    universes.push((4,3)); //(1/1/2) ..

    universes.push((5, 6)); //(1/1/3) .., (1/2/2) ..
    universes.push((6, 7));    //(1/2/3), (1/3/2), (2/1/3), (2/3/1), (3/2/1), (3/1/2),(2/2/2)
    universes.push((7, 6));


    universes.push ((8,3));
    universes.push((9, 1));
    //entspricht 27 universen = 3³
    universes
}


fn game(mut player1: Player, mut player2: Player, mut die: DeterministicDice) -> u64 {
    let looser_score;
    loop {
        player1.move_steps(&die.roll_three_times());
        if player1.get_score() >= 1000 {
            looser_score = player2.get_score();
            break;
        }
        player2.move_steps(&die.roll_three_times());
        if player2.get_score() >= 1000 {
            looser_score = player1.get_score();
            break;
        }
    }
    looser_score * die.count
}
#[derive(Clone)]
struct Player{
    field : u32,
    score : u64,
}

impl Player{
    fn new (field : u32) -> Self{
      Player{
          field : field - 1 ,
          score: 0
      }
    }

    fn move_steps (&mut self, steps : &u32){
        self.field = (self.field+ *steps) % 10;
        self.score += (self.field +1) as u64;
    }

    fn get_score(&self) -> u64 {
        self.score
    }
}

struct DeterministicDice{
    rolled :u32,
    count :u64,
}

impl DeterministicDice{
    fn new () -> Self{
        DeterministicDice{
            rolled: 99,
            count: 0
        }
    }
    fn roll(&mut self)-> u32{
        self.rolled =(self.rolled+ 1) %100;
        self.count += 1;
        self.rolled +1
    }

    fn roll_three_times(&mut self) ->u32{
        let steps = self.roll()+self.roll()+self.roll();
        steps
    }
}

#[cfg(test)]
mod test{
    use crate::day21::{DeterministicDice, game, game_quantum_dice, Player};

    #[test]
    fn test21(){
        let mut player = Player::new(4);
        let mut die = DeterministicDice::new();
        player.move_steps(&die.roll_three_times());
        assert_eq!(player.get_score(), 10);
        let result = game(Player::new(4), Player::new(8), DeterministicDice::new());
        assert_eq!(result, 739785);
        //let res_q = game_quantum_dice(Player::new(4), Player::new(8));
        //assert_eq!(res_q, 444356092776315);
    }


}