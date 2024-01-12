use clap::{Arg, Command, ArgAction, ArgMatches};
pub fn get_match_result() -> ArgMatches {
  let name_args = Arg::new("name")
  .short('n')
  .long("name")
  .action(ArgAction::Set)
  .help("set the name of todo for the command or subcommand");

  let finish_args = Arg::new("finished")
  .short('f')
  .long("finished")
  .action(ArgAction::SetTrue)
  .help("shows todo marked as done");

  let ongoing_args = Arg::new("on-going")
  .short('o')
  .long("on-going")
  .action(ArgAction::SetTrue)
  .help("shows todo not marked as done");

  let index_args = Arg::new("index")
  .short('i')
  .long("index")
  .action(ArgAction::Set)
  .help("use index of todo instead(starts at index 0)");

  let match_result: ArgMatches = Command::new("Todo CLI")
  .about("Quickly summon or add to your personal todo list.")
  .arg(&finish_args)
  .arg(&ongoing_args) 
  .subcommand(
      Command::new("add") 
      .about("Add to your todo list.")
      .arg(&name_args)
      .arg(&finish_args)
      .arg(&ongoing_args)
  )
  .subcommand(
      Command::new("done")
      .about("sets a todo to done.")
      .arg(&name_args)
      .arg(&index_args)
  )
  .subcommand(
      Command::new("clear")
      .about("clears the todo list or a specific todo if name is provided")
      .arg(&name_args)
      .arg(&index_args)
  )
  .get_matches();
  match_result
}
