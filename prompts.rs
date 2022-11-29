use std::io;

fn prompt() -> bool {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  return input.to_ascii_lowercase().starts_with("y");
}
// first prompt
pub fn intro() -> () {
    println!("");
    println!("Here is the story of a hobbit who lived in his hobbit hole living his normal life.");
    println!("little bit he knew that his life is about to change soon.");
    println!("");
    println!("Someone's knocking at the door, do you answer?");

  // if prompt() returns true, that's enough for rust to carry on!
  // a few other expressions that also amount to true are 2 == 2,
  // 1 > 0, or just the word true.
  if prompt() {
    door_answered();
  } else {
    println!("You are such a borring hobbit. Get back with your lonly life!!!");
  }
}

// second prompt
pub fn door_answered() -> () {
  println!("There is a tall old bearded man standing at the door.  '");
  println!("He asks if you would like to go on an adventure. Do you accept?");

  if prompt() {
    challenge_accepted();
  }
  else {
    println!("You close the door telling him to go bother someone else. You didn't find out what was all that about. Maybe you will. One day...");
    println!("YOU WON! To be continued...");
  }
}

// third prompt
pub fn challenge_accepted() -> () {
  println!("You meet up with all these dwarves and they hand you the contract to sign. While they sing a beautiful song named misty mountains. ");
  println!("You wake up to the clean house with all the dishes are washed and the contract on the floor. You realize this chance cannot be missed. Pack your bags. Because we are going on an adventure.");
  println!("Congratulations! You have finally decided to go on an adventure. dwarves are waiting for you.");
  println!("GAME OVER!");
}